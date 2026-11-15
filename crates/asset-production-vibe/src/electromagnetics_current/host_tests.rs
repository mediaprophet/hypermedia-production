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

fn verify_and_consume_v4_resource(
    bytes: Vec<u8>,
    expected_profile: &str,
    expected_min_dipoles: u64,
    label: &str,
) {
    let cancel = AtomicBool::new(false);
    let limits = ToolLimits::default();
    let mut host = host(&[], &cancel, &limits);
    let handle = host.admit_encoded("emf", bytes).unwrap();

    let verify = host
        .invoke(
            "hypermedia.electromagnetics.verify_emf",
            &Value::Record(BTreeMap::from([("emf".into(), handle.clone())])),
        )
        .unwrap_or_else(|e| panic!("{label} verify_emf failed: {e}"));
    let verify_rec = record(&verify).unwrap();
    assert_eq!(verify_rec.get("valid"), Some(&Value::Bool(true)), "{label}");
    assert_eq!(
        verify_rec.get("container_version"),
        Some(&Value::U64(4)),
        "{label}"
    );
    assert_eq!(
        verify_rec.get("format_profile"),
        Some(&Value::String(expected_profile.into())),
        "{label}"
    );
    let dipoles = match verify_rec.get("dipole_records") {
        Some(Value::U64(n)) => *n,
        other => panic!("{label} missing dipole_records: {other:?}"),
    };
    assert!(dipoles >= expected_min_dipoles, "{label} dipole_records={dipoles}");
    assert!(verify_rec.contains_key("radiative_transfer_records"));
    assert!(verify_rec.contains_key("sh_coeff_records"));
    assert!(verify_rec.contains_key("constitutive_records"));
    assert!(verify_rec.contains_key("validity_mask_records"));

    let slice = host
        .invoke(
            "hypermedia.electromagnetics.render_spectral_slice",
            &Value::Record(BTreeMap::from([
                ("emf".into(), handle.clone()),
                ("width".into(), Value::U64(32)),
                ("height".into(), Value::U64(32)),
                ("exposure".into(), Value::F64(1.0e3)),
            ])),
        )
        .unwrap_or_else(|e| panic!("{label} render_spectral_slice failed: {e}"));
    match host.workspace.get(&slice).unwrap() {
        Artifact::Rgba(rgba) => {
            assert_eq!(rgba.width, 32, "{label}");
            assert_eq!(rgba.height, 32, "{label}");
            assert_eq!(rgba.pixels.len(), 32 * 32, "{label}");
            assert!(
                rgba.pixels.iter().any(|p| p[3] > 0 && (p[0] > 0 || p[1] > 0 || p[2] > 0)),
                "{label} slice should render non-black field pixels"
            );
        }
        _ => panic!("{label} slice did not return rgba"),
    }

    let raymarch = host
        .invoke(
            "hypermedia.electromagnetics.raymarch_vox0_dda",
            &Value::Record(BTreeMap::from([
                ("emf".into(), handle.clone()),
                ("width".into(), Value::U64(32)),
                ("height".into(), Value::U64(32)),
            ])),
        )
        .unwrap_or_else(|e| panic!("{label} raymarch_vox0_dda failed: {e}"));
    let raymarch_rec = record(&raymarch).unwrap();
    let rm_img = raymarch_rec
        .get("image")
        .unwrap_or_else(|| panic!("{label} raymarch_vox0_dda missing image handle"));
    match host.workspace.get(rm_img).unwrap() {
        Artifact::Rgba(rgba) => {
            assert_eq!(rgba.width, 32, "{label}");
            assert_eq!(rgba.height, 32, "{label}");
            assert_eq!(rgba.pixels.len(), 32 * 32, "{label}");
        }
        _ => panic!("{label} raymarch_vox0_dda image did not resolve to rgba"),
    }

    let current_rm = host
        .raymarch_current_emf(&BTreeMap::from([
            ("emf".into(), handle.clone()),
            ("width".into(), Value::U64(32)),
            ("height".into(), Value::U64(32)),
        ]))
        .unwrap_or_else(|e| panic!("{label} raymarch_current_emf failed: {e}"));
    let current_rm_rec = record(&current_rm).unwrap();
    let current_rm_img = current_rm_rec.get("image").unwrap_or(&current_rm);
    let current_rm_img_rec = record(current_rm_img).unwrap();
    assert_eq!(
        current_rm_img_rec.get("projection_profile"),
        Some(&Value::String("electric_magnitude_grid_alpha_dda_v1".into())),
        "{label}"
    );
    match host.workspace.get(current_rm_img).unwrap() {
        Artifact::Rgba(rgba) => {
            assert_eq!(rgba.width, 32, "{label}");
            assert_eq!(rgba.height, 32, "{label}");
            assert_eq!(rgba.pixels.len(), 32 * 32, "{label}");
        }
        _ => panic!("{label} raymarch_current_emf did not return rgba"),
    }

    let sonic = host
        .invoke(
            "hypermedia.acoustics.synthesize_tone",
            &Value::Record(BTreeMap::from([
                ("emf".into(), handle),
                ("frames".into(), Value::U64(2048)),
            ])),
        )
        .unwrap_or_else(|e| panic!("{label} synthesize_tone failed: {e}"));
    match host.workspace.get(&sonic).unwrap() {
        Artifact::Audio { rate, samples } => {
            assert_eq!(*rate, 48000, "{label}");
            assert_eq!(samples.len(), 2048, "{label}");
            assert!(samples.iter().all(|s| s.is_finite()), "{label}");
            assert!(
                samples.iter().any(|s| s.abs() > 0.0),
                "{label} sonification should be non-silent"
            );
        }
        _ => panic!("{label} synthesize_tone did not return audio"),
    }
}

