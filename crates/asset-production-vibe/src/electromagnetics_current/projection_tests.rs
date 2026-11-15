use super::*;
#[test]
fn magnitude_slice_and_dda_are_explicit_nonblack_derivatives() {
    let bytes = generated(
        &[Emitter {
            electric_amplitude: [150.0, 0.0, 0.0],
            ..emitter(None)
        }],
        -0.125,
    );
    let doc = open(&bytes);
    let grid = GridAtTime::new(&doc, 0).unwrap();
    let limits = ProjectionLimits {
        pixels: 64,
        work: 100_000,
    };
    let pixels = render_slice(&grid, 2, 0, 8, 8, 1.0, limits).unwrap();
    assert!(pixels.iter().any(|p| p[0] > 0));
    assert!(pixels
        .iter()
        .all(|p| p[0] == p[1] && p[1] == p[2] && p[3] == 255));
    let ray = render_raymarch(
        &grid,
        8,
        8,
        64,
        None,
        1.0,
        ProjectionLimits {
            pixels: 64,
            work: 1_000_000,
        },
    )
    .unwrap();
    assert!(ray.steps > 0);
    assert!(ray.present_samples > 0);
    assert!(ray.pixels.iter().any(|p| p[0] > 0 && p[3] > 0));
    assert!(render_slice(&grid, 2, 8, 8, 8, 1.0, limits).is_err());
    assert!(render_slice(
        &grid,
        2,
        0,
        8,
        8,
        1.0,
        ProjectionLimits {
            pixels: 0,
            work: 100_000
        }
    )
    .is_err());
}
#[test]
fn zero_and_extreme_fields_are_not_invented_irradiance_or_missing_values() {
    assert_eq!(
        super::super::projection::magnitude_brightness([0.0; 3], 1.0).unwrap(),
        0.0
    );
    assert_eq!(
        super::super::projection::magnitude_brightness([f64::MAX; 3], 1.0).unwrap(),
        1.0
    );
    let source = Emitter {
        electric_amplitude: [0.0; 3],
        ..emitter(None)
    };
    let bytes = generated(&[source], 0.0);
    let doc = open(&bytes);
    let grid = GridAtTime::new(&doc, 0).unwrap();
    assert!(grid.sample([0; 3]).unwrap().is_some());
    let pixels = render_slice(
        &grid,
        2,
        0,
        1,
        1,
        1.0,
        ProjectionLimits {
            pixels: 1,
            work: 100,
        },
    )
    .unwrap();
    assert_eq!(pixels, vec![[0, 0, 0, 255]]);
    assert!(super::super::projection::magnitude_brightness([1.0; 3], 0.0).is_err());
}
#[test]
fn acoustic_frequency_is_chosen_independently_of_the_em_field() {
    let bytes = generated(
        &[Emitter {
            electric_amplitude: [150.0, 0.0, 0.0],
            ..emitter(None)
        }],
        0.0,
    );
    let doc = open(&bytes);
    let tone =
        super::super::sonification::sonify(&doc, 48000, 4800, 1000.0, 0.25, 1 << 20).unwrap();
    assert_eq!(tone.source_samples, 512);
    assert_eq!(tone.missing_samples, 0);
    assert!(tone.samples.iter().any(|s| s.abs() > 0.01));
    assert!(tone
        .samples
        .iter()
        .all(|s| s.is_finite() && s.abs() <= 0.25));
    assert!(super::super::sonification::sonify(&doc, 48000, 100, 24000.0, 0.25, 1 << 20).is_err());
}

