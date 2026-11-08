//! Linear float RGBA raster image surface with filtering and bilinear sampling.

use serde::{Deserialize, Serialize};

/// A linear 32-bit float RGBA image in row-major order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RasterImage {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<[f32; 4]>,
}

impl RasterImage {
    pub fn new(width: u32, height: u32) -> Self {
        let count = (width as usize).saturating_mul(height as usize);
        Self {
            width,
            height,
            pixels: vec![[0.0, 0.0, 0.0, 0.0]; count],
        }
    }

    pub fn filled(width: u32, height: u32, color: [f32; 4]) -> Self {
        let count = (width as usize).saturating_mul(height as usize);
        Self {
            width,
            height,
            pixels: vec![color; count],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0 || self.pixels.is_empty()
    }

    #[inline(always)]
    pub fn pixel(&self, x: u32, y: u32) -> [f32; 4] {
        if x >= self.width || y >= self.height {
            return [0.0, 0.0, 0.0, 0.0];
        }
        let index = (y as usize) * (self.width as usize) + (x as usize);
        self.pixels.get(index).copied().unwrap_or([0.0, 0.0, 0.0, 0.0])
    }

    #[inline(always)]
    pub fn set_pixel(&mut self, x: u32, y: u32, color: [f32; 4]) {
        if x >= self.width || y >= self.height {
            return;
        }
        let index = (y as usize) * (self.width as usize) + (x as usize);
        if let Some(slot) = self.pixels.get_mut(index) {
            *slot = color;
        }
    }

    pub fn sample_nearest(&self, u: f32, v: f32) -> [f32; 4] {
        if self.is_empty() {
            return [0.0, 0.0, 0.0, 0.0];
        }
        let u_wrapped = u.rem_euclid(1.0);
        let v_wrapped = v.rem_euclid(1.0);
        let x = ((u_wrapped * self.width as f32) as u32).min(self.width - 1);
        let y = ((v_wrapped * self.height as f32) as u32).min(self.height - 1);
        self.pixel(x, y)
    }

    pub fn sample_bilinear(&self, u: f32, v: f32) -> [f32; 4] {
        if self.is_empty() {
            return [0.0, 0.0, 0.0, 0.0];
        }
        let u_wrapped = u.rem_euclid(1.0) * self.width as f32 - 0.5;
        let v_wrapped = v.rem_euclid(1.0) * self.height as f32 - 0.5;

        let x0 = u_wrapped.floor() as i32;
        let y0 = v_wrapped.floor() as i32;
        let fx = u_wrapped - x0 as f32;
        let fy = v_wrapped - y0 as f32;

        let wrap_x = |x: i32| (x.rem_euclid(self.width as i32)) as u32;
        let wrap_y = |y: i32| (y.rem_euclid(self.height as i32)) as u32;

        let c00 = self.pixel(wrap_x(x0), wrap_y(y0));
        let c10 = self.pixel(wrap_x(x0 + 1), wrap_y(y0));
        let c01 = self.pixel(wrap_x(x0), wrap_y(y0 + 1));
        let c11 = self.pixel(wrap_x(x0 + 1), wrap_y(y0 + 1));

        let mut out = [0.0f32; 4];
        for i in 0..4 {
            let top = c00[i] * (1.0 - fx) + c10[i] * fx;
            let btm = c01[i] * (1.0 - fx) + c11[i] * fx;
            out[i] = top * (1.0 - fy) + btm * fy;
        }
        out
    }

    /// Fast separable box blur of all channels.
    pub fn box_blur(&self, radius: u32) -> Self {
        if radius == 0 || self.is_empty() {
            return self.clone();
        }
        let r = radius as i32;
        let w = self.width;
        let h = self.height;

        // Horizontal pass
        let mut horiz = Self::new(w, h);
        for y in 0..h {
            for x in 0..w {
                let mut acc = [0.0f32; 4];
                let mut count = 0.0f32;
                for k in -r..=r {
                    let sx = (x as i32 + k).clamp(0, w as i32 - 1) as u32;
                    let p = self.pixel(sx, y);
                    for c in 0..4 {
                        acc[c] += p[c];
                    }
                    count += 1.0;
                }
                horiz.set_pixel(x, y, [acc[0] / count, acc[1] / count, acc[2] / count, acc[3] / count]);
            }
        }

        // Vertical pass
        let mut result = Self::new(w, h);
        for y in 0..h {
            for x in 0..w {
                let mut acc = [0.0f32; 4];
                let mut count = 0.0f32;
                for k in -r..=r {
                    let sy = (y as i32 + k).clamp(0, h as i32 - 1) as u32;
                    let p = horiz.pixel(x, sy);
                    for c in 0..4 {
                        acc[c] += p[c];
                    }
                    count += 1.0;
                }
                result.set_pixel(x, y, [acc[0] / count, acc[1] / count, acc[2] / count, acc[3] / count]);
            }
        }

        result
    }
}
