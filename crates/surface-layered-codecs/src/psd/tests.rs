use super::*;
use std::collections::HashMap;

#[test]
fn test_psd_header_and_blend_mode_keys() {
    for mode in [
        PsdBlendMode::Normal,
        PsdBlendMode::Multiply,
        PsdBlendMode::Screen,
        PsdBlendMode::Overlay,
        PsdBlendMode::SoftLight,
        PsdBlendMode::HardLight,
        PsdBlendMode::Difference,
        PsdBlendMode::Luminosity,
        PsdBlendMode::PassThrough,
    ] {
        let key = mode.to_key();
        assert_eq!(PsdBlendMode::from_key(&key), mode);
    }

    let hdr = PsdHeader {
        version: PsdVersion::Psb,
        channels: 4,
        height: 16,
        width: 32,
        depth: 8,
        color_mode: PsdColorMode::Rgb,
    };
    let encoded = hdr.encode();
    let (decoded, consumed) = PsdHeader::decode(&encoded).expect("header decode");
    assert_eq!(consumed, 26);
    assert_eq!(decoded, hdr);
}

#[test]
fn test_layer_rgba8_conversion() {
    let mut channels = HashMap::new();
    channels.insert(0, vec![10, 20]);
    channels.insert(1, vec![30, 40]);
    channels.insert(2, vec![50, 60]);
    channels.insert(-1, vec![200, 255]);

    let layer = PsdLayer {
        name: "Test".to_string(),
        top: 0,
        left: 0,
        bottom: 1,
        right: 2,
        blend_mode: PsdBlendMode::Normal,
        opacity: 255,
        clipping: false,
        visible: true,
        channels,
    };
    let rgba = layer.to_rgba8();
    assert_eq!(rgba, vec![[10, 30, 50, 200], [20, 40, 60, 255]]);
}
