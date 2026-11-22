//! VibeScript ABI FuncDescriptor tables, sub-namespace descriptors, and HypermediaOntologyProvider.

use super::ONTOLOGY_NQUINS;

#[cfg(test)]
use super::MORPHISMS;
use vibescript_abi::{
    BorrowedSliceView, CallerOutputSlice, FuncDescriptor, NQuin48, ProviderDescriptor, SemVer64,
    StableFuncId, SubNamespaceDescriptor, TargetProfileMask, VibeNamespaceProvider, VibeTrap,
};

const fn function_count(sub_id: u16) -> usize {
    let mut index = 0;
    let mut count = 0;
    while index < super::ALL_MORPHISMS.len() {
        if super::ALL_MORPHISMS[index].abi_descriptor.stable_id.sub_namespace_id == sub_id {
            count += 1;
        }
        index += 1;
    }
    count
}

const fn functions_in<const N: usize>(sub_id: u16) -> [FuncDescriptor; N] {
    let mut functions = [super::ALL_MORPHISMS[0].abi_descriptor; N];
    let mut index = 0;
    let mut count = 0;
    while index < super::ALL_MORPHISMS.len() {
        let descriptor = super::ALL_MORPHISMS[index].abi_descriptor;
        if descriptor.stable_id.sub_namespace_id == sub_id {
            functions[count] = descriptor;
            count += 1;
        }
        index += 1;
    }
    assert!(count == N);
    functions
}

