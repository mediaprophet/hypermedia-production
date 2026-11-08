//! Paint scopes grids into RGBA8 pixel buffers for previews and HUD rendering.

use crate::matrix::{Matrix, ycbcr_to_rgb};
use crate::scopes::{Grid, Vectorscope};

#[inline]
pub fn intensity(count: u32, reference: f32, gain: f32) -> f32 {
    if count == 0 {
        return 0.0;
    }
    let r = (1.0 + reference.max(1.0)).ln();
    (gain * (0.12 + 0.88 * (1.0 + count as f32).ln() / r)).clamp(0.0, 1.0)
}

pub fn grids(layers: &[(&Grid, [f32; 3])], reference: f32, gain: f32) -> (usize, usize, Vec<u8>) {
    let Some((first, _)) = layers.first() else {
        return (0, 0, Vec::new());
    };
    let (w, h) = (first.cols, first.rows);
    let mut out = vec![0u8; w * h * 4];
    let mut acc = vec![[0f32; 3]; w * h];
    for (g, col) in layers {
        if g.cols != w || g.rows != h {
            continue;
        }
        for row in 0..h {
            let dst = (h - 1 - row) * w;
            for x in 0..w {
                let c = g.data[row * w + x];
                if c > 0 {
                    let i = intensity(c, reference, gain);
                    let a = &mut acc[dst + x];
                    for k in 0..3 {
                        a[k] += col[k] * i;
                    }
                }
            }
        }
    }
    for (idx, a) in acc.iter().enumerate() {
        let m = a[0].max(a[1]).max(a[2]);
        if m <= 0.0 {
            continue;
        }
        let base = idx * 4;
        out[base] = (a[0].min(1.0) * 255.0).round() as u8;
        out[base + 1] = (a[1].min(1.0) * 255.0).round() as u8;
        out[base + 2] = (a[2].min(1.0) * 255.0).round() as u8;
        out[base + 3] = (m.min(1.0) * 255.0).round() as u8;
    }
    (w, h, out)
}

pub fn waveform_reference(per_column: usize) -> f32 {
    (per_column as f32 / 2.0).max(1.0)
}

pub fn vectorscope(
    v: &Vectorscope,
    col: [f32; 3],
    gain: f32,
    colorize: bool,
) -> (usize, usize, Vec<u8>) {
    let n = v.size;
    let reference = (v.samples as f32 / 64.0).max(1.0);
    let mut level = vec![0f32; n * n];
    let mut tint = vec![[0f32; 3]; n * n];
    for y in 0..n {
        for x in 0..n {
            let c = v.at(x, y);
            if c == 0 {
                continue;
            }
            let i = intensity(c, reference, gain);
            let t = if colorize {
                let [cb, cr] = v.point(x, y);
                let [r, g, b] = ycbcr_to_rgb(0.6, cb * 0.8, cr * 0.8, Matrix::Bt709);
                [
                    0.55 + 0.45 * r.clamp(0.0, 1.0),
                    0.55 + 0.45 * g.clamp(0.0, 1.0),
                    0.55 + 0.45 * b.clamp(0.0, 1.0),
                ]
            } else {
                col
            };
            for (dy, dx, w) in [
                (0i64, 0i64, 1.0f32),
                (-1, 0, 0.6),
                (1, 0, 0.6),
                (0, -1, 0.6),
                (0, 1, 0.6),
                (-1, -1, 0.35),
                (-1, 1, 0.35),
                (1, -1, 0.35),
                (1, 1, 0.35),
            ] {
                let (yy, xx) = (y as i64 + dy, x as i64 + dx);
                if yy < 0 || xx < 0 || yy >= n as i64 || xx >= n as i64 {
                    continue;
                }
                let k = yy as usize * n + xx as usize;
                if i * w > level[k] {
                    level[k] = i * w;
                    tint[k] = t;
                }
            }
        }
    }
    let mut out = vec![0u8; n * n * 4];
    for (idx, &lev) in level.iter().enumerate() {
        if lev <= 0.0 {
            continue;
        }
        let t = tint[idx];
        let base = idx * 4;
        out[base] = (t[0] * lev.min(1.0) * 255.0).round() as u8;
        out[base + 1] = (t[1] * lev.min(1.0) * 255.0).round() as u8;
        out[base + 2] = (t[2] * lev.min(1.0) * 255.0).round() as u8;
        out[base + 3] = (lev.min(1.0) * 255.0).round() as u8;
    }
    (n, n, out)
}
