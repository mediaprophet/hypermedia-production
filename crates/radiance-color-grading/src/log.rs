//! Camera log curves implemented from manufacturers' published specifications.
//!
//! Grounded in Sony S-Log3, Panasonic V-Log, Canon Log 2/3, ARRI LogC3/LogC4, Apple Log,
//! DJI D-Log, and Academy ACEScc (S-2014-003) / ACEScct (S-2016-001).

use serde::{Deserialize, Serialize};

use crate::matrix::Range;

/// Normalisation used by a published curve formula.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Domain {
    /// 10-bit code value / 1023.
    CodeValue,
    /// Video-range normalised (code 64 -> 0, code 940 -> 1).
    Ire,
}

/// A camera log or logarithmic grading encoding.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LogCurve {
    SLog3,
    VLog,
    CLog2,
    CLog3,
    LogC3,
    LogC4,
    AppleLog,
    DLog,
    AcesCc,
    AcesCct,
}

impl LogCurve {
    pub const ALL: [LogCurve; 10] = [
        LogCurve::SLog3,
        LogCurve::VLog,
        LogCurve::CLog2,
        LogCurve::CLog3,
        LogCurve::LogC3,
        LogCurve::LogC4,
        LogCurve::AppleLog,
        LogCurve::DLog,
        LogCurve::AcesCc,
        LogCurve::AcesCct,
    ];

    pub fn domain(self) -> Domain {
        match self {
            LogCurve::CLog2 | LogCurve::CLog3 | LogCurve::AppleLog => Domain::Ire,
            _ => Domain::CodeValue,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            LogCurve::SLog3 => "Sony S-Log3",
            LogCurve::VLog => "Panasonic V-Log",
            LogCurve::CLog2 => "Canon Log 2",
            LogCurve::CLog3 => "Canon Log 3",
            LogCurve::LogC3 => "ARRI LogC3",
            LogCurve::LogC4 => "ARRI LogC4",
            LogCurve::AppleLog => "Apple Log",
            LogCurve::DLog => "DJI D-Log",
            LogCurve::AcesCc => "ACEScc",
            LogCurve::AcesCct => "ACEScct",
        }
    }

    /// Linear scene reflectance -> log signal.
    pub fn encode(self, x: f64) -> f64 {
        match self {
            LogCurve::SLog3 => slog3_encode(x),
            LogCurve::VLog => vlog_encode(x),
            LogCurve::CLog2 => clog2_encode(x),
            LogCurve::CLog3 => clog3_encode(x),
            LogCurve::LogC3 => logc3_encode(x),
            LogCurve::LogC4 => logc4_encode(x),
            LogCurve::AppleLog => apple_log_encode(x),
            LogCurve::DLog => dlog_encode(x),
            LogCurve::AcesCc => aces_cc_encode(x),
            LogCurve::AcesCct => aces_cct_encode(x),
        }
    }

    /// Log signal -> linear scene reflectance.
    pub fn decode(self, y: f64) -> f64 {
        match self {
            LogCurve::SLog3 => slog3_decode(y),
            LogCurve::VLog => vlog_decode(y),
            LogCurve::CLog2 => clog2_decode(y),
            LogCurve::CLog3 => clog3_decode(y),
            LogCurve::LogC3 => logc3_decode(y),
            LogCurve::LogC4 => logc4_decode(y),
            LogCurve::AppleLog => apple_log_decode(y),
            LogCurve::DLog => dlog_decode(y),
            LogCurve::AcesCc => aces_cc_decode(y),
            LogCurve::AcesCct => aces_cct_decode(y),
        }
    }

    pub fn signal_from_normalized(self, n: f32, range: Range) -> f32 {
        match (self.domain(), range) {
            (Domain::CodeValue, Range::Limited) => (n * (235.0 - 16.0) + 16.0) / 255.0,
            (Domain::CodeValue, Range::Full) => n,
            (Domain::Ire, Range::Limited) => n,
            (Domain::Ire, Range::Full) => (n * 255.0 - 16.0) / (235.0 - 16.0),
        }
    }

