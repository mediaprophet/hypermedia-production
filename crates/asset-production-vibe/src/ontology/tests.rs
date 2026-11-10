use super::*;
use vibescript_abi::VibeNamespaceProvider;

#[test]
fn all_ontology_nquins_verify_five_field_parity_and_spans() {
    let provider = HypermediaOntologyProvider;
    let desc = provider.descriptor();
    assert!(desc.versions.is_engine_compatible());
    let quins = provider.static_ontology_nquins();
    assert_eq!(MORPHISMS.len(), 145);
    assert_eq!(crate::OPERATIONS.len(), 145);
    assert_eq!(quins.len(), 435);
    assert_eq!(quins.len(), MORPHISMS.len() * 3);
    for (idx, q) in quins.iter().enumerate() {
        assert!(q.verify_parity(), "NQuin48 at index {idx} failed parity");
        let expected = q.0[..5].iter().fold(0u64, |acc, w| acc ^ *w);
        assert_eq!(q.parity(), expected);
    }
    for ns in desc.sub_namespaces {
        for f in ns.functions {
            assert_eq!(f.stable_id.sub_namespace_id, ns.sub_id);
            assert!(
                f.contract.validate_spans(quins.len()),
                "Invalid NQuin span on {}",
                f.name
            );
        }
    }
}

#[test]
fn zero_craft_names_in_canonical_categories_or_morphism_ids() {
    for cat in CATEGORIES {
        assert!(!cat.id.contains("craft"));
    }
    for m in MORPHISMS {
        assert!(
            !m.id.contains("craft"),
            "Canonical morphism ID {} must not contain 'craft'",
            m.id
        );
        assert_eq!(canonical_morphism_id(m.id), Some(m.id));
        for alias in m.aliases {
            assert_eq!(canonical_morphism_id(alias), Some(m.id));
        }
    }
}
