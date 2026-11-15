use super::*;
#[test]
fn generation_keeps_f64_fields_optional_carrier_and_model_context() {
    let source = emitter(Some(532e-9));
    let bytes = generated(&[source.clone()], 0.0);
    let doc = open(&bytes);
    let field = doc.chunks[0].field(0).unwrap().unwrap();
    assert_eq!(
        field.electric_v_per_m()[0].to_bits(),
        source.electric_amplitude[0].to_bits()
    );
    assert_ne!(
        field.electric_v_per_m()[0],
        (source.electric_amplitude[0] as f32) as f64
    );
    assert_eq!(field.stored_poynting_w_per_m2(), None);
    assert_eq!(field.confidence(), None);
    assert!(field.carrier_hz().unwrap() > 0.0);
    assert_eq!(doc.summary.records, [512, 0, 0]);
    let mixed = generated(&[source, emitter(Some(1e-3))], 0.0);
    let doc = open(&mixed);
    assert_eq!(doc.chunks[0].field(0).unwrap().unwrap().carrier_hz(), None);
    let bytes = generated(&[emitter(None)], 0.0);
    let doc = open(&bytes);
    let field = doc.chunks[0].field(0).unwrap().unwrap();
    assert_eq!(field.carrier_hz(), None);
    assert_eq!(field.magnetic_tesla(), [0.0; 3]);
}
#[test]
fn stored_physics_has_no_visible_rf_or_gamma_band_gate() {
    for wavelength in [1e8, 0.1249, 1e-3, 1e-5, 1e-6, 532e-9, 100e-9, 1e-10, 1e-12] {
        let bytes = generated(&[emitter(Some(wavelength))], 0.0);
        let doc = open(&bytes);
        assert!(doc.chunks[0]
            .field(0)
            .unwrap()
            .unwrap()
            .carrier_hz()
            .is_some());
        let grid = GridAtTime::new(&doc, 0).unwrap();
        assert!(grid.sample([0; 3]).unwrap().is_some());
    }
}
#[test]
fn malformed_payload_and_admission_budgets_fail_before_cached_use() {
    let mut bytes = generated(&[emitter(None)], 0.0);
    let view =
        StructuralView::open(&bytes, Family::EmfV4Candidate, StorageLimits::default()).unwrap();
    let at = view.entry(1).unwrap().payload_offset() as usize;
    bytes[at] ^= 1;
    assert!(CurrentDocument::open(
        &bytes,
        ReadLimits {
            file_bytes: 4 << 20,
            cold_work: 50_000_000,
            cache_bytes: 8 << 20
        }
    )
    .is_err());
    let bytes = generated(&[emitter(None)], 0.0);
    assert!(CurrentDocument::open(
        &bytes,
        ReadLimits {
            file_bytes: bytes.len() - 1,
            cold_work: 50_000_000,
            cache_bytes: 8 << 20
        }
    )
    .is_err());
    assert!(CurrentDocument::open(
        &bytes,
        ReadLimits {
            file_bytes: 4 << 20,
            cold_work: 0,
            cache_bytes: 8 << 20
        }
    )
    .is_err());
    assert!(CurrentDocument::open(
        &bytes,
        ReadLimits {
            file_bytes: 4 << 20,
            cold_work: 50_000_000,
            cache_bytes: 0
        }
    )
    .is_err());
}

#[test]
fn empty_document_has_no_field_maximum_and_host_charge_is_prebounded() {
    use qualia_core_db::q42::field_envelope::EnvelopeInput;
    let input = EnvelopeInput {
        family: Family::EmfV4Candidate,
        flags: 0,
        metadata: &[],
        entries: &[],
    };
    let mut bytes = vec![0; input.encoded_len(StorageLimits::default()).unwrap()];
    input
        .encode_into(&mut bytes, StorageLimits::default())
        .unwrap();
    let document = open(&bytes);
    assert_eq!(document.summary.valid_records[0], 0);
    assert_eq!(document.summary.max_e_magnitude, None);
    let view = &document.view;
    let bound = super::super::document::host_work_bound(
        read_work_bound(
            view.metadata_bytes().len(),
            view.blocks().count(),
            view.entry_count() as usize,
        )
        .unwrap(),
        bytes.len(),
    )
    .unwrap();
    assert!(document.host_work_used().unwrap() <= bound);
    let populated = generated(&[emitter(None)], 0.0);
    let document = open(&populated);
    let view = &document.view;
    let bound = super::super::document::host_work_bound(
        read_work_bound(
            view.metadata_bytes().len(),
            view.blocks().count(),
            view.entry_count() as usize,
        )
        .unwrap(),
        populated.len(),
    )
    .unwrap();
    assert!(document.host_work_used().unwrap() <= bound);
    assert!(super::super::document::host_work_bound(u64::MAX, 1).is_err());
}
