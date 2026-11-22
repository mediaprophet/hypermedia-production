//! Stable indexed random draws for replayable primal checkpoints.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

pub(crate) const COUNTER_ALGORITHM: &str = "splitmix64-counter-v1";

/// An open-interval uniform draw addressed by one seed, stream and counter.
pub(crate) fn counter_uniform(seed: u64, stream: u64, counter: u64) -> f64 {
    let key = mix(seed ^ mix(stream));
    let draw = mix(key.wrapping_add(counter.wrapping_mul(0x9e3779b97f4a7c15)));
    ((draw >> 12) as f64 + 0.5) / (1_u64 << 52) as f64
}

fn mix(value: u64) -> u64 {
    let value = (value ^ (value >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    let value = (value ^ (value >> 27)).wrapping_mul(0x94d049bb133111eb);
    value ^ (value >> 31)
}
