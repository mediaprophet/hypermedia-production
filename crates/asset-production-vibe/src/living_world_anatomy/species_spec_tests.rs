use super::behavioral_hooks::compile_model_logic_q42_bytes;
use super::rule_pack::*;
use super::*;
use std::collections::BTreeMap;
use std::sync::atomic::AtomicBool;

const VERTEBRATE_OP: &str = "hypermedia.manifold.compile_vertebrate_anatomy_10d";

fn rec(pairs: Vec<(&str, Value)>) -> Value {
    Value::Record(
        pairs
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect::<BTreeMap<_, _>>(),
    )
}

fn s(v: &str) -> Value {
    Value::String(v.into())
}

fn v3(x: f64, y: f64, z: f64) -> Value {
    Value::List(vec![Value::F64(x), Value::F64(y), Value::F64(z)])
}

fn test_spec() -> Value {
    rec(vec![
        ("schema", s(species_spec::SCAFFOLD_SCHEMA_V1)),
        ("species_id", s("test-species")),
        ("archetype", s("macropod_saltatorial")),
        (
            "submeshes",
            Value::List(vec![
                rec(vec![
                    ("id", s("ost_femur")),
                    ("layer", s("osteology")),
                    (
                        "long_bones",
                        Value::List(vec![rec(vec![
                            ("prox", v3(0.0, 0.40, 0.0)),
                            ("dist", v3(0.0, 0.20, 0.02)),
                            ("shaft_r", Value::F64(0.01)),
                        ])]),
                    ),
                ]),
                rec(vec![
                    ("id", s("myo_thigh")),
                    ("layer", s("myology")),
                    (
                        "muscles",
                        Value::List(vec![rec(vec![
                            ("origin", v3(0.0, 0.40, 0.0)),
                            ("insertion", v3(0.0, 0.22, 0.02)),
                            ("rx", Value::F64(0.02)),
                        ])]),
                    ),
                ]),
                rec(vec![
                    ("id", s("int_body")),
                    ("layer", s("integument")),
                    (
                        "lofts",
                        Value::List(vec![rec(vec![(
                            "stations",
                            Value::List(vec![
                                rec(vec![
                                    ("center", v3(0.0, 0.40, -0.1)),
                                    ("radii", Value::List(vec![Value::F64(0.05), Value::F64(0.05)])),
                                ]),
                                rec(vec![
                                    ("center", v3(0.0, 0.42, 0.1)),
                                    ("radii", Value::List(vec![Value::F64(0.04), Value::F64(0.04)])),
                                ]),
                            ]),
                        )])]),
                    ),
                ]),
            ]),
        ),
    ])
}

fn rule(kind: &str, property: &str, value: f64) -> Value {
    rec(vec![
        ("kind", s(kind)),
        ("property", s(property)),
        ("value", Value::F64(value)),
    ])
}

fn run(inputs: &[Resource], a: &Args) -> Result<BTreeMap<String, Value>, String> {
    let cancel = AtomicBool::new(false);
    let limits = ToolLimits::default();
    let mut host = AssetHost {
        workspace: AssetWorkspace::default(),
        inputs,
        limits: &limits,
        cancel: &cancel,
        work: 0,
    };
    let res = host.living_world_anatomy_op(VERTEBRATE_OP, a)?;
    Ok(record(&res)?.clone())
}

#[test]
fn rule_pack_reproduces_legacy_hardcoded_packs() {
    for flora in [false, true] {
        let (domain, ctx) = if flora {
            (DOMAIN_FLORA, FLORA_CONTEXT_DID)
        } else {
            (DOMAIN_FAUNA, FAUNA_CONTEXT_DID)
        };
        let data = compile_rule_pack_q42("wallabia-bicolor", domain, ctx, &archetype_default_rules(flora))
            .unwrap();
        assert_eq!(data, compile_model_logic_q42_bytes("wallabia-bicolor", flora));
        let summary = validate_q42_logic(&data).unwrap();
        assert_eq!(summary.quin_count, 16);
        assert_eq!(summary.kind_counts, [4, 4, 4, 4]);
        assert_eq!(summary.domain_code, domain);
    }
}

