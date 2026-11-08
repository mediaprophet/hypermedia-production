//! Unit tests for video scopes mathematics.

use crate::matrix::{Matrix, rgb_to_ycbcr};
use crate::scopes::*;

fn rgba8(w: usize, h: usize, mut f: impl FnMut(usize, usize) -> [u8; 3]) -> Vec<u8> {
    let mut px = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let [r, g, b] = f(x, y);
            px.extend_from_slice(&[r, g, b, 255]);
        }
    }
    px
}

fn solid(w: usize, h: usize, c: [u8; 3]) -> Signal {
    Signal::from_rgba8(w, h, &rgba8(w, h, |_, _| c))
}

const BAR: u8 = 191;
const BARS: [[u8; 3]; 8] = [
    [BAR, BAR, BAR],
    [BAR, BAR, 0],
    [0, BAR, BAR],
    [0, BAR, 0],
    [BAR, 0, BAR],
    [BAR, 0, 0],
    [0, 0, BAR],
    [0, 0, 0],
];

fn bars() -> Signal {
    Signal::from_rgba8(256, 16, &rgba8(256, 16, |x, _| BARS[x / 32]))
}

fn code(v: u8) -> f32 {
    v as f32 / 255.0
}

#[test]
fn single_colour_lands_in_exact_histogram_bins() {
    let s = solid(64, 36, [200, 100, 50]);
    let n = (64 * 36) as u32;
    let h = histogram(&s, &Params::default());
    assert_eq!(h.samples, n);
    assert_eq!((h.r[200], h.g[100], h.b[50]), (n, n, n));
    assert_eq!(h.r.iter().sum::<u32>(), n);
    let y = rgb_to_ycbcr(code(200), code(100), code(50), Matrix::Bt709)[0];
    assert_eq!(h.y[(y * 255.0).round() as usize], n);
    assert_eq!((h.below, h.above), ([0; 4], [0; 4]));
}

#[test]
fn ramp_fills_every_bin_and_waveform_row_equals_code_value() {
    let s = Signal::from_rgba8(256, 10, &rgba8(256, 10, |x, _| [x as u8; 3]));
    let p = Params::default();
    let h = histogram(&s, &p);
    for k in 0..256 {
        assert_eq!((h.r[k], h.g[k], h.b[k], h.y[k]), (10, 10, 10, 10), "bin {k}");
    }
    for kind in [WaveformType::Luma, WaveformType::YcNoChroma] {
        let w = waveform(&s, kind, &p);
        let y = w.trace("Y").unwrap();
        for x in 0..256 {
            assert_eq!(y.rows_in(x), vec![x], "{kind:?} column {x}");
            assert_eq!(y.at(x, x), 10);
        }
    }
    let w = waveform(&s, WaveformType::Rgb, &p);
    assert_eq!(w.traces.len(), 3);
    for t in &w.traces {
        assert!((0..256).all(|x| t.rows_in(x) == vec![x]), "{}", t.name);
    }
}

#[test]
fn colour_bars_parade_levels_and_vectorscope_targets() {
    let s = bars();
    let p = Params::default();
    let par = parade(&s, ParadeType::Rgb, &p);
    for (ch, name) in ["R", "G", "B"].iter().enumerate() {
        let t = par.trace(name).unwrap();
        for (i, bar) in BARS.iter().enumerate() {
            for x in i * 32..(i + 1) * 32 {
                assert_eq!(t.rows_in(x), vec![bar[ch] as usize], "{name} bar {i}");
                assert_eq!(t.at(x, bar[ch] as usize), 16);
            }
        }
    }
    assert_eq!(
        parade(&s, ParadeType::RgbWhite, &p)
            .traces
            .iter()
            .map(|t| t.name)
            .collect::<Vec<_>>(),
        ["R", "G", "B", "Y"]
    );
    assert_eq!(
        parade(&s, ParadeType::Yuv, &p)
            .traces
            .iter()
            .map(|t| t.name)
            .collect::<Vec<_>>(),
        ["Y", "Cb", "Cr"]
    );

    for m in [Matrix::Bt601, Matrix::Bt709, Matrix::Bt2020Ncl] {
        let p = Params {
            matrix: m,
            ..Params::default()
        };
        let v = vectorscope_yuv(&s, &p);
        assert_eq!(v.samples, 256 * 16);
        let bar_samples = 32 * 16;
        for t in targets(m, code(BAR)) {
            let (x, y) = v.cell(t.cb, t.cr).unwrap();
            assert_eq!(v.at(x, y), bar_samples, "{m:?} {}", t.name);
        }
        let (cx, cy) = v.cell(0.0, 0.0).unwrap();
        assert_eq!(v.at(cx, cy), 2 * bar_samples);
    }
}
