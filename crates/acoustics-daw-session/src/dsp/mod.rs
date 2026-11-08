//! Core acoustics digital signal processing (DSP) primitives and studio plugins.

pub mod biquad;
pub mod meter;
pub mod osc;
pub mod pan;
pub mod pitch_detect;
pub mod plugins;

pub use biquad::{BiquadCoeffs, BiquadFilter, FilterType};
pub use meter::{EbuR128Meter, KWeightingChannel, TruePeakMeter};
pub use osc::{Oscillator, Waveform};
pub use pan::{LAYOUT_5_1, LAYOUT_7_1_4, PanLaw, SpeakerPosition, VbapPanner, stereo_pan_gains};
pub use pitch_detect::{DetectedNote, detect_pitch_yin, hz_to_midi};
pub use plugins::{
    ChannelStrip, Compressor, DeEsser, Eq7, ExpanderGate, FdnReverb, Maximizer, SubtractiveSynth,
};