#[test]
fn rule_pack_validation_rejects_corruption() {
    let good = compile_model_logic_q42_bytes("x", false);
    assert!(validate_q42_logic(&good).is_ok());

    let mut bad_magic = good.clone();
    bad_magic[0] = b'X';
    assert!(validate_q42_logic(&bad_magic).unwrap_err().contains("magic"));

    assert!(validate_q42_logic(&good[..good.len() - 1])
        .unwrap_err()
        .contains("length"));

    let mut bad_payload = good.clone();
    bad_payload[64] ^= 1;
    assert!(validate_q42_logic(&bad_payload).unwrap_err().contains("digest"));

    assert!(validate_q42_logic(&good[..10]).is_err());
}

#[test]
fn rules_value_parsing_fails_closed() {
    assert!(parse_rules_value(&Value::List(vec![])).is_err());
    assert!(parse_rules_value(&s("permit")).is_err());
    assert!(parse_rules_value(&Value::List(vec![rule("allow", "q42:p", 1.0)])).is_err());
    assert!(parse_rules_value(&Value::List(vec![rule("permit", "q42:p", -1.0)])).is_err());
    assert!(parse_rules_value(&Value::List(vec![rule("permit", "", 1.0)])).is_err());
    let extra = rec(vec![
        ("kind", s("permit")),
        ("property", s("q42:p")),
        ("value", Value::F64(1.0)),
        ("colour", s("red")),
    ]);
    assert!(parse_rules_value(&Value::List(vec![extra]))
        .unwrap_err()
        .contains("unknown field"));

    let forbid = parse_rules_value(&Value::List(vec![rule("forbid", "q42:p", 1.0)])).unwrap();
    assert!(forbid[0].defeater);
    assert_eq!(forbid[0].kind.opcode(), 0x12);
}

#[test]
fn vertebrate_op_compiles_vibescript_spec_with_rules() {
    let mut a = Args::new();
    a.insert("spec".into(), test_spec());
    a.insert(
        "rules".into(),
        Value::List(vec![
            rule("permit", "q42:safeCervicalNeckPitchRad", 0.6),
            rule("forbid", "q42:injuryThresholdAchillesStrain01", 1.0),
        ]),
    );
    let out = run(&[], &a).unwrap();
    assert_eq!(out.get("species_id"), Some(&s("test-species")));
    assert_eq!(out.get("spec_source"), Some(&s("vibescript")));
    assert_eq!(out.get("rules_source"), Some(&s("spec")));
    assert_eq!(out.get("rules_quin_count"), Some(&Value::I64(2)));
    assert_eq!(out.get("submesh_count"), Some(&Value::I64(3)));
}

#[test]
fn vertebrate_op_default_rules_use_spec_species_id() {
    let mut a = Args::new();
    a.insert("spec".into(), test_spec());
    let out = run(&[], &a).unwrap();
    assert_eq!(out.get("rules_source"), Some(&s("default")));
    assert_eq!(out.get("rules_quin_count"), Some(&Value::I64(16)));
    let expected = validate_q42_logic(&compile_model_logic_q42_bytes("test-species", false))
        .unwrap()
        .payload_digest;
    assert_eq!(
        out.get("rules_payload_digest"),
        Some(&s(&format!("{expected:015x}")))
    );

    let out = run(&[], &Args::new()).unwrap();
    assert_eq!(out.get("spec_source"), Some(&s("archetype")));
    assert_eq!(out.get("species_id"), Some(&s("macropod_saltatorial")));
}

#[test]
fn vertebrate_op_accepts_validated_q42_input() {
    let inputs = [Resource {
        key: "rules.q42".into(),
        kind: "q42".into(),
        bytes: compile_model_logic_q42_bytes("test-species", false),
    }];
    let mut a = Args::new();
    a.insert("spec".into(), test_spec());
    a.insert("rules".into(), s("rules.q42"));
    let out = run(&inputs, &a).unwrap();
    assert_eq!(out.get("rules_source"), Some(&s("input")));

    let mut corrupt = inputs[0].bytes.clone();
    corrupt[70] ^= 0xFF;
    let bad = [Resource {
        key: "rules.q42".into(),
        kind: "q42".into(),
        bytes: corrupt,
    }];
    assert!(run(&bad, &a).is_err());

    a.insert("rules".into(), s("missing.q42"));
    assert!(run(&inputs, &a).unwrap_err().contains("not found"));
}

