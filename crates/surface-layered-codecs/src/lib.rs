//! # Surface Layered Codecs & Multi-Layer Document Exchange
//!
//! Provides pure-Rust, memory-bounded, zero-unsafe codecs for layered raster document formats:
//! - Adobe Photoshop PSD (v1) and PSB (v2 64-bit Large Document Format) encoder and decoder
//! - Channel compression: Apple PackBits RLE and Raw codecs
//! - Layer records: geometry, clipping masks, blend mode tags (`8BIM`), and Pascal layer names
//! - Adobe Brush Resource (`.abr`) parser for legacy v1/v2 and modern v6+ sampled brush tips
//! - Serif Affinity suite container format reader (`.afphoto`, `.afdesign`, `.afpub`)
//! - GIMP XCF (`.xcf`) and OpenRaster (`.ora`) multi-layer image decoders
//!
//! All execution and provenance guarantees are grounded in international human rights instruments
//! (UDHR Articles 12, 18, 19, 27; ICCPR Articles 17, 19; ICESCR Article 15) and Informatics Fiduciary duties.

#![forbid(unsafe_code)]

pub mod affinity;
pub mod ontology;
pub mod psd;
pub mod xcf_ora;

pub use affinity::{
    extract_affinity_preview, parse_affinity_archive, read_affinity_header, AffinityArchive,
    AffinityEntry, AffinityHeader, AffinityPreview,
};
pub use ontology::{
    get_descriptors, get_morphism_specs, AgenticMorphismSpec, FuncDescriptor, NQuin48,
    OntologicalContract,
};
pub use psd::{
    packbits, parse_abr, AbrBrush, AbrFile, AbrSampledTip, AbrTip, PsdBlendMode, PsdChannelInfo,
    PsdColorMode, PsdCompression, PsdDocument, PsdHeader, PsdLayer, PsdVersion,
};
pub use xcf_ora::{
    parse_ora, parse_xcf, OraDocument, OraLayer, XcfColorType, XcfDocument, XcfLayer,
};

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_packbits_rle_round_trip() {
        let input = vec![
            10, 10, 10, 10, 10, // 5 identical bytes -> run packet
            1, 2, 3, 4, 5,       // 5 distinct bytes -> literal packet
            255, 255, 255, 255, // 4 identical bytes -> run packet
            42,                 // single byte -> literal packet
        ];

        let encoded = packbits::encode(&input);
        assert!(!encoded.is_empty());

        let decoded = packbits::decode(&encoded, input.len()).expect("Packbits decode should succeed");
        assert_eq!(decoded, input, "Decoded PackBits output must match original input");
    }

    #[test]
    fn test_psd_encode_decode_round_trip() {
        let width = 8;
        let height = 8;
        let mut doc = PsdDocument::new(width, height, PsdColorMode::Rgb, false);

        // Add a layer
        let mut channels = HashMap::new();
        let red_plane = vec![255u8; (width * height) as usize];
        let green_plane = vec![128u8; (width * height) as usize];
        let blue_plane = vec![64u8; (width * height) as usize];
        let alpha_plane = vec![255u8; (width * height) as usize];

        channels.insert(0, red_plane);
        channels.insert(1, green_plane);
        channels.insert(2, blue_plane);
        channels.insert(-1, alpha_plane);

        let layer = PsdLayer {
            name: "Background Layer".to_string(),
            top: 0,
            left: 0,
            bottom: height as i32,
            right: width as i32,
            blend_mode: PsdBlendMode::Normal,
            opacity: 255,
            clipping: false,
            visible: true,
            channels,
        };
        doc.layers.push(layer);

        let encoded = doc.encode();
        assert!(encoded.len() > 26, "Encoded PSD file must contain valid sections");

        let decoded = PsdDocument::decode(&encoded).expect("PSD decode should succeed");
        assert_eq!(decoded.header.version, PsdVersion::Psd);
        assert_eq!(decoded.header.width, width);
        assert_eq!(decoded.header.height, height);
        assert_eq!(decoded.layers.len(), 1);

        let dec_layer = &decoded.layers[0];
        assert_eq!(dec_layer.name, "Background Layer");
        assert_eq!(dec_layer.blend_mode, PsdBlendMode::Normal);
        assert_eq!(dec_layer.opacity, 255);

        let rgba = dec_layer.to_rgba8();
        assert_eq!(rgba.len(), (width * height) as usize);
        assert_eq!(rgba[0], [255, 128, 64, 255]);
    }

    #[test]
    fn test_psb_64bit_encode_decode() {
        let width = 16;
        let height = 16;
        let mut doc = PsdDocument::new(width, height, PsdColorMode::Rgb, true);
        assert_eq!(doc.header.version, PsdVersion::Psb);

        let mut channels = HashMap::new();
        channels.insert(0, vec![200u8; (width * height) as usize]);
        channels.insert(1, vec![100u8; (width * height) as usize]);
        channels.insert(2, vec![50u8; (width * height) as usize]);

        doc.layers.push(PsdLayer {
            name: "Large PSB Layer".to_string(),
            top: 0,
            left: 0,
            bottom: height as i32,
            right: width as i32,
            blend_mode: PsdBlendMode::Multiply,
            opacity: 180,
            clipping: false,
            visible: true,
            channels,
        });

        let encoded = doc.encode();
        let decoded = PsdDocument::decode(&encoded).expect("PSB decode should succeed");
        assert_eq!(decoded.header.version, PsdVersion::Psb);
        assert_eq!(decoded.layers.len(), 1);
        assert_eq!(decoded.layers[0].blend_mode, PsdBlendMode::Multiply);
    }

    #[test]
    fn test_abr_computed_and_sampled_parsing() {
        // Construct synthetic ABR v1 stream with 1 computed brush
        let mut abr_data = Vec::new();
        abr_data.extend_from_slice(&1u16.to_be_bytes()); // version 1
        abr_data.extend_from_slice(&1u16.to_be_bytes()); // count 1

        // Brush 1: computed
        abr_data.extend_from_slice(&1u16.to_be_bytes()); // type 1 (computed)
        abr_data.extend_from_slice(&14u32.to_be_bytes()); // length 14 bytes
        abr_data.extend_from_slice(&0u32.to_be_bytes()); // misc
        abr_data.extend_from_slice(&25u16.to_be_bytes()); // spacing 25%
        abr_data.extend_from_slice(&45u16.to_be_bytes()); // diameter 45px
        abr_data.extend_from_slice(&100u16.to_be_bytes()); // roundness 100%
        abr_data.extend_from_slice(&0i16.to_be_bytes()); // angle 0 deg
        abr_data.extend_from_slice(&80u16.to_be_bytes()); // hardness 80%

        let parsed = parse_abr(&abr_data).expect("ABR v1 parse should succeed");
        assert_eq!(parsed.version, 1);
        assert_eq!(parsed.brushes.len(), 1);
        match &parsed.brushes[0].tip {
            AbrTip::Computed { diameter, hardness, .. } => {
                assert_eq!(*diameter, 45);
                assert_eq!(*hardness, 80);
            }
            _ => panic!("Expected computed brush tip"),
        }
    }

    #[test]
    fn test_affinity_container_header_inspection() {
        let mut container = vec![0u8; 80];
        container[0..4].copy_from_slice(affinity::AFFINITY_MAGIC);
        container[4..8].copy_from_slice(affinity::AFFINITY_DOC_KIND);
        container[8..10].copy_from_slice(&10u16.to_le_bytes()); // version 10
        container[10..12].copy_from_slice(&0u16.to_le_bytes()); // flags 0
        container[12..20].copy_from_slice(&0u64.to_le_bytes()); // fat_offset 0
        container[20..28].copy_from_slice(&0u64.to_le_bytes()); // thumbnail_offset 0
        container[28..32].copy_from_slice(&1700000000u32.to_le_bytes()); // saved timestamp

        let header = read_affinity_header(&container).expect("Affinity header read should succeed");
        assert_eq!(header.version, 10);
        assert_eq!(header.flags, 0);
        assert_eq!(header.saved_timestamp, 1700000000);

        let archive = parse_affinity_archive(&container).expect("Archive parse should succeed");
        assert_eq!(archive.header.version, 10);
    }

    #[test]
    fn test_xcf_parsing_synthetic() {
        let mut xcf_bytes = Vec::new();
        xcf_bytes.extend_from_slice(b"gimp xcf");
        xcf_bytes.extend_from_slice(b"file\0\0"); // version 0
        xcf_bytes.extend_from_slice(&100u32.to_be_bytes()); // width
        xcf_bytes.extend_from_slice(&100u32.to_be_bytes()); // height
        xcf_bytes.extend_from_slice(&0u32.to_be_bytes()); // base type RGB
        xcf_bytes.extend_from_slice(&0u32.to_be_bytes()); // PROP_END
        xcf_bytes.extend_from_slice(&0u32.to_be_bytes()); // PROP_END length
        xcf_bytes.extend_from_slice(&0u32.to_be_bytes()); // 0 terminator for layer pointers

        let doc = parse_xcf(&xcf_bytes).expect("XCF parse should succeed");
        assert_eq!(doc.width, 100);
        assert_eq!(doc.height, 100);
        assert_eq!(doc.color_type, XcfColorType::Rgb);
    }

    #[test]
    fn test_open_raster_parsing_synthetic() {
        // Create synthetic ZIP with stack.xml
        let stack_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<image w="800" h="600">
  <stack>
    <layer name="Foreground" src="data/layer0.png" x="10" y="20" opacity="0.9" visibility="visible" composite-op="svg:src-over"/>
  </stack>
</image>"#;

        let xml_bytes = stack_xml.as_bytes();
        let name_bytes = b"stack.xml";

        let mut zip_data = Vec::new();
        // Local file header: PK\x03\x04
        zip_data.extend_from_slice(b"PK\x03\x04");
        zip_data.extend_from_slice(&20u16.to_le_bytes()); // version needed
        zip_data.extend_from_slice(&0u16.to_le_bytes()); // general flag
        zip_data.extend_from_slice(&0u16.to_le_bytes()); // compression method: 0 (Store)
        zip_data.extend_from_slice(&0u32.to_le_bytes()); // time/date
        zip_data.extend_from_slice(&0u32.to_le_bytes()); // crc32
        zip_data.extend_from_slice(&(xml_bytes.len() as u32).to_le_bytes()); // compressed size
        zip_data.extend_from_slice(&(xml_bytes.len() as u32).to_le_bytes()); // uncompressed size
        zip_data.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes()); // name length
        zip_data.extend_from_slice(&0u16.to_le_bytes()); // extra length
        zip_data.extend_from_slice(name_bytes);
        zip_data.extend_from_slice(xml_bytes);

        let ora_doc = parse_ora(&zip_data).expect("ORA parse should succeed");
        assert_eq!(ora_doc.width, 800);
        assert_eq!(ora_doc.height, 600);
        assert_eq!(ora_doc.layers.len(), 1);
        assert_eq!(ora_doc.layers[0].name, "Foreground");
        assert_eq!(ora_doc.layers[0].x, 10);
        assert_eq!(ora_doc.layers[0].y, 20);
        assert_eq!(ora_doc.layers[0].opacity, 0.9);
    }

    #[test]
    fn test_ontology_contracts() {
        let descriptors = get_descriptors();
        assert_eq!(descriptors.len(), 3);
        for d in descriptors {
            assert_eq!(d.contract.triples.len(), 3);
            for quin in &d.contract.triples {
                assert!(quin.verify_parity());
            }
        }
    }
}
