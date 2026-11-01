//! CPAL lifecycle reuses the controlled local backend and the existing native
//! WAV adapter's exact-format and WASAPI acknowledged-reset policy. No codec/DSP.
use super::queue::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use qualia_core_db::audio::scene_audio_graph::source_coupling::{SoundBinding, SoundBlockReceipt};
use std::{
    sync::{
        atomic::{AtomicBool, AtomicU64, AtomicU8, Ordering},
        mpsc, Arc, Mutex,
    },
    time::Duration,
};

enum Command {
    Play(mpsc::Sender<Result<(), String>>),
    Shutdown,
}
#[repr(u8)]
#[derive(Clone, Copy)]
enum DeviceOwnership { Free = 0, Held = 1, Poisoned = 2 }
static DEVICE_OWNER: AtomicU8 = AtomicU8::new(DeviceOwnership::Free as u8);
struct DeviceSlot<'a> {
    owner: &'a AtomicU8,
    play_attempted: bool,
    reset_ack: Arc<AtomicBool>,
}
impl<'a> DeviceSlot<'a> {
    fn acquire(owner: &'a AtomicU8, reset_ack: Arc<AtomicBool>) -> Result<Self, &'static str> {
        owner.compare_exchange(DeviceOwnership::Free as u8, DeviceOwnership::Held as u8,
            Ordering::AcqRel, Ordering::Acquire).map_err(|state|
                if state == DeviceOwnership::Poisoned as u8 {
                    "PCM device owner is poisoned: prior play attempt has no acknowledged reset; automatic recovery is unavailable"
                } else { "PCM device owner is active or has not acknowledged shutdown" })?;
        Ok(Self { owner, play_attempted: false, reset_ack })
    }
    fn before_play(&mut self) {
        // A failed/panicking play may still have started backend activity. Clear
        // any prior acknowledgement BEFORE attempting the fallible device call.
        self.reset_ack.store(false, Ordering::Release);
        self.play_attempted = true;
    }
}
impl Drop for DeviceSlot<'_> {
    fn drop(&mut self) {
        let next = if !self.play_attempted || self.reset_ack.load(Ordering::Acquire) {
            DeviceOwnership::Free
        } else { DeviceOwnership::Poisoned };
        self.owner.store(next as u8, Ordering::Release);
    }
}
pub struct DeviceOutput {
    queue: Arc<Mutex<Queue>>,
    cancel: Arc<AtomicBool>,
    status: Arc<AtomicU8>,
    consumed: Arc<AtomicU64>,
    sender: mpsc::SyncSender<Command>,
    binding: SoundBinding,
    reset_ack: Arc<AtomicBool>,
}
impl DeviceOutput {
    /// `live` is called on the native device worker, NEVER inside the realtime
    /// callback. The embedding supplies the engine's exact source/session guard.
    pub fn prepare(
        binding: SoundBinding,
        slots: usize,
        block_frames: usize,
        byte_budget: usize,
        cancel: Arc<AtomicBool>,
        live: Arc<dyn Fn() -> bool + Send + Sync>,
    ) -> Result<Self, String> {
        if cancel.load(Ordering::Acquire) || !live() {
            return Err("Audio source/authority unavailable".into());
        }
        let status = Arc::new(AtomicU8::new(PREPARED));
        let consumed = Arc::new(AtomicU64::new(0));
        let reset_ack = Arc::new(AtomicBool::new(false));
        let ack = reset_ack.clone();
        let guard = DeviceSlot::acquire(&DEVICE_OWNER, reset_ack.clone())?;
        let queue = Arc::new(Mutex::new(Queue::new(
            binding,
            slots,
            block_frames,
            byte_budget,
            cancel.clone(),
            status.clone(),
            consumed.clone(),
        )?));
        let (sender, receiver) = mpsc::sync_channel(1);
        let (ready_tx, ready_rx) = mpsc::sync_channel(1);
        let q = queue.clone();
        let cancelled = cancel.clone();
        let state = status.clone();
        std::thread::Builder::new()
            .name("pcm-output-device".into())
            .spawn(move || {
                let mut guard = guard;
                let stream = match prepare_stream(q.clone(), cancelled.clone(), state.clone()) {
                    Ok(stream) => {
                        let _ = ready_tx.send(Ok(()));
                        stream
                    }
                    Err(e) => {
                        cancelled.store(true, Ordering::Release);
                        state.store(FAILED, Ordering::Release);
                        let _ = ready_tx.send(Err(e));
                        return;
                    }
                };
                loop {
                    // A native authority check may conservatively refuse contention.
                    // Polling is not an instantaneous revocation or latency guarantee.
                    if cancelled.load(Ordering::Acquire) || !live() {
                        cancelled.store(true, Ordering::Release);
                        let result = stop_and_reset(&stream);
                        ack.store(result.is_ok(), Ordering::Release);
                        if matches!(state.load(Ordering::Acquire), PREPARED | PLAYING) {
                            state.store(
                                if result.is_ok() { RETIRED } else { FAILED },
                                Ordering::Release,
                            );
                        }
                        break;
                    }
                    match receiver.recv_timeout(Duration::from_millis(10)) {
                        Ok(Command::Play(reply)) => {
                            let result = (|| {
                                if !live() || cancelled.load(Ordering::Acquire) {
                                    return Err("Output lease is no longer current".into());
                                }
                                if !q.try_lock().map_err(|_| "Device queue busy")?.ready() {
                                    return Err("Prefill PCM before starting output".into());
                                }
                                guard.before_play();
                                state.store(PLAYING, Ordering::Release);
                                if let Err(e) = stream.play() {
                                    cancelled.store(true, Ordering::Release);
                                    state.store(FAILED, Ordering::Release);
                                    return Err(format!("Audio start failed: {e}"));
                                }
                                Ok(())
                            })();
                            let _ = reply.send(result);
                        }
                        Ok(Command::Shutdown) | Err(mpsc::RecvTimeoutError::Disconnected) => {
                            cancelled.store(true, Ordering::Release);
                            ack.store(stop_and_reset(&stream).is_ok(), Ordering::Release);
                            break;
                        }
                        Err(mpsc::RecvTimeoutError::Timeout) => {}
                    }
                }
            })
            .map_err(|_| "Audio device worker creation failed")?;
        match ready_rx.recv_timeout(Duration::from_secs(3)) {
            Ok(Ok(())) => Ok(Self {
                queue,
                cancel,
                status,
                consumed,
                sender,
                binding,
                reset_ack,
            }),
            Ok(Err(e)) => Err(e),
            Err(_) => {
                cancel.store(true, Ordering::Release);
                Err("Audio preparation not acknowledged; output lease retired".into())
            }
        }
    }
    pub fn enqueue(&self, receipt: &SoundBlockReceipt, l: &[f32], r: &[f32]) -> Result<(), String> {
        self.queue
            .try_lock()
            .map_err(|_| "Device queue busy; no block accepted")?
            .enqueue(receipt, l, r)
    }
    fn request(
        &self,
        make: impl FnOnce(mpsc::Sender<Result<(), String>>) -> Command,
    ) -> Result<(), String> {
        let (tx, rx) = mpsc::channel();
        self.sender
            .try_send(make(tx))
            .map_err(|_| "Device command unavailable")?;
        match rx.recv_timeout(Duration::from_secs(3)) {
            Ok(result) => result,
            Err(_) => {
                self.cancel();
                Err("Device command not acknowledged; lease retired and presentation state uncertain".into())
            }
        }
    }
    pub fn play(&self) -> Result<(), String> {
        self.request(Command::Play)
    }
    pub fn stop(&self) -> Result<(), String> {
        self.cancel();
        let deadline = std::time::Instant::now() + Duration::from_secs(3);
        loop {
            if self.reset_ack.load(Ordering::Acquire) {
                return Ok(());
            }
            if std::time::Instant::now() >= deadline {
                return Err("Output lease retired; device reset not acknowledged".into());
            }
            std::thread::sleep(Duration::from_millis(5));
        }
    }
    pub fn binding(&self) -> SoundBinding {
        self.binding
    }
    pub fn cancel(&self) {
        self.cancel.store(true, Ordering::Release);
    }
    pub fn consumed_frames(&self) -> u64 {
        self.consumed.load(Ordering::Acquire)
    }
    pub fn status(&self) -> &'static str {
        match self.status.load(Ordering::Acquire) {
            PLAYING => "PCM callbacks active; acoustic presentation unverified",
            RETIRED => "stream stopped and reset acknowledged",
            FAILED => "device failed or reset unacknowledged",
            STARVED => "output starved; epoch retired",
            _ => "prepared; PCM output not started",
        }
    }
}
impl Drop for DeviceOutput {
    fn drop(&mut self) {
        self.cancel();
        let _ = self.sender.try_send(Command::Shutdown);
    }
}
fn prepare_stream(
    queue: Arc<Mutex<Queue>>,
    cancel: Arc<AtomicBool>,
    status: Arc<AtomicU8>,
) -> Result<cpal::Stream, String> {
    let device = cpal::default_host()
        .default_output_device()
        .ok_or("No default audio output")?;
    let supported = device
        .supported_output_configs()
        .map_err(|_| "Audio format enumeration failed")?
        .find(|c| {
            c.channels() == 2
                && c.sample_format() == cpal::SampleFormat::F32
                && c.min_sample_rate().0 <= 48_000
                && c.max_sample_rate().0 >= 48_000
        })
        .ok_or(
            "Exact 48 kHz stereo f32 device format unavailable; no conversion was substituted",
        )?;
    let config: cpal::StreamConfig = supported.with_sample_rate(cpal::SampleRate(48_000)).into();
    let error_cancel = cancel.clone();
    let error_status = status.clone();
    device
        .build_output_stream(
            &config,
            move |out: &mut [f32], _| {
                if cancel.load(Ordering::Acquire) {
                    out.fill(0.);
                    return;
                }
                if let Ok(mut q) = queue.try_lock() {
                    q.render(out)
                } else {
                    out.fill(0.);
                    cancel.store(true, Ordering::Release);
                    status.store(STARVED, Ordering::Release);
                }
            },
            move |_| {
                error_cancel.store(true, Ordering::Release);
                error_status.store(FAILED, Ordering::Release);
            },
            None,
        )
        .map_err(|_| "Audio stream preparation failed".into())
}
#[cfg(windows)]
fn stop_and_reset(stream: &cpal::Stream) -> Result<(), String> {
    match stream.as_inner() {
        cpal::platform::StreamInner::Wasapi(s) => s.stop_and_reset(Duration::from_secs(1)),
        #[allow(unreachable_patterns)]
        _ => Err("No acknowledged reset for selected device backend".into()),
    }
}
#[cfg(not(windows))]
fn stop_and_reset(_stream: &cpal::Stream) -> Result<(), String> {
    Err("No acknowledged reset for selected device backend".into())
}

#[cfg(test)]
#[path = "device_slot_tests.rs"]
mod device_slot_tests;