#[test]
fn anatomy_bridge_v4_canonical_and_profile_resources_roundtrip_all_consumers() {
    use crate::emf_qac_anatomy_bridge::{
        canonical_ghost_gum_metamer_rig_and_corpus, canonical_wallaby_rig_and_corpus,
        produce_botanical_emf_v4, produce_vertebrate_emf_v4,
    };

    let (wallaby_corpus, wallaby_rig) = canonical_wallaby_rig_and_corpus().unwrap();
    let wallaby_bundle =
        produce_vertebrate_emf_v4(&wallaby_rig, &wallaby_corpus, "up03-wallaby").unwrap();
    verify_and_consume_v4_resource(
        wallaby_bundle.canonical_resource.bytes,
        "wire_freeze_v4",
        1,
        "wallaby canonical",
    );
    verify_and_consume_v4_resource(
        wallaby_bundle.profile_resource.bytes,
        "profile_v4",
        0,
        "wallaby profile",
    );

    let (gum_corpus, gum_rig) = canonical_ghost_gum_metamer_rig_and_corpus().unwrap();
    let gum_bundle = produce_botanical_emf_v4(&gum_rig, &gum_corpus, "up03-gum").unwrap();
    verify_and_consume_v4_resource(
        gum_bundle.canonical_resource.bytes,
        "wire_freeze_v4",
        0,
        "ghost gum canonical",
    );
    verify_and_consume_v4_resource(
        gum_bundle.profile_resource.bytes,
        "profile_v4",
        0,
        "ghost gum profile",
    );
}

#[test]
fn dipole_only_wire_freeze_v4_container_slices_raymarches_and_sonifies() {
    use crate::emf_qac_anatomy_bridge::{
        canonical_wallaby_rig_and_corpus, produce_vertebrate_emf_v4,
    };
    use qualia_core_db::q42::field_envelope::{
        EntryInput, EnvelopeInput, Family, Limits, StructuralView,
    };

    let (corpus, rig) = canonical_wallaby_rig_and_corpus().unwrap();
    let bundle = produce_vertebrate_emf_v4(&rig, &corpus, "up03-dipole-only").unwrap();
    let bytes = bundle.canonical_resource.bytes;
    let view = StructuralView::open(&bytes, Family::EmfV4Candidate, Limits::default()).unwrap();

    let mut dipole_idx = None;
    let mut dipole_payload = Vec::new();
    for i in 1..=view.entry_count() {
        let entry = view.entry(i).unwrap();
        if entry.mode() == 0x0107 {
            dipole_idx = Some(entry);
            dipole_payload = view.verify_chunk(i).unwrap().to_vec();
            break;
        }
    }
    let entry = dipole_idx.expect("wallaby canonical bundle should carry mode 0x0107");
    let entries = [EntryInput {
        mode: entry.mode(),
        revision: entry.revision(),
        flags: entry.flags(),
        stream_id: entry.stream_id(),
        frame_id: entry.frame_id(),
        start_tick: entry.start_tick(),
        time_count: entry.time_count(),
        spatial_key: entry.spatial_key(),
        record_count: entry.record_count(),
        stride: entry.stride(),
        codec: 0,
        descriptor_id: entry.descriptor_id(),
        payload: &dipole_payload,
    }];
    let env = EnvelopeInput {
        family: Family::EmfV4Candidate,
        flags: 0,
        metadata: view.metadata_bytes(),
        entries: &entries,
    };
    let mut dipole_only = vec![0u8; env.encoded_len(Limits::default()).unwrap()];
    env.encode_into(&mut dipole_only, Limits::default()).unwrap();

    verify_and_consume_v4_resource(dipole_only, "wire_freeze_v4", 1, "dipole-only 0x0107");
}