    pub fn normalized_from_signal(self, s: f32, range: Range) -> f32 {
        match (self.domain(), range) {
            (Domain::CodeValue, Range::Limited) => (s * 255.0 - 16.0) / (235.0 - 16.0),
            (Domain::CodeValue, Range::Full) => s,
            (Domain::Ire, Range::Limited) => s,
            (Domain::Ire, Range::Full) => (s * (235.0 - 16.0) + 16.0) / 255.0,
        }
    }
}

// --- Sony S-Log3 ---
fn slog3_encode(x: f64) -> f64 {
    if x >= 0.01125000 {
        (420.0 + (x + 0.01) / (0.18 + 0.01) * 261.5).log10() / 1023.0
    } else {
        (x * (171.2102946929 - 95.0) / 0.01125000 + 95.0) / 1023.0
    }
}

fn slog3_decode(y: f64) -> f64 {
    let cv = y * 1023.0;
    if cv >= 171.2102946929 {
        10f64.powf((cv - 420.0) / 261.5) * (0.18 + 0.01) - 0.01
    } else {
        (cv - 95.0) * 0.01125000 / (171.2102946929 - 95.0)
    }
}

// --- Panasonic V-Log ---
const V_CUT: f64 = 0.01;
const V_B: f64 = 0.00873;
const V_C: f64 = 0.241514;
const V_D: f64 = 0.598206;

fn vlog_encode(x: f64) -> f64 {
    if x < V_CUT {
        5.6 * x + 0.125
    } else {
        V_C * (x + V_B).ln() + V_D
    }
}

fn vlog_decode(y: f64) -> f64 {
    if y < 5.6 * V_CUT + 0.125 {
        (y - 0.125) / 5.6
    } else {
        ((y - V_D) / V_C).exp() - V_B
    }
}

// --- Canon Log 2 ---
fn clog2_encode(x: f64) -> f64 {
    if x <= 0.0 {
        0.03537714
    } else if x < 0.02705907 {
        0.03537714 + 1.2584149 * x
    } else {
        0.07921976 + 0.2818631 * (x + 0.01).log10()
    }
}

fn clog2_decode(y: f64) -> f64 {
    if y < 0.06942767 {
        (y - 0.03537714) / 1.2584149
    } else {
        10f64.powf((y - 0.07921976) / 0.2818631) - 0.01
    }
}

// --- Canon Log 3 ---
fn clog3_encode(x: f64) -> f64 {
    if x <= 0.0 {
        0.07305936
    } else if x < 0.03988647 {
        0.07305936 + 2.3069815 * x
    } else {
        0.1659632 + 0.23069815 * (x + 0.01).log10()
    }
}

fn clog3_decode(y: f64) -> f64 {
    if y < 0.16507693 {
        (y - 0.07305936) / 2.3069815
    } else {
        10f64.powf((y - 0.1659632) / 0.23069815) - 0.01
    }
}

// --- ARRI LogC3 (EI 800) ---
const C3_CUT: f64 = 0.010591;
const C3_A: f64 = (200.0 * 0.18) / 0.9;
const C3_B: f64 = (1.0 - 0.075) / 0.9;
const C3_C: f64 = 0.075;
const C3_D: f64 = 0.092809;
const C3_E: f64 = 5.555556;
const C3_F: f64 = 0.085467;

fn logc3_encode(x: f64) -> f64 {
    if x > C3_CUT {
        C3_C * (C3_A * x + C3_B).log10() + C3_D
    } else {
        C3_E * x + C3_F
    }
}

fn logc3_decode(y: f64) -> f64 {
    if y > C3_E * C3_CUT + C3_F {
        (10f64.powf((y - C3_D) / C3_C) - C3_B) / C3_A
    } else {
        (y - C3_F) / C3_E
    }
}