#[test]
fn sparse_missing_and_valid_zero_cells_remain_distinct() {
    let mut payloads = [[0u8; 128]; 2];
    for (index, cell) in [0.0, 2.0].into_iter().enumerate() {
        encode_field_samples(
            &[FieldSample {
                electric_v_per_m: if index == 0 {
                    [2.0, 0.0, 0.0]
                } else {
                    [0.0; 3]
                },
                magnetic_tesla: [0.0; 3],
                position_m: [cell, 0.0, 0.0],
                tick: 0,
                carrier_hz: None,
                stored_poynting_w_per_m2: None,
                confidence: None,
            }],
            &mut payloads[index],
            Limits::default(),
        )
        .unwrap();
    }
    let chunks = [
        FieldChunkInput {
            stream_id: 1,
            start_tick: 0,
            time_count: 1,
            spatial_key: Some(0),
            payload: &payloads[0],
        },
        FieldChunkInput {
            stream_id: 1,
            start_tick: 0,
            time_count: 1,
            spatial_key: Some(2),
            payload: &payloads[1],
        },
    ];
    let doc = FieldDocument {
        clock: FieldClock {
            scale: 1,
            epoch_seconds: 0,
            epoch_nanoseconds: 0,
            ticks_per_second_num: 1,
            ticks_per_second_den: 1,
        },
        frame: FieldFrame {
            origin_m: [0.0; 3],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            grid: Some(GridSpec {
                dimensions: [3, 1, 1],
                brick_dimensions: [1; 3],
                spacing_m: [1.0; 3],
                indexing: GridIndexing::Linear,
            }),
        },
        provenance: FieldProvenance {
            role: 0,
            algorithm: None,
            algorithm_version: None,
            lineage: None,
        },
        temporal_step_ticks: Some(1),
        chunks: &chunks,
        align_payloads_to_4096: false,
    };
    let limits = FieldWriteLimits::default();
    let mut metadata = vec![0; field_metadata_len(&doc, limits).unwrap()];
    let mut entries = [field_entry_slot(); 2];
    let prepared = prepare_field_document(&doc, &mut metadata, &mut entries, limits).unwrap();
    let size = prepared.encoded_len().unwrap();
    let mut stage = vec![0; size];
    let mut bytes = vec![0; size];
    let mut m = [qualia_core_db::q42::field_semantics::NodeState::default(); 5];
    let mut e = [qualia_core_db::q42::field_semantics::NodeState::default(); 2];
    prepared
        .encode_into(
            &mut stage,
            &mut bytes,
            &mut qualia_core_db::q42::field_semantics::Scratch {
                metadata: &mut m,
                entries: &mut e,
            },
        )
        .unwrap();
    let document = open(&bytes);
    let grid = GridAtTime::new(&document, 0).unwrap();
    assert!(grid.sample([1, 0, 0]).unwrap().is_none());
    assert_eq!(
        grid.sample([2, 0, 0]).unwrap().unwrap().electric_v_per_m(),
        [0.0; 3]
    );
    let pixels = render_slice(
        &grid,
        2,
        0,
        3,
        1,
        1.0,
        ProjectionLimits {
            pixels: 3,
            work: 100,
        },
    )
    .unwrap();
    assert!(pixels[0][0] > 0);
    assert_eq!(pixels[1], [0; 4]);
    assert_eq!(pixels[2], [0, 0, 0, 255]);
}
#[test]
fn invalid_masked_cells_stay_absent_through_cached_lookup() {
    use qualia_core_db::q42::field_envelope::{
        BlockInput, EntryInput, EnvelopeInput, FieldInput, FieldValue as V, MetadataInput,
    };
    let source = generated(&[emitter(None)], 0.0);
    let source_view =
        StructuralView::open(&source, Family::EmfV4Candidate, StorageLimits::default()).unwrap();
    let mut definitions: Vec<_> = source_view
        .blocks()
        .map(|b| {
            (
                b.id(),
                b.schema_type(),
                b.revision(),
                b.flags(),
                b.fields()
                    .map(|f| FieldInput {
                        key: f.key(),
                        flags: f.flags(),
                        value: V::Raw {
                            type_tag: f.type_tag(),
                            element_count: f.element_count(),
                            bytes: f.value(),
                        },
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    let desc = &mut definitions.iter_mut().find(|b| b.0 == 5).unwrap().4;
    desc.push(FieldInput {
        key: 15,
        flags: 1,
        value: V::U64(&[511]),
    });
    desc.push(FieldInput {
        key: 16,
        flags: 1,
        value: V::EntryRefs(&[2]),
    });
    desc.sort_by_key(|f| f.key);
    let f = |key, value| FieldInput {
        key,
        flags: 1,
        value,
    };
    definitions.push((
        6,
        1,
        1,
        1,
        vec![
            f(1, V::U32(&[0])),
            f(2, V::I64(&[0])),
            f(3, V::U32(&[0])),
            f(4, V::U64(&[1])),
            f(5, V::U64(&[1])),
        ],
    ));
    definitions.push((
        7,
        103,
        1,
        1,
        vec![
            f(1, V::U32(&[0x106])),
            f(2, V::U32(&[1])),
            f(3, V::MetadataRefs(&[6])),
            f(5, V::MetadataRefs(&[3])),
            f(6, V::Utf8("validity_v1")),
            f(7, V::Utf8("linear")),
            f(8, V::U64(&[1, 1, 1])),
            f(9, V::U32(&[1, 1, 1])),
            f(10, V::U32(&[0])),
            f(11, V::I64(&[0])),
        ],
    ));
    definitions.push((
        8,
        3,
        1,
        1,
        vec![
            f(3, V::MetadataRefs(&[3])),
            f(7, V::Utf8("validity")),
            f(8, V::Utf8("1")),
            f(9, V::U32(&[1])),
            f(10, V::Utf8("byte")),
            f(11, V::U32(&[7])),
            f(12, V::Utf8("urn:qualia:emf:validity_v1")),
            f(18, V::MetadataRefs(&[7])),
        ],
    ));
    let blocks: Vec<_> = definitions
        .iter()
        .map(|b| BlockInput {
            id: b.0,
            schema_type: b.1,
            revision: b.2,
            flags: b.3,
            fields: &b.4,
        })
        .collect();
    let input = MetadataInput { blocks: &blocks };
    let mut metadata = vec![0; input.encoded_len(StorageLimits::default()).unwrap()];
    input
        .encode_into(&mut metadata, StorageLimits::default())
        .unwrap();
    let entry = source_view.entry(1).unwrap();
    let mut payload = source_view.verify_chunk(1).unwrap().to_vec();
    payload[..128].fill(0);
    let mut position = [0; 24];
    position.copy_from_slice(&payload[176..200]);
    payload[128..256].fill(0);
    payload[176..200].copy_from_slice(&position);
    let mut mask = vec![1u8; 512];
    mask[0] = 0;
    let rows = [
        EntryInput {
            mode: 0,
            revision: 1,
            flags: entry.flags(),
            stream_id: entry.stream_id(),
            frame_id: entry.frame_id(),
            start_tick: entry.start_tick(),
            time_count: entry.time_count(),
            spatial_key: entry.spatial_key(),
            record_count: 512,
            stride: 128,
            codec: 0,
            descriptor_id: 5,
            payload: &payload,
        },
        EntryInput {
            mode: 0x106,
            revision: 1,
            flags: 0,
            stream_id: 1,
            frame_id: 0,
            start_tick: 0,
            time_count: 0,
            spatial_key: 0,
            record_count: 512,
            stride: 1,
            codec: 0,
            descriptor_id: 8,
            payload: &mask,
        },
    ];
    let input = EnvelopeInput {
        family: Family::EmfV4Candidate,
        flags: 0,
        metadata: &metadata,
        entries: &rows,
    };
    let mut bytes = vec![0; input.encoded_len(StorageLimits::default()).unwrap()];
    input
        .encode_into(&mut bytes, StorageLimits::default())
        .unwrap();
    let document = open(&bytes);
    let grid = GridAtTime::new(&document, 0).unwrap();
    assert!(grid.sample([0, 0, 0]).unwrap().is_none());
    assert_eq!(
        grid.sample([1, 0, 0]).unwrap().unwrap().electric_v_per_m(),
        [0.0; 3]
    );
    let pixels = render_slice(
        &grid,
        2,
        0,
        8,
        8,
        1.0,
        ProjectionLimits {
            pixels: 64,
            work: 10000,
        },
    )
    .unwrap();
    assert_eq!(pixels[0], [0; 4]);
    assert_eq!(pixels[1], [0, 0, 0, 255]);
}