#[test]
fn vertebrate_op_rejects_non_vibescript_specs() {
    let mut a = Args::new();
    a.insert("spec".into(), s("{\"schema\":\"qualia.portable-anatomy-scaffold/v1\"}"));
    assert!(run(&[], &a).unwrap_err().contains("VibeScript record"));

    let mut a = Args::new();
    a.insert("scaffold_input".into(), s("wallaby.json"));
    assert!(run(&[], &a).unwrap_err().contains("no longer accepted"));

    let mut a = Args::new();
    a.insert("spec".into(), rec(vec![("species_id", s("x"))]));
    assert!(run(&[], &a).unwrap_err().contains("schema"));

    let mut a = Args::new();
    a.insert("spec".into(), test_spec());
    a.insert("scaffold".into(), test_spec());
    assert!(run(&[], &a).unwrap_err().contains("not both"));
}

fn with_field(spec: Value, key: &str, v: Value) -> Value {
    let Value::Record(mut m) = spec else {
        unreachable!()
    };
    m.insert(key.into(), v);
    Value::Record(m)
}

fn range(min: f64, max: f64) -> Value {
    Value::List(vec![Value::F64(min), Value::F64(max)])
}

fn ankle_joint() -> Value {
    rec(vec![
        ("id", s("ankle_l")),
        (
            "axes",
            rec(vec![(
                "flexion",
                rec(vec![
                    ("healthy", range(-0.20, 0.95)),
                    ("injury", range(-0.45, 1.20)),
                ]),
            )]),
        ),
        (
            "when_injured",
            rec(vec![
                ("axes", rec(vec![("flexion", range(-0.05, 0.40))])),
                ("gait", s("asymmetric_limp")),
                ("guarding", Value::F64(0.9)),
            ]),
        ),
    ])
}

fn joints_json(joints: Vec<Value>) -> serde_json::Value {
    species_spec::vibe_value_to_json(&with_field(test_spec(), "joints", Value::List(joints)))
}

#[test]
fn joint_rom_parses_classifies_and_compiles() {
    use super::joint_rom::*;
    let spec = joints_json(vec![ankle_joint()]);
    let joints = parse_joint_rom(Some(&spec)).unwrap();
    assert_eq!(joints.len(), 1);
    assert_eq!(joints[0].joint, "left_ankle");
    let flex = joints[0].axis(Axis::Flexion).unwrap();

    assert_eq!(flex.classify(0.5, false), MotionZone::Healthy);
    assert_eq!(flex.classify(1.1, false), MotionZone::Strain);
    assert_eq!(flex.classify(1.3, false), MotionZone::Injury);
    assert_eq!(flex.classify(-0.5, false), MotionZone::Injury);
    assert_eq!(flex.classify(0.5, true), MotionZone::Strain);
    assert_eq!(flex.active_range(true).clamp(0.9), 0.40);
    assert_eq!(flex.healthy_basis, "model_assumption");
    assert!(flex.healthy_assertion_ids.is_empty());

    let rules = joint_rom_rules(&joints);
    assert_eq!(rules.len(), 7);
    assert!(rules
        .iter()
        .any(|r| r.property == "q42:rom:left_ankle:flexion:healthyNeg" && r.value == 0.20));
    assert!(rules
        .iter()
        .any(|r| r.property == "q42:rom:left_ankle:injuredGait:asymmetric_limp"
            && r.kind == RuleKind::Obligate));
}