static SUB_NAMESPACES: &[SubNamespaceDescriptor] = &[
    SubNamespaceDescriptor {
        name: "locus",
        sub_id: 1,
        functions: &functions_in::<{ function_count(1) }>(1),
    },
    SubNamespaceDescriptor {
        name: "manifold",
        sub_id: 2,
        functions: &functions_in::<{ function_count(2) }>(2),
    },
    SubNamespaceDescriptor {
        name: "surface",
        sub_id: 3,
        functions: &functions_in::<{ function_count(3) }>(3),
    },
    SubNamespaceDescriptor {
        name: "radiance",
        sub_id: 4,
        functions: &functions_in::<{ function_count(4) }>(4),
    },
    SubNamespaceDescriptor {
        name: "glyph",
        sub_id: 5,
        functions: &functions_in::<{ function_count(5) }>(5),
    },
    SubNamespaceDescriptor {
        name: "layout",
        sub_id: 6,
        functions: &functions_in::<{ function_count(6) }>(6),
    },
    SubNamespaceDescriptor {
        name: "kinetics",
        sub_id: 7,
        functions: &functions_in::<{ function_count(7) }>(7),
    },
    SubNamespaceDescriptor {
        name: "acoustics",
        sub_id: 8,
        functions: &functions_in::<{ function_count(8) }>(8),
    },
    SubNamespaceDescriptor {
        name: "electromagnetics",
        sub_id: 9,
        functions: &functions_in::<{ function_count(9) }>(9),
    },
    SubNamespaceDescriptor {
        name: "geodesy",
        sub_id: 10,
        functions: &functions_in::<{ function_count(10) }>(10),
    },
    SubNamespaceDescriptor {
        name: "discourse",
        sub_id: 11,
        functions: &functions_in::<{ function_count(11) }>(11),
    },
    SubNamespaceDescriptor {
        name: "chronicle",
        sub_id: 12,
        functions: &functions_in::<{ function_count(12) }>(12),
    },
    SubNamespaceDescriptor {
        name: "container",
        sub_id: 13,
        functions: &functions_in::<{ function_count(13) }>(13),
    },
    SubNamespaceDescriptor {
        name: "math",
        sub_id: 14,
        functions: &functions_in::<{ function_count(14) }>(14),
    },
    SubNamespaceDescriptor {
        name: "statistics",
        sub_id: 15,
        functions: &functions_in::<{ function_count(15) }>(15),
    },
    SubNamespaceDescriptor {
        name: "knowledge",
        sub_id: 16,
        functions: &functions_in::<{ function_count(16) }>(16),
    },
    SubNamespaceDescriptor {
        name: "geospatial",
        sub_id: 17,
        functions: &functions_in::<{ function_count(17) }>(17),
    },
    SubNamespaceDescriptor {
        name: "inference",
        sub_id: 18,
        functions: &functions_in::<{ function_count(18) }>(18),
    },
    SubNamespaceDescriptor {
        name: "epistemics",
        sub_id: 19,
        functions: &functions_in::<{ function_count(19) }>(19),
    },
    SubNamespaceDescriptor {
        name: "live",
        sub_id: 20,
        functions: &functions_in::<{ function_count(20) }>(20),
    },
    SubNamespaceDescriptor {
        name: "network",
        sub_id: 21,
        functions: &functions_in::<{ function_count(21) }>(21),
    },
    SubNamespaceDescriptor {
        name: "physics",
        sub_id: 22,
        functions: &functions_in::<{ function_count(22) }>(22),
    },
    SubNamespaceDescriptor {
        name: "topology",
        sub_id: 23,
        functions: &functions_in::<{ function_count(23) }>(23),
    },
    SubNamespaceDescriptor {
        name: "governance",
        sub_id: 24,
        functions: &functions_in::<{ function_count(24) }>(24),
    },
    SubNamespaceDescriptor {
        name: "cooperative",
        sub_id: 25,
        functions: &functions_in::<{ function_count(25) }>(25),
    },
    SubNamespaceDescriptor {
        name: "geometry",
        sub_id: 26,
        functions: &functions_in::<{ function_count(26) }>(26),
    },
    SubNamespaceDescriptor {
        name: "anatomy",
        sub_id: 27,
        functions: &functions_in::<{ function_count(27) }>(27),
    },
    SubNamespaceDescriptor {
        name: "twin",
        sub_id: 28,
        functions: &functions_in::<{ function_count(28) }>(28),
    },
    SubNamespaceDescriptor {
        name: "science",
        sub_id: 29,
        functions: &functions_in::<{ function_count(29) }>(29),
    },
    SubNamespaceDescriptor {
        name: "q42",
        sub_id: 30,
        functions: &functions_in::<{ function_count(30) }>(30),
    },
    SubNamespaceDescriptor {
        name: "solvers",
        sub_id: 31,
        functions: &functions_in::<{ function_count(31) }>(31),
    },
    SubNamespaceDescriptor {
        name: "economics",
        sub_id: 32,
        functions: &functions_in::<{ function_count(32) }>(32),
    },
    SubNamespaceDescriptor {
        name: "vision",
        sub_id: 33,
        functions: &functions_in::<{ function_count(33) }>(33),
    },
    SubNamespaceDescriptor {
        name: "wellfair",
        sub_id: 34,
        functions: &functions_in::<{ function_count(34) }>(34),
    },
    SubNamespaceDescriptor {
        name: "vibe",
        sub_id: 35,
        functions: &functions_in::<{ function_count(35) }>(35),
    },
    SubNamespaceDescriptor {
        name: "graph",
        sub_id: 36,
        functions: &functions_in::<{ function_count(36) }>(36),
    },
    SubNamespaceDescriptor {
        name: "sensing",
        sub_id: 37,
        functions: &functions_in::<{ function_count(37) }>(37),
    },
    SubNamespaceDescriptor {
        name: "vector",
        sub_id: 38,
        functions: &functions_in::<{ function_count(38) }>(38),
    },
    SubNamespaceDescriptor {
        name: "image",
        sub_id: 39,
        functions: &functions_in::<{ function_count(39) }>(39),
    },
    SubNamespaceDescriptor {
        name: "rights",
        sub_id: 40,
        functions: &functions_in::<{ function_count(40) }>(40),
    },
    SubNamespaceDescriptor {
        name: "cosmos",
        sub_id: 41,
        functions: &functions_in::<{ function_count(41) }>(41),
    },
    SubNamespaceDescriptor {
        name: "compute",
        sub_id: 42,
        functions: &functions_in::<{ function_count(42) }>(42),
    },
    SubNamespaceDescriptor {
        name: "linguistics",
        sub_id: 43,
        functions: &functions_in::<{ function_count(43) }>(43),
    },
    SubNamespaceDescriptor {
        name: "interchange",
        sub_id: 44,
        functions: &functions_in::<{ function_count(44) }>(44),
    },
    SubNamespaceDescriptor {
        name: "category",
        sub_id: 45,
        functions: &functions_in::<{ function_count(45) }>(45),
    },
    SubNamespaceDescriptor {
        name: "accessibility",
        sub_id: 46,
        functions: &functions_in::<{ function_count(46) }>(46),
    },
    SubNamespaceDescriptor {
        name: "quantum",
        sub_id: 47,
        functions: &functions_in::<{ function_count(47) }>(47),
    },
    SubNamespaceDescriptor {
        name: "narrative",
        sub_id: 48,
        functions: &functions_in::<{ function_count(48) }>(48),
    },
    SubNamespaceDescriptor {
        name: "logic",
        sub_id: 49,
        functions: &functions_in::<{ function_count(49) }>(49),
    },
    SubNamespaceDescriptor {
        name: "fabrication",
        sub_id: 50,
        functions: &functions_in::<{ function_count(50) }>(50),
    },
    SubNamespaceDescriptor {
        name: "privacy",
        sub_id: 51,
        functions: &functions_in::<{ function_count(51) }>(51),
    },
    SubNamespaceDescriptor {
        name: "infrastructure",
        sub_id: 52,
        functions: &functions_in::<{ function_count(52) }>(52),
    },
    SubNamespaceDescriptor {
        name: "control",
        sub_id: 53,
        functions: &functions_in::<{ function_count(53) }>(53),
    },
    SubNamespaceDescriptor {
        name: "environment",
        sub_id: 54,
        functions: &functions_in::<{ function_count(54) }>(54),
    },
    SubNamespaceDescriptor {
        name: "botany",
        sub_id: 55,
        functions: &functions_in::<{ function_count(55) }>(55),
    },
    SubNamespaceDescriptor {
        name: "anvil",
        sub_id: 56,
        functions: &functions_in::<{ function_count(56) }>(56),
    },
    SubNamespaceDescriptor {
        name: "storage",
        sub_id: 57,
        functions: &functions_in::<{ function_count(57) }>(57),
    },
    SubNamespaceDescriptor {
        name: "provenance",
        sub_id: 58,
        functions: &functions_in::<{ function_count(58) }>(58),
    },
    SubNamespaceDescriptor {
        name: "medical",
        sub_id: 59,
        functions: &functions_in::<{ function_count(59) }>(59),
    },
    SubNamespaceDescriptor {
        name: "matter",
        sub_id: 60,
        functions: &functions_in::<{ function_count(60) }>(60),
    },
    SubNamespaceDescriptor {
        name: "ai",
        sub_id: 61,
        functions: &functions_in::<{ function_count(61) }>(61),
    },
    SubNamespaceDescriptor {
        name: "crypto",
        sub_id: 62,
        functions: &functions_in::<{ function_count(62) }>(62),
    },
    SubNamespaceDescriptor {
        name: "chemistry",
        sub_id: 63,
        functions: &functions_in::<{ function_count(63) }>(63),
    },
    SubNamespaceDescriptor {
        name: "bio",
        sub_id: 64,
        functions: &functions_in::<{ function_count(64) }>(64),
    },
    SubNamespaceDescriptor {
        name: "biosphere",
        sub_id: 65,
        functions: &functions_in::<{ function_count(65) }>(65),
    },
    SubNamespaceDescriptor {
        name: "neuro",
        sub_id: 66,
        functions: &functions_in::<{ function_count(66) }>(66),
    },
    SubNamespaceDescriptor {
        name: "wellfare",
        sub_id: 67,
        functions: &functions_in::<{ function_count(67) }>(67),
    },
    SubNamespaceDescriptor {
        name: "poet",
        sub_id: 68,
        functions: &functions_in::<{ function_count(68) }>(68),
    },
    SubNamespaceDescriptor {
        name: "biology",
        sub_id: 69,
        functions: &functions_in::<{ function_count(69) }>(69),
    },
];