#[test]
fn bioelectric_dipole_bad_record_crc32c_fails_closed() {
    use crate::emf_qac_anatomy_bridge::{
        canonical_wallaby_rig_and_corpus, produce_vertebrate_emf_v4,
    };
    use qualia_core_db::q42::field_envelope::{
        EntryInput, EnvelopeInput, Family, Limits, StructuralView,
    };

    let (corpus, rig) = canonical_wallaby_rig_and_corpus().unwrap();
    let bundle = produce_vertebrate_emf_v4(&rig, &corpus, "up03-tamper").unwrap();
    let bytes = bundle.canonical_resource.bytes;
    let view = StructuralView::open(&bytes, Family::EmfV4Candidate, Limits::default()).unwrap();

    let mut payloads: Vec<Vec<u8>> = (1..=view.entry_count())
        .map(|i| view.verify_chunk(i).unwrap().to_vec())
        .collect();
    let mut tampered = false;
    for i in 1..=view.entry_count() {
        if view.entry(i).unwrap().mode() == 0x0107 {
            payloads[(i - 1) as usize][124] ^= 0x01;
            tampered = true;
        }
    }
    assert!(tampered, "expected mode 0x0107 entry in canonical bundle");

    let entries: Vec<EntryInput<'_>> = (1..=view.entry_count())
        .map(|i| {
            let entry = view.entry(i).unwrap();
            EntryInput {
                mode: entry.mode(),
                revision: entry.revision(),
                flags: entry.flags(),
                stream_id: entry.stream_id(),
                frame_id: entry.frame_id(),
                start_tick: entry.start_tick(),
                time_count: entry.time_count(),
                spatial_key: entry.spatial_key(),
                record_count: entry.record_count(),
                stride: entry.stride(),
                codec: 0,
                descriptor_id: entry.descriptor_id(),
                payload: &payloads[(i - 1) as usize],
            }
        })
        .collect();
    let env = EnvelopeInput {
        family: Family::EmfV4Candidate,
        flags: 0,
        metadata: view.metadata_bytes(),
        entries: &entries,
    };
    let mut resealed = vec![0u8; env.encoded_len(Limits::default()).unwrap()];
    env.encode_into(&mut resealed, Limits::default()).unwrap();

    let cancel = AtomicBool::new(false);
    let limits = ToolLimits::default();
    let mut host = host(&[], &cancel, &limits);
    let handle = host.admit_encoded("emf", resealed).unwrap();
    let arg = Value::Record(BTreeMap::from([("emf".into(), handle)]));

    let err = host
        .invoke("hypermedia.electromagnetics.verify_emf", &arg)
        .unwrap_err();
    assert!(err.contains("CRC-32C"), "unexpected error: {err}");
    assert!(host
        .invoke("hypermedia.electromagnetics.render_spectral_slice", &arg)
        .is_err());
    assert!(host
        .invoke("hypermedia.electromagnetics.raymarch_vox0_dda", &arg)
        .is_err());
    assert!(host
        .invoke("hypermedia.acoustics.synthesize_tone", &arg)
        .is_err());
}