#[test]
fn joint_rom_fails_closed() {
    use super::joint_rom::parse_joint_rom;
    let bad_joint = rec(vec![
        ("id", s("left_flipper")),
        ("axes", rec(vec![("flexion", rec(vec![("healthy", range(-0.1, 0.1)), ("injury", range(-0.2, 0.2))]))])),
    ]);
    assert!(parse_joint_rom(Some(&joints_json(vec![bad_joint])))
        .unwrap_err()
        .contains("unknown rig joint"));

    let not_enclosed = rec(vec![
        ("id", s("neck")),
        ("axes", rec(vec![("flexion", rec(vec![("healthy", range(-0.5, 0.5)), ("injury", range(-0.4, 0.6))]))])),
    ]);
    assert!(parse_joint_rom(Some(&joints_json(vec![not_enclosed])))
        .unwrap_err()
        .contains("enclose"));

    let no_neutral = rec(vec![
        ("id", s("neck")),
        ("axes", rec(vec![("flexion", rec(vec![("healthy", range(0.1, 0.5)), ("injury", range(-0.4, 0.6))]))])),
    ]);
    assert!(parse_joint_rom(Some(&joints_json(vec![no_neutral])))
        .unwrap_err()
        .contains("neutral"));

    assert!(parse_joint_rom(Some(&joints_json(vec![ankle_joint(), ankle_joint()])))
        .unwrap_err()
        .contains("twice"));
}

#[test]
fn vertebrate_op_appends_joint_rules() {
    let mut a = Args::new();
    a.insert(
        "spec".into(),
        with_field(test_spec(), "joints", Value::List(vec![ankle_joint()])),
    );
    let out = run(&[], &a).unwrap();
    assert_eq!(out.get("joint_rom_count"), Some(&Value::I64(1)));
    assert_eq!(out.get("rules_quin_count"), Some(&Value::I64(16 + 7)));

    let inputs = [Resource {
        key: "rules.q42".into(),
        kind: "q42".into(),
        bytes: compile_model_logic_q42_bytes("test-species", false),
    }];
    a.insert("rules".into(), s("rules.q42"));
    assert!(run(&inputs, &a).unwrap_err().contains("cannot be combined"));
}

#[test]
fn vertebrate_op_accepts_cbor_ld_spec_input() {
    let mut json = species_spec::vibe_value_to_json(&test_spec());
    json.as_object_mut().unwrap().insert(
        "@context".into(),
        serde_json::Value::String("hmc://schemas/webcivics/v1".into()),
    );
    let mut cbor = Vec::new();
    ciborium::ser::into_writer(&json, &mut cbor).unwrap();
    let inputs = [Resource {
        key: "wallaby.cborld".into(),
        kind: "cborld".into(),
        bytes: cbor,
    }];
    let mut a = Args::new();
    a.insert("spec".into(), s("wallaby.cborld"));
    let out = run(&inputs, &a).unwrap();
    assert_eq!(out.get("spec_source"), Some(&s("cbor-ld")));
    assert_eq!(out.get("species_id"), Some(&s("test-species")));
    assert_eq!(out.get("submesh_count"), Some(&Value::I64(3)));

    let json_text = [Resource {
        key: "wallaby.json".into(),
        kind: "json".into(),
        bytes: serde_json::to_vec(&json).unwrap(),
    }];
    a.insert("spec".into(), s("wallaby.json"));
    assert!(run(&json_text, &a).is_err());
}

fn range_assertion(id: &str, property: &str, min: f64, max: f64) -> serde_json::Value {
    serde_json::json!({
        "assertion": {
            "id": id,
            "subject_uri": "https://biodiversity.org.au/afd/taxa/Wallabia_bicolor",
            "statement_type": "observation",
            "assertion_status": "active",
            "verification_status": "proposed",
            "claim_summary": id,
            "author_did": "did:example:curator",
            "root_source_id": "src-1",
            "scope_domain": "species:wallabia-bicolor",
            "sensitivity": "Public",
            "asserted_at_unix": 1700000000
        },
        "evidence": {
            "property_iri": property,
            "basis": "measured",
            "method_iri": "urn:example:goniometer",
            "value": {
                "kind": "range",
                "min": min,
                "max": max,
                "unit_iri": "http://qudt.org/vocab/unit/RAD"
            }
        }
    })
}

