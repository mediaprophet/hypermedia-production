//! Fast MSB-first bit reader (64-bit cache) and writer for macroblock data.

use crate::dnx::tables::{LINK, Lut};

/// Bit reader over one macroblock scan line.
pub struct Reader<'a> {
    data: &'a [u8],
    next: usize,
    cache: u64,
    avail: u32,
}

impl<'a> Reader<'a> {
    #[inline]
    pub fn new(data: &'a [u8]) -> Self {
        let mut r = Self {
            data,
            next: 0,
            cache: 0,
            avail: 0,
        };
        r.refill();
        r
    }

    #[inline(always)]
    fn refill(&mut self) {
        if self.avail >= 56 {
            return;
        }
        if let Some(s) = self.data.get(self.next..self.next + 8) {
            let w = u64::from_be_bytes([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]]);
            self.cache |= w >> self.avail;
            self.next += ((63 - self.avail) >> 3) as usize;
            self.avail |= 56;
        } else {
            while self.avail <= 56 {
                let b = self.data.get(self.next).copied().unwrap_or(0) as u64;
                self.cache |= b << (56 - self.avail);
                self.avail += 8;
                self.next += 1;
            }
        }
    }

    #[inline(always)]
    fn consume(&mut self, n: u32) {
        self.cache <<= n;
        self.avail -= n;
    }

    #[inline]
    pub fn consumed(&self) -> usize {
        self.next * 8 - self.avail as usize
    }

    #[inline]
    pub fn overrun(&self) -> bool {
        self.consumed() > self.data.len() * 8
    }

    #[inline(always)]
    pub fn read(&mut self, n: u32) -> u32 {
        if n == 0 {
            return 0;
        }
        self.refill();
        let v = (self.cache >> (64 - n)) as u32;
        self.consume(n);
        v
    }

    #[inline(always)]
    pub fn vlc(&mut self, t: &Lut) -> u32 {
        self.refill();
        let w = self.cache;
        let mut e = t.primary[(w >> (64 - t.bits)) as usize];
        if e & LINK != 0 {
            let sb = (e >> 24) & 0x7f;
            let off = (e & 0xff_ffff) as usize;
            e = t.secondary[off + ((w << t.bits) >> (64 - sb)) as usize];
        }
        self.consume(e >> 16);
        e & 0xffff
    }
}

/// MSB-first bit writer.
#[derive(Default)]
pub struct Writer {
    pub buf: Vec<u8>,
    acc: u64,
    n: u32,
}

impl Writer {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn write(&mut self, bits: u32, val: u64) {
        if bits == 0 {
            return;
        }
        let val = val & ((1u64 << bits) - 1);
        self.acc = (self.acc << bits) | val;
        self.n += bits;
        while self.n >= 8 {
            self.n -= 8;
            self.buf.push((self.acc >> self.n) as u8);
        }
    }

    #[inline]
    pub fn flush(&mut self) {
        if self.n > 0 {
            self.buf.push((self.acc << (8 - self.n)) as u8);
            self.acc = 0;
            self.n = 0;
        }
    }
}