#[test]
fn compile_emf_accepts_dipoles_and_constitutive_and_fails_closed_on_invalid() {
    let cancel = AtomicBool::new(false);
    let limits = ToolLimits::default();
    let mut host = host(&[], &cancel, &limits);

    let base_sources = Value::List(vec![Value::Record(BTreeMap::from([
        (
            "position".into(),
            Value::List(vec![Value::F64(0.25), Value::F64(0.25), Value::F64(0.25)]),
        ),
        (
            "e_amplitude".into(),
            Value::List(vec![Value::F64(5.0), Value::F64(0.0), Value::F64(0.0)]),
        ),
        ("wavelength_nm".into(), Value::F64(550.0)),
    ]))]);
    let dipoles = Value::List(vec![
        Value::Record(BTreeMap::from([
            (
                "position".into(),
                Value::List(vec![Value::F64(0.1), Value::F64(0.2), Value::F64(0.3)]),
            ),
            (
                "moment".into(),
                Value::List(vec![Value::F64(1.2e-9), Value::F64(0.0), Value::F64(0.0)]),
            ),
            ("carrier_hz".into(), Value::F64(12.0)),
            ("potential_v".into(), Value::F64(-0.065)),
            ("label".into(), Value::String("vagus_axon".into())),
        ])),
        Value::Record(BTreeMap::from([
            (
                "position".into(),
                Value::List(vec![Value::F64(0.4), Value::F64(0.2), Value::F64(0.1)]),
            ),
            (
                "current_dipole_moment_a_m".into(),
                Value::List(vec![Value::F64(0.0), Value::F64(2.5e-9), Value::F64(0.0)]),
            ),
        ])),
    ]);
    let constitutive = Value::Record(BTreeMap::from([
        ("epsilon_r".into(), Value::F64(78.4)),
        ("mu_r".into(), Value::F64(1.0)),
        ("conductivity_s_per_m".into(), Value::F64(0.35)),
        ("model".into(), Value::String("debye_tissue".into())),
        ("label".into(), Value::String("extracellular_saline".into())),
    ]));

    let compiled = host
        .invoke(
            "hypermedia.electromagnetics.compile_emf",
            &Value::Record(BTreeMap::from([
                ("sources".into(), base_sources.clone()),
                ("dipoles".into(), dipoles),
                ("constitutive".into(), constitutive),
            ])),
        )
        .unwrap();
    let rec = record(&compiled).unwrap();
    assert_eq!(rec.get("dipole_records"), Some(&Value::U64(2)));
    let dipole_sum = record(rec.get("bioelectric_dipole_summary").unwrap()).unwrap();
    assert_eq!(dipole_sum.get("dipole_count"), Some(&Value::U64(2)));
    assert_eq!(
        dipole_sum.get("schema"),
        Some(&Value::String("bioelectric_dipole_v1".into()))
    );
    let const_sum = record(rec.get("constitutive_summary").unwrap()).unwrap();
    assert_eq!(const_sum.get("epsilon_relative"), Some(&Value::F64(78.4)));
    assert_eq!(const_sum.get("conductivity_s_per_m"), Some(&Value::F64(0.35)));

    let verified = host
        .invoke(
            "hypermedia.electromagnetics.verify_emf",
            &Value::Record(BTreeMap::from([("emf".into(), compiled)])),
        )
        .unwrap();
    let vrec = record(&verified).unwrap();
    let prov = match vrec.get("provenance") {
        Some(Value::List(items)) => items,
        other => panic!("missing provenance: {other:?}"),
    };
    let lineage = prov
        .iter()
        .find_map(|item| record(item).ok()?.get("lineage").and_then(|v| match v {
            Value::String(s) => Some(s.as_str()),
            _ => None,
        }))
        .expect("compiled provenance lineage missing");
    assert!(lineage.contains("vagus_axon"), "lineage={lineage}");
    assert!(lineage.contains("debye_tissue"), "lineage={lineage}");

    // Fail-closed cases: empty dipoles & negative epsilon_r.
    let bad_empty_dipoles = host.invoke(
        "hypermedia.electromagnetics.compile_emf",
        &Value::Record(BTreeMap::from([
            ("sources".into(), base_sources.clone()),
            ("dipoles".into(), Value::List(vec![])),
        ])),
    );
    assert!(bad_empty_dipoles.is_err());

    let bad_constitutive = host.invoke(
        "hypermedia.electromagnetics.compile_emf",
        &Value::Record(BTreeMap::from([
            ("sources".into(), base_sources),
            (
                "constitutive".into(),
                Value::Record(BTreeMap::from([("epsilon_r".into(), Value::F64(-1.0))])),
            ),
        ])),
    );
    assert!(bad_constitutive.is_err());
}