pub static HYPERMEDIA_PROVIDER_DESCRIPTOR: ProviderDescriptor = ProviderDescriptor::const_new(
    "https://qualiadb.org/ext/hypermedia/production",
    "hypermedia",
    SemVer64::new(0, 2, 0, 0),
    SemVer64::new(0, 2, 0, 0),
    TargetProfileMask::ALL_CPU,
    SUB_NAMESPACES,
    ONTOLOGY_NQUINS.len() as u32,
);

/// Zero-heap VibeScript namespace provider exposing the ontological contracts and `NQuin48` triples
/// for all Hypermedia asset production morphisms.
pub struct HypermediaOntologyProvider;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_catalog_morphism_has_exactly_one_metadata_descriptor() {
        let descriptors: Vec<_> = SUB_NAMESPACES.iter().flat_map(|ns| ns.functions).collect();
        assert_eq!(descriptors.len(), MORPHISMS.len());
        for morphism in MORPHISMS {
            assert_eq!(descriptors.iter().filter(|d| d.stable_id == morphism.abi_descriptor.stable_id).count(), 1,
                "descriptor must be bijective for {}", morphism.id);
        }
    }

    #[test]
    fn descriptor_contracts_and_arguments_expose_emf_v4_qac_v1_and_anatomy_parameters() {
        assert_eq!(crate::OPERATIONS.len(), MORPHISMS.len());

        let category_total: usize = crate::ontology::CATEGORIES
            .iter()
            .map(|c| {
                MORPHISMS
                    .iter()
                    .filter(|m| m.category == c.id)
                    .count()
            })
            .sum();
        assert_eq!(category_total, MORPHISMS.len());
        assert!(crate::ontology::resolve_morphism("qualia.species.assemble_corpus").is_some());

        let provider_funcs: Vec<&FuncDescriptor> = HYPERMEDIA_PROVIDER_DESCRIPTOR
            .sub_namespaces
            .iter()
            .flat_map(|ns| ns.functions)
            .collect();
        assert_eq!(provider_funcs.len(), MORPHISMS.len());

        let checks: &[(&str, &[&str], &[&str])] = &[
            (
                "hypermedia.electromagnetics.compile_emf",
                &["dipoles", "constitutive"],
                &["dipoles", "constitutive", "Q42EMF64 v4"],
            ),
            (
                "hypermedia.electromagnetics.verify_emf",
                &["emf"],
                &["format_profile", "dipole_records", "constitutive_records", "max_e_magnitude_v_per_m"],
            ),
            (
                "hypermedia.electromagnetics.render_spectral_slice",
                &["emf", "axis", "slice_index", "time_slice"],
                &["emf", "Q42EMF64"],
            ),
            (
                "hypermedia.electromagnetics.raymarch_vox0_dda",
                &["emf", "deflection_mass=0.0", "camera_position"],
                &["deflection_mass=0.0"],
            ),
            (
                "hypermedia.acoustics.synthesize_tone",
                &["emf"],
                &["emf"],
            ),
            (
                "hypermedia.acoustics.encode_aac",
                &["format", "qac", "species_id", "cue_profile"],
                &["qac"],
            ),
            (
                "hypermedia.acoustics.analyze_ebu_r128_camelot_daw",
                &["audio", "qac"],
                &["qac_v1_verification"],
            ),
            (
                "hypermedia.manifold.compile_botanical_organ_10d",
                &["stage", "emit_emf_qac"],
                &["emit_emf_qac", "emf_canonical_handle", "emf_profile_handle", "qac_canonical_handle"],
            ),
            (
                "hypermedia.manifold.compile_vertebrate_anatomy_10d",
                &["emit_emf_qac"],
                &["emit_emf_qac", "emf_canonical_handle", "emf_profile_handle", "qac_canonical_handle"],
            ),
            (
                "hypermedia.kinetics.rig_biomechanical_archetype",
                &["emit_emf_qac"],
                &["emit_emf_qac", "emf_canonical_handle", "emf_profile_handle", "qac_canonical_handle"],
            ),
        ];

        for (id, expected_args, expected_spec_needles) in checks {
            let morphism = crate::ontology::resolve_morphism(id)
                .unwrap_or_else(|| panic!("missing morphism {id}"));
            let matched = provider_funcs
                .iter()
                .find(|d| d.stable_id == morphism.abi_descriptor.stable_id)
                .unwrap_or_else(|| panic!("missing descriptor for {id}"));
            assert_eq!(matched.name, morphism.morphism);
            assert_eq!(
                matched.contract.schema_hash,
                morphism.abi_descriptor.contract.schema_hash,
                "descriptor schema_hash mismatch for {id}"
            );

            let args = crate::arguments(id);
            for needle in *expected_args {
                assert!(
                    args.contains(needle),
                    "arguments({id}) missing '{needle}': {args}"
                );
            }

            let blob = format!(
                "{} | {} | {} | {}",
                morphism.summary, morphism.domain, morphism.codomain, morphism.teleology
            );
            for needle in *expected_spec_needles {
                assert!(
                    blob.contains(needle),
                    "morphism spec for {id} missing '{needle}': {blob}"
                );
            }
        }

        let catalogue = crate::catalogue();
        let ops = catalogue["operations"]
            .as_array()
            .expect("catalogue operations array");
        assert_eq!(ops.len(), MORPHISMS.len());
        for (id, expected_args, _) in checks {
            let entry = ops
                .iter()
                .find(|op| op["id"] == *id)
                .unwrap_or_else(|| panic!("catalogue() missing {id}"));
            let args_json = entry["arguments"].as_str().unwrap_or("");
            for needle in *expected_args {
                assert!(
                    args_json.contains(needle),
                    "catalogue() arguments for {id} missing '{needle}': {args_json}"
                );
            }
        }
    }
}

impl VibeNamespaceProvider for HypermediaOntologyProvider {
    fn descriptor(&self) -> &'static ProviderDescriptor {
        &HYPERMEDIA_PROVIDER_DESCRIPTOR
    }

    fn static_ontology_nquins(&self) -> &'static [NQuin48] {
        ONTOLOGY_NQUINS
    }

    fn call(
        &self,
        stable_id: StableFuncId,
        _inputs: &[BorrowedSliceView<'_>],
        output: &mut CallerOutputSlice<'_>,
    ) -> Result<u64, VibeTrap> {
        let found = SUB_NAMESPACES
            .iter()
            .flat_map(|ns| ns.functions.iter())
            .find(|f| f.stable_id == stable_id)
            .ok_or(VibeTrap::FunctionNotFound)?;
        output.push_u64_le(found.contract.schema_hash)?;
        Ok(1)
    }
}