#[test]
fn corpus_joint_ranges_cite_assertion_ids_and_unsourced_rules_stay_assumptions() {
    use super::corpus_values::joint_rom_from_corpus;
    use qualia_core_db::q42::species_corpus::SpeciesCorpus;

    let doc = serde_json::json!({
        "@context": "https://webizen.org/ns/qualia/species-corpus/v1",
        "@type": "QualiaSpeciesCorpus",
        "format": "qualia.q42.species-corpus.v1",
        "corpus_iri": "urn:corpus:wallabia-bicolor",
        "taxon_concept_iri": "https://biodiversity.org.au/afd/taxa/Wallabia_bicolor",
        "species_id": "wallabia-bicolor",
        "revision": "r1",
        "sources": [{
            "id": "src-1",
            "title": "Example treatment",
            "access_policy": "public-not-for-redistribution",
            "source_sha256": "ab".repeat(32),
            "mapping_version": "1"
        }],
        "assertions": [
            range_assertion("ea-healthy", "q42:rom:left_ankle:flexion:healthy", -0.20, 0.95),
            range_assertion("ea-injury", "q42:rom:left_ankle:flexion:injury", -0.45, 1.20)
        ],
        "gaps": []
    });
    let corpus = SpeciesCorpus::from_json_value(doc).unwrap();
    let joints = joint_rom_from_corpus(&corpus).unwrap();
    assert_eq!(joints.len(), 1);
    let flex = joints[0].axis(super::joint_rom::Axis::Flexion).unwrap();
    assert_eq!(flex.healthy_basis, "measured");
    assert_eq!(flex.healthy_assertion_ids, ["ea-healthy"]);
    assert_eq!(flex.injury_assertion_ids, ["ea-injury"]);
    let rules = super::joint_rom::joint_rom_rules(&joints);
    assert!(rules.iter().any(|r| {
        r.property == "q42:rom:left_ankle:flexion:healthyNeg"
            && r.basis == "measured"
            && r.assertion_ids == ["ea-healthy"]
    }));
    assert!(archetype_default_rules(false)
        .iter()
        .all(|r| r.basis == "model_assumption" && r.assertion_ids.is_empty()));

    let sealed = corpus.seal().unwrap();
    let mut spec = test_spec();
    if let Value::Record(ref mut m) = spec {
        m.insert("species_id".into(), s("wallabia-bicolor"));
        m.insert("corpus_iri".into(), s("urn:corpus:wallabia-bicolor"));
        m.insert("corpus_sha256".into(), s(&sealed.q42_sha256));
    }
    let inputs = [Resource {
        key: "wallaby.corpus.q42".into(),
        kind: "q42".into(),
        bytes: sealed.q42,
    }];
    let mut a = Args::new();
    a.insert("spec".into(), spec);
    a.insert("corpus".into(), s("wallaby.corpus.q42"));
    let out = run(&inputs, &a).unwrap();
    assert_eq!(out.get("joint_rom_count"), Some(&Value::I64(1)));
    assert_eq!(out.get("cited_rule_count"), Some(&Value::I64(4)));
    assert_eq!(out.get("corpus_iri"), Some(&s("urn:corpus:wallabia-bicolor")));

    let mut assemble = Args::new();
    assemble.insert("corpus".into(), s("wallaby.corpus.q42"));
    let cancel = AtomicBool::new(false);
    let limits = ToolLimits::default();
    let mut host = AssetHost {
        workspace: AssetWorkspace::default(),
        inputs: &inputs,
        limits: &limits,
        cancel: &cancel,
        work: 0,
    };
    let assembled = host
        .living_world_anatomy_op("qualia.species.assemble_corpus", &assemble)
        .unwrap();
    let assembled = record(&assembled).unwrap();
    assert_eq!(assembled.get("conflict_count"), Some(&Value::I64(0)));
    assert_eq!(assembled.get("assertion_count"), Some(&Value::I64(2)));
    assert_eq!(
        assembled.get("q42_sha256"),
        Some(&s(&sealed.q42_sha256))
    );
}

#[test]
fn measured_joint_without_an_assertion_id_is_rejected() {
    let measured = rec(vec![
        ("id", s("neck")),
        (
            "axes",
            rec(vec![(
                "flexion",
                rec(vec![
                    ("healthy", range(-0.2, 0.4)),
                    ("injury", range(-0.5, 0.6)),
                    ("basis", s("measured")),
                ]),
            )]),
        ),
    ]);
    let err = super::joint_rom::parse_joint_rom(Some(&joints_json(vec![measured]))).unwrap_err();
    assert!(err.contains("assertion id"), "{err}");
}