// --- ARRI LogC4 ---
const C4_A: f64 = (262144.0 - 16.0) / 117.45;
const C4_B: f64 = (1023.0 - 95.0) / 1023.0;
const C4_C: f64 = 95.0 / 1023.0;
const C4_S: f64 = (7.0 * std::f64::consts::LN_2) / (16384.0 * (262144.0 - 16.0));
const C4_T: f64 = (16384.0 * (-14.0 * std::f64::consts::LN_2 + 64.0)) / (262144.0 - 16.0);

fn logc4_encode(x: f64) -> f64 {
    if x >= -0.0019238383 {
        ((C4_A * x + 64.0).log2() - 6.0) / 14.0 * C4_B + C4_C
    } else {
        (C4_S * x + C4_T) * C4_B + C4_C
    }
}

fn logc4_decode(y: f64) -> f64 {
    let p = (y - C4_C) / C4_B;
    if p >= 0.0 {
        (2f64.powf(p * 14.0 + 6.0) - 64.0) / C4_A
    } else {
        (p - C4_T) / C4_S
    }
}

// --- Apple Log ---
const APPLE_R0: f64 = -0.0075189287;
const APPLE_RT: f64 = 0.0088283285;
const APPLE_C: f64 = 47.28711236;
const APPLE_BETA: f64 = 0.00964052;
const APPLE_GAMMA: f64 = 0.08550479;
const APPLE_DELTA: f64 = 0.69336945;

fn apple_log_encode(r: f64) -> f64 {
    if r < APPLE_R0 {
        0.0
    } else if r <= APPLE_RT {
        APPLE_C * (r - APPLE_R0).powi(2)
    } else {
        APPLE_GAMMA * (r + APPLE_BETA).log2() + APPLE_DELTA
    }
}

fn apple_log_decode(p: f64) -> f64 {
    let pt = APPLE_C * (APPLE_RT - APPLE_R0).powi(2);
    if p <= 0.0 {
        APPLE_R0
    } else if p <= pt {
        (p / APPLE_C).sqrt() + APPLE_R0
    } else {
        2f64.powf((p - APPLE_DELTA) / APPLE_GAMMA) - APPLE_BETA
    }
}

// --- DJI D-Log ---
fn dlog_encode(x: f64) -> f64 {
    if x <= 0.0039805 {
        6.025 * x + 0.0929
    } else {
        (x * 0.9892 + 0.0108).log10() * 0.256663 + 0.584555
    }
}

fn dlog_decode(y: f64) -> f64 {
    if y <= 0.0039805 * 6.025 + 0.0929 {
        (y - 0.0929) / 6.025
    } else {
        (10f64.powf((y - 0.584555) / 0.256663) - 0.0108) / 0.9892
    }
}

// --- ACEScc (Academy S-2014-003) ---
const ACESCC_MIN: f64 = (-16.0 + 9.72) / 17.52;
const ACESCC_TOE: f64 = (-15.0 + 9.72) / 17.52;

fn aces_cc_encode(v: f64) -> f64 {
    if v <= 0.0 {
        ACESCC_MIN
    } else if v < 2f64.powi(-15) {
        ((2f64.powi(-16) + v * 0.5).log2() + 9.72) / 17.52
    } else {
        (v.log2() + 9.72) / 17.52
    }
}

fn aces_cc_decode(v: f64) -> f64 {
    if v < ACESCC_TOE {
        (2f64.powf(v * 17.52 - 9.72) - 2f64.powi(-16)) * 2.0
    } else if v < (65504f64.log2() + 9.72) / 17.52 {
        2f64.powf(v * 17.52 - 9.72)
    } else {
        65504.0
    }
}

// --- ACEScct (Academy S-2016-001) ---
fn aces_cct_encode(v: f64) -> f64 {
    if v <= 0.0078125 {
        10.5402377416545 * v + 0.0729055341958355
    } else {
        (v.log2() + 9.72) / 17.52
    }
}

fn aces_cct_decode(v: f64) -> f64 {
    if v <= 0.155251141552511 {
        (v - 0.0729055341958355) / 10.5402377416545
    } else if v < (65504f64.log2() + 9.72) / 17.52 {
        2f64.powf(v * 17.52 - 9.72)
    } else {
        65504.0
    }
}
