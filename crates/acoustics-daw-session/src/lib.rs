//! Acoustics DSP, surround VBAP, ITU-R BS.1770 / EBU R128 LUFS metering, YIN pitch detection,
//! Flex Time elastic warp, Camelot music intelligence, AIFF/FLAC/WAV codecs, and PDC DAW mix engine.
//!
//! Grounded in international human rights instruments (UDHR Articles 19, 27; ICESCR Article 15)
//! and Informatics Fiduciary duties ensuring authentic acoustic fidelity and creative freedom.

#![forbid(unsafe_code)]

pub mod codecs;
pub mod dsp;
pub mod elastic_warp;
pub mod mix_session;
pub mod music_intelligence;

pub use codecs::{
    AudioBuffer, PEAK_BLOCK_SIZE, PEAK_FANOUT, PeakFile, decode_wav, encode_aiff, encode_flac,
    encode_wav,
};
pub use dsp::{
    BiquadCoeffs, BiquadFilter, ChannelStrip, Compressor, DeEsser, EbuR128Meter, Eq7, ExpanderGate,
    FdnReverb, FilterType, KWeightingChannel, LAYOUT_5_1, LAYOUT_7_1_4, Maximizer, Oscillator,
    PanLaw, SpeakerPosition, SubtractiveSynth, TruePeakMeter, VbapPanner, Waveform,
    detect_pitch_yin, hz_to_midi, stereo_pan_gains,
};
pub use elastic_warp::{FlexWarpMode, WarpPoint, WsolaTimeStretch, evaluate_warp_timeline};
pub use mix_session::{
    AudioClip, AutomationCurve, AutomationPoint, DawSession, DawTrack, PdcMixEngine, TrackKind,
};
pub use music_intelligence::{
    CamelotKey, HarmonicRelation, MusicalPhrase, PhraseLabel, ScratchKey, SmpteFps,
    detect_camelot_key, encode_mtc_quarter_frame, evaluate_scratch_position, parse_scratch_dsl,
    segment_musical_phrases,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ebu_r128_metering() {
        let sr = 48000.0;
        let mut meter = EbuR128Meter::new(sr, 1);

        // Generate 1 second of 1 kHz sine at -20 dBFS (amplitude = 0.1)
        let num_samples = 48000;
        let mut samples = Vec::with_capacity(num_samples);
        for i in 0..num_samples {
            let t = (i as f32) / sr;
            samples.push(0.1 * (2.0 * std::f32::consts::PI * 1000.0 * t).sin());
        }

        meter.process_interleaved(&samples);
        let lufs = meter.integrated_lufs();
        // Sine at -20 dBFS with K-weighting high-shelf is approximately -20 to -23 LUFS
        assert!(lufs > -25.0 && lufs < -18.0, "Measured LUFS: {lufs}");
    }

    #[test]
    fn test_vbap_panning() {
        let panner = VbapPanner::new(&LAYOUT_5_1);
        let mut gains = [0.0f32; 6];

        // Pan center (0 degrees azimuth)
        panner.pan(0.0, 0.0, &mut gains);
        let power: f32 = gains.iter().map(|g| g * g).sum();
        assert!((power - 1.0).abs() < 1e-4, "VBAP power should be normalized to 1.0, got: {power}");
    }

    #[test]
    fn test_yin_pitch_detection() {
        let sr = 48000.0;
        let target_freq = 440.0f32; // A4
        let num_samples = 2048;
        let mut frame = Vec::with_capacity(num_samples);
        for i in 0..num_samples {
            let t = (i as f32) / sr;
            frame.push((2.0 * std::f32::consts::PI * target_freq * t).sin());
        }

        let detected = detect_pitch_yin(&frame, sr, 50.0, 1000.0, 0.15).expect("YIN failed");
        assert!((detected - target_freq).abs() < 1.0, "Detected frequency: {detected}");
        assert_eq!(hz_to_midi(detected), 69); // MIDI note 69 = A4
    }

    #[test]
    fn test_camelot_harmony_relations() {
        let k_8a = CamelotKey::new(8, 'A').unwrap(); // A minor
        let k_8b = CamelotKey::new(8, 'B').unwrap(); // C major
        let k_9a = CamelotKey::new(9, 'A').unwrap(); // E minor
        let k_10a = CamelotKey::new(10, 'A').unwrap(); // B minor

        assert_eq!(HarmonicRelation::between(k_8a, k_8a), HarmonicRelation::Same);
        assert_eq!(HarmonicRelation::between(k_8a, k_8b), HarmonicRelation::Relative);
        assert_eq!(HarmonicRelation::between(k_8a, k_9a), HarmonicRelation::Neighbor);
        assert_eq!(HarmonicRelation::between(k_8a, k_10a), HarmonicRelation::EnergyBoost);
    }

    #[test]
    fn test_wav_codec_roundtrip() {
        let mut buffer = AudioBuffer::new(44100, 2, 1000);
        for i in 0..1000 {
            buffer.channels[0][i] = 0.5 * (i as f32 / 100.0).sin();
            buffer.channels[1][i] = -0.5 * (i as f32 / 100.0).cos();
        }

        let wav_bytes = encode_wav(&buffer);
        let decoded = decode_wav(&wav_bytes).expect("WAV decode failed");

        assert_eq!(decoded.sample_rate, 44100);
        assert_eq!(decoded.num_channels(), 2);
        assert_eq!(decoded.num_frames(), 1000);
        assert!((decoded.channels[0][50] - buffer.channels[0][50]).abs() < 1e-3);
    }

    #[test]
    fn test_pdc_mix_engine() {
        let mut session = DawSession::new("TestSession", 48000, 120.0);
        let mut track1 = DawTrack::new(1, "Track1", TrackKind::Audio);
        track1.plugin_latency_samples = 128; // Has 128 samples plugin latency
        track1.clips.push(AudioClip {
            id: 101,
            start_sample: 0,
            length_samples: 512,
            source_offset_samples: 0,
            gain_db: 0.0,
            fade_in_samples: 0,
            fade_out_samples: 0,
            samples: vec![1.0; 512],
        });

        let mut track2 = DawTrack::new(2, "Track2", TrackKind::Audio);
        track2.plugin_latency_samples = 0; // 0 latency, requires 128 samples compensation delay
        track2.clips.push(AudioClip {
            id: 102,
            start_sample: 0,
            length_samples: 512,
            source_offset_samples: 0,
            gain_db: 0.0,
            fade_in_samples: 0,
            fade_out_samples: 0,
            samples: vec![1.0; 512],
        });

        session.tracks.push(track1);
        session.tracks.push(track2);

        let mut engine = PdcMixEngine::new();
        let (left, _right) = engine.render_block(&session, 0, 256);
        assert_eq!(left.len(), 256);
    }
}
