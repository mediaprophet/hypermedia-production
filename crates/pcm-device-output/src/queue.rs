use qualia_core_db::audio::scene_audio_graph::source_coupling::{SoundBinding, SoundBlockReceipt};
use std::sync::{
    atomic::{AtomicBool, AtomicU64, AtomicU8, Ordering},
    Arc,
};
#[cfg(test)]
#[path = "queue_tests.rs"]
mod tests;
pub(super) const PREPARED: u8 = 0;
pub(super) const PLAYING: u8 = 1;
pub(super) const RETIRED: u8 = 2;
pub(super) const FAILED: u8 = 3;
pub(super) const STARVED: u8 = 4;
struct Slot {
    samples: Box<[f32]>,
    frames: usize,
}
pub(super) struct Queue {
    slots: Vec<Slot>,
    read: usize,
    write: usize,
    count: usize,
    offset: usize,
    next_frame: Option<u64>,
    binding: SoundBinding,
    pub cancel: Arc<AtomicBool>,
    pub status: Arc<AtomicU8>,
    pub consumed: Arc<AtomicU64>,
}
impl Queue {
    pub fn new(
        binding: SoundBinding,
        slots: usize,
        block: usize,
        bytes: usize,
        cancel: Arc<AtomicBool>,
        status: Arc<AtomicU8>,
        consumed: Arc<AtomicU64>,
    ) -> Result<Self, String> {
        binding.validate().map_err(|_| "Invalid sound lease")?;
        if !(2..=32).contains(&slots) || !(1..=8192).contains(&block) {
            return Err("Device queue profile exceeded".into());
        }
        let samples = block.checked_mul(2).ok_or("PCM size overflow")?;
        let required = samples
            .checked_mul(slots)
            .and_then(|v| v.checked_mul(4))
            .and_then(|v| v.checked_add(slots * std::mem::size_of::<Slot>()))
            .ok_or("PCM queue size overflow")?;
        if required > bytes {
            return Err("Device queue exceeds caller byte budget".into());
        }
        let mut storage = Vec::new();
        storage
            .try_reserve_exact(slots)
            .map_err(|_| "Device queue allocation refused")?;
        for _ in 0..slots {
            let mut data = Vec::new();
            data.try_reserve_exact(samples)
                .map_err(|_| "PCM slot allocation refused")?;
            data.resize(samples, 0.);
            storage.push(Slot {
                samples: data.into_boxed_slice(),
                frames: 0,
            });
        }
        Ok(Self {
            slots: storage,
            read: 0,
            write: 0,
            count: 0,
            offset: 0,
            next_frame: None,
            binding,
            cancel,
            status,
            consumed,
        })
    }
    pub fn enqueue(
        &mut self,
        receipt: &SoundBlockReceipt,
        l: &[f32],
        r: &[f32],
    ) -> Result<(), String> {
        if self.cancel.load(Ordering::Acquire) {
            return Err("Output lease retired".into());
        }
        if receipt.binding != self.binding
            || receipt.output_frames != l.len()
            || l.len() != r.len()
            || l.is_empty()
            || l.len() > self.slots[self.write].samples.len() / 2
            || receipt.first_output_frame != self.next_frame.unwrap_or(0)
        {
            return Err("PCM receipt, frame range or lease differs".into());
        }
        let end = receipt
            .first_output_frame
            .checked_add(u64::try_from(l.len()).map_err(|_| "PCM range overflow")?)
            .ok_or("PCM range overflow")?;
        if self.count == self.slots.len() {
            return Err("Device queue full; no block accepted".into());
        }
        if l.iter().chain(r).any(|v| !(-1.0..=1.0).contains(v)) {
            return Err("Invalid PCM sample".into());
        }
        let slot = &mut self.slots[self.write];
        for (i, (&left, &right)) in l.iter().zip(r).enumerate() {
            slot.samples[2 * i] = left;
            slot.samples[2 * i + 1] = right;
        }
        if self.cancel.load(Ordering::Acquire) {
            return Err("Output lease retired during preparation".into());
        }
        slot.frames = l.len();
        self.write = (self.write + 1) % self.slots.len();
        self.count += 1;
        self.next_frame = Some(end);
        Ok(())
    }
    pub fn ready(&self) -> bool {
        self.count > 0 && !self.cancel.load(Ordering::Acquire)
    }
    /// All slot storage is retained: no allocation/deallocation or source lookup
    /// in the callback. Starvation retires the epoch instead of time-shifting it.
    pub fn render(&mut self, out: &mut [f32]) {
        out.fill(0.);
        if self.cancel.load(Ordering::Acquire) || self.status.load(Ordering::Acquire) != PLAYING {
            return;
        }
        if out.len() % 2 != 0 {
            self.fail(FAILED);
            return;
        }
        let mut written = 0usize;
        while written < out.len() / 2 {
            if self.count == 0 {
                self.fail(STARVED);
                out.fill(0.);
                return;
            }
            let slot = &self.slots[self.read];
            let n = (slot.frames - self.offset).min(out.len() / 2 - written);
            out[2 * written..2 * (written + n)]
                .copy_from_slice(&slot.samples[2 * self.offset..2 * (self.offset + n)]);
            self.offset += n;
            written += n;
            if self.offset == slot.frames {
                self.read = (self.read + 1) % self.slots.len();
                self.count -= 1;
                self.offset = 0;
            }
        }
        if self.cancel.load(Ordering::Acquire) {
            out.fill(0.);
            return;
        }
        let frames = (out.len() / 2) as u64;
        if self
            .consumed
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |v| {
                v.checked_add(frames)
            })
            .is_err()
        {
            self.fail(FAILED);
            out.fill(0.);
        }
    }
    fn fail(&self, status: u8) {
        self.cancel.store(true, Ordering::Release);
        self.status.store(status, Ordering::Release);
    }
}
