use crate::*;
use std::sync::atomic::AtomicBool;
fn host<'a>(
    inputs: &'a [Resource],
    cancel: &'a AtomicBool,
    limits: &'a ToolLimits,
) -> AssetHost<'a> {
    AssetHost {
        workspace: AssetWorkspace::default(),
        inputs,
        limits,
        cancel,
        work: 0,
    }
}
fn args(epoch: f64) -> Args {
    let source = Value::Record(BTreeMap::from([
        ("position".into(), Value::List(vec![Value::F64(0.0); 3])),
        (
            "e_amplitude".into(),
            Value::List(vec![Value::F64(150.0), Value::F64(0.0), Value::F64(0.0)]),
        ),
        ("wavelength_nm".into(), Value::Null),
        (
            "stokes".into(),
            Value::List(vec![Value::F64(1.0), Value::F64(2.0), Value::F64(3.0)]),
        ),
    ]));
    BTreeMap::from([
        ("sources".into(), Value::List(vec![source])),
        ("epoch_s".into(), Value::F64(epoch)),
    ])
}
#[test]
fn current_morphism_production_verify_slice_ray_and_sound_agree() {
    let cancel = AtomicBool::new(false);
    let limits = ToolLimits::default();
    let mut host = host(&[], &cancel, &limits);
    let emf = host
        .electromagnetics_op("hypermedia.electromagnetics.compile_emf", &args(-0.125))
        .unwrap();
    let bytes = host.workspace.encoded(&emf).unwrap().bytes.clone();
    assert_eq!(&bytes[..8], b"Q42EMF64");
    assert_eq!(u16::from_le_bytes(bytes[8..10].try_into().unwrap()), 4);
    let verified = host
        .electromagnetics_op(
            "hypermedia.electromagnetics.verify_emf",
            &BTreeMap::from([("emf".into(), emf.clone())]),
        )
        .unwrap();
    let report = record(&verified).unwrap();
    assert_eq!(report.get("energy_density_j_per_m3"), Some(&Value::Null));
    assert_eq!(report.get("field_records"), Some(&Value::U64(512)));
    let projection = BTreeMap::from([
        ("emf".into(), emf.clone()),
        ("width".into(), Value::U64(8)),
        ("height".into(), Value::U64(8)),
    ]);
    let image = host
        .electromagnetics_op(
            "hypermedia.electromagnetics.render_spectral_slice",
            &projection,
        )
        .unwrap();
    assert_eq!(
        record(&image).unwrap().get("source_units"),
        Some(&Value::String("V/m".into()))
    );
    assert!(host.rgba(&image).unwrap().pixels.iter().any(|p| p[0] > 0));
    let ray = host
        .electromagnetics_geodesy_ext_op(
            "hypermedia.electromagnetics.raymarch_vox0_dda",
            &projection,
        )
        .unwrap();
    let image = record(&ray).unwrap().get("image").unwrap();
    assert!(host.rgba(image).unwrap().pixels.iter().any(|p| p[0] > 0));
    let tone = host
        .sound(
            "hypermedia.acoustics.synthesize_tone",
            &BTreeMap::from([
                ("emf".into(), emf),
                ("frequency".into(), Value::F64(1000.0)),
                ("frames".into(), Value::U64(4800)),
            ]),
        )
        .unwrap();
    assert_eq!(
        record(&tone).unwrap().get("acoustic_frequency_hz"),
        Some(&Value::F64(1000.0))
    );
}
#[test]
fn all_current_consumers_reject_corrupted_payload() {
    let cancel = AtomicBool::new(false);
    let limits = ToolLimits::default();
    let mut source_host = host(&[], &cancel, &limits);
    let emf = source_host.compile_emf_current(&args(1.5)).unwrap();
    let mut bytes = source_host.workspace.encoded(&emf).unwrap().bytes.clone();
    let view = qualia_core_db::q42::field_envelope::StructuralView::open(
        &bytes,
        qualia_core_db::q42::field_envelope::Family::EmfV4Candidate,
        qualia_core_db::q42::field_envelope::Limits::default(),
    )
    .unwrap();
    let at = view.entry(1).unwrap().payload_offset() as usize;
    bytes[at] ^= 1;
    let inputs = [Resource {
        key: "bad.emf".into(),
        kind: "emf".into(),
        bytes,
    }];
    let mut host = host(&inputs, &cancel, &limits);
    let args = BTreeMap::from([("emf".into(), Value::String("bad.emf".into()))]);
    assert!(host.verify_emf_current(&args).is_err());
    assert!(host.render_current_emf_slice(&args).is_err());
    assert!(host.raymarch_current_emf(&args).is_err());
    assert!(host
        .sound("hypermedia.acoustics.synthesize_tone", &args)
        .is_err());
}

#[test]
fn empty_current_verification_reports_missing_field_maxima() {
    use qualia_core_db::q42::field_envelope::{EnvelopeInput, Family, Limits};
    let input = EnvelopeInput {
        family: Family::EmfV4Candidate,
        flags: 0,
        metadata: &[],
        entries: &[],
    };
    let mut bytes = vec![0; input.encoded_len(Limits::default()).unwrap()];
    input.encode_into(&mut bytes, Limits::default()).unwrap();
    let inputs = [Resource {
        key: "empty.emf".into(),
        kind: "emf".into(),
        bytes,
    }];
    let cancel = AtomicBool::new(false);
    let limits = ToolLimits::default();
    let mut host = host(&inputs, &cancel, &limits);
    let report = host
        .verify_emf_current(&BTreeMap::from([(
            "emf".into(),
            Value::String("empty.emf".into()),
        )]))
        .unwrap();
    let report = record(&report).unwrap();
    assert_eq!(report.get("max_e_magnitude_v_per_m"), Some(&Value::Null));
    assert_eq!(
        report.get("max_abs_e_component_v_per_m"),
        Some(&Value::Null)
    );
}
