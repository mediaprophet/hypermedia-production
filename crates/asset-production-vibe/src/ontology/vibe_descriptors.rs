//! VibeScript ABI FuncDescriptor tables, sub-namespace descriptors, and HypermediaOntologyProvider.

use super::{MORPHISMS, ONTOLOGY_NQUINS};
use vibescript_abi::{
    BorrowedSliceView, CallerOutputSlice, FuncDescriptor, NQuin48, ProviderDescriptor, SemVer64,
    StableFuncId, SubNamespaceDescriptor, TargetProfileMask, VibeNamespaceProvider, VibeTrap,
};

static LOCUS_FUNCS: &[FuncDescriptor] = &[
    MORPHISMS[0].abi_descriptor,
    MORPHISMS[1].abi_descriptor,
    MORPHISMS[2].abi_descriptor,
    MORPHISMS[26].abi_descriptor,
    MORPHISMS[27].abi_descriptor,
    MORPHISMS[28].abi_descriptor,
    MORPHISMS[29].abi_descriptor,
    MORPHISMS[68].abi_descriptor,
    MORPHISMS[69].abi_descriptor,
    MORPHISMS[70].abi_descriptor,
    MORPHISMS[71].abi_descriptor,
    MORPHISMS[72].abi_descriptor,
    MORPHISMS[120].abi_descriptor,
    MORPHISMS[121].abi_descriptor,
    MORPHISMS[122].abi_descriptor,
];
static MANIFOLD_FUNCS: &[FuncDescriptor] = &[
    MORPHISMS[3].abi_descriptor,
    MORPHISMS[4].abi_descriptor,
    MORPHISMS[5].abi_descriptor,
    MORPHISMS[6].abi_descriptor,
    MORPHISMS[45].abi_descriptor,
    MORPHISMS[46].abi_descriptor,
    MORPHISMS[47].abi_descriptor,
    MORPHISMS[48].abi_descriptor,
    MORPHISMS[49].abi_descriptor,
    MORPHISMS[73].abi_descriptor,
    MORPHISMS[74].abi_descriptor,
    MORPHISMS[75].abi_descriptor,
    MORPHISMS[76].abi_descriptor,
    MORPHISMS[77].abi_descriptor,
    MORPHISMS[78].abi_descriptor,
    MORPHISMS[79].abi_descriptor,
    MORPHISMS[80].abi_descriptor,
    MORPHISMS[81].abi_descriptor,
    MORPHISMS[82].abi_descriptor,
    MORPHISMS[83].abi_descriptor,
    MORPHISMS[84].abi_descriptor,
    MORPHISMS[114].abi_descriptor,
    MORPHISMS[115].abi_descriptor,
    MORPHISMS[123].abi_descriptor,
    MORPHISMS[124].abi_descriptor,
    MORPHISMS[125].abi_descriptor,
    // The Qualia-owned species constructor declares ABI sub-id 2 (manifold).
    MORPHISMS[144].abi_descriptor,
];
static SURFACE_FUNCS: &[FuncDescriptor] = &[
    MORPHISMS[24].abi_descriptor,
    MORPHISMS[85].abi_descriptor,
    MORPHISMS[86].abi_descriptor,
    MORPHISMS[87].abi_descriptor,
    MORPHISMS[88].abi_descriptor,
    MORPHISMS[126].abi_descriptor,
    MORPHISMS[127].abi_descriptor,
    MORPHISMS[147].abi_descriptor,
];
static RADIANCE_FUNCS: &[FuncDescriptor] = &[
    MORPHISMS[7].abi_descriptor,
    MORPHISMS[8].abi_descriptor,
    MORPHISMS[9].abi_descriptor,
    MORPHISMS[10].abi_descriptor,
    MORPHISMS[11].abi_descriptor,
    MORPHISMS[12].abi_descriptor,
    MORPHISMS[13].abi_descriptor,
    MORPHISMS[18].abi_descriptor,
    MORPHISMS[19].abi_descriptor,
    MORPHISMS[20].abi_descriptor,
    MORPHISMS[21].abi_descriptor,
    MORPHISMS[22].abi_descriptor,
    MORPHISMS[23].abi_descriptor,
    MORPHISMS[38].abi_descriptor,
    MORPHISMS[39].abi_descriptor,
    MORPHISMS[40].abi_descriptor,
    MORPHISMS[41].abi_descriptor,
    MORPHISMS[42].abi_descriptor,
    MORPHISMS[43].abi_descriptor,
    MORPHISMS[44].abi_descriptor,
    MORPHISMS[89].abi_descriptor,
    MORPHISMS[90].abi_descriptor,
    MORPHISMS[91].abi_descriptor,
    MORPHISMS[92].abi_descriptor,
    MORPHISMS[93].abi_descriptor,
    MORPHISMS[128].abi_descriptor,
    MORPHISMS[129].abi_descriptor,
    MORPHISMS[130].abi_descriptor,
    MORPHISMS[145].abi_descriptor,
    MORPHISMS[146].abi_descriptor,
];
static GLYPH_FUNCS: &[FuncDescriptor] = &[
    MORPHISMS[30].abi_descriptor,
    MORPHISMS[31].abi_descriptor,
    MORPHISMS[32].abi_descriptor,
    MORPHISMS[33].abi_descriptor,
    MORPHISMS[94].abi_descriptor,
    MORPHISMS[95].abi_descriptor,
    MORPHISMS[131].abi_descriptor,
];
static LAYOUT_FUNCS: &[FuncDescriptor] = &[
    MORPHISMS[34].abi_descriptor,
    MORPHISMS[35].abi_descriptor,
    MORPHISMS[36].abi_descriptor,
    MORPHISMS[37].abi_descriptor,
    MORPHISMS[96].abi_descriptor,
    MORPHISMS[97].abi_descriptor,
    MORPHISMS[119].abi_descriptor,
    MORPHISMS[132].abi_descriptor,
    MORPHISMS[133].abi_descriptor,
    MORPHISMS[134].abi_descriptor,
];
static KINETICS_FUNCS: &[FuncDescriptor] = &[
    MORPHISMS[17].abi_descriptor,
    MORPHISMS[56].abi_descriptor,
    MORPHISMS[57].abi_descriptor,
    MORPHISMS[58].abi_descriptor,
    MORPHISMS[98].abi_descriptor,
    MORPHISMS[99].abi_descriptor,
    MORPHISMS[100].abi_descriptor,
    MORPHISMS[101].abi_descriptor,
    MORPHISMS[102].abi_descriptor,
    MORPHISMS[103].abi_descriptor,
    MORPHISMS[116].abi_descriptor,
    MORPHISMS[135].abi_descriptor,
    MORPHISMS[136].abi_descriptor,
    MORPHISMS[137].abi_descriptor,
    MORPHISMS[138].abi_descriptor,
];
static ACOUSTICS_FUNCS: &[FuncDescriptor] = &[
    MORPHISMS[14].abi_descriptor,
    MORPHISMS[15].abi_descriptor,
    MORPHISMS[16].abi_descriptor,
    MORPHISMS[59].abi_descriptor,
    MORPHISMS[60].abi_descriptor,
    MORPHISMS[61].abi_descriptor,
    MORPHISMS[104].abi_descriptor,
    MORPHISMS[105].abi_descriptor,
    MORPHISMS[139].abi_descriptor,
];
static ELECTROMAGNETICS_FUNCS: &[FuncDescriptor] = &[
    MORPHISMS[50].abi_descriptor,
    MORPHISMS[51].abi_descriptor,
    MORPHISMS[52].abi_descriptor,
    MORPHISMS[106].abi_descriptor,
    MORPHISMS[107].abi_descriptor,
    MORPHISMS[140].abi_descriptor,
];
static GEODESY_FUNCS: &[FuncDescriptor] = &[
    MORPHISMS[53].abi_descriptor,
    MORPHISMS[54].abi_descriptor,
    MORPHISMS[55].abi_descriptor,
    MORPHISMS[108].abi_descriptor,
    MORPHISMS[109].abi_descriptor,
    MORPHISMS[118].abi_descriptor,
    MORPHISMS[141].abi_descriptor,
];
static DISCOURSE_FUNCS: &[FuncDescriptor] = &[
    MORPHISMS[25].abi_descriptor,
    MORPHISMS[64].abi_descriptor,
    MORPHISMS[65].abi_descriptor,
    MORPHISMS[66].abi_descriptor,
    MORPHISMS[67].abi_descriptor,
    MORPHISMS[110].abi_descriptor,
    MORPHISMS[111].abi_descriptor,
    MORPHISMS[117].abi_descriptor,
    MORPHISMS[142].abi_descriptor,
];
static CHRONICLE_FUNCS: &[FuncDescriptor] = &[
    MORPHISMS[62].abi_descriptor,
    MORPHISMS[63].abi_descriptor,
    MORPHISMS[112].abi_descriptor,
    MORPHISMS[113].abi_descriptor,
    MORPHISMS[143].abi_descriptor,
];

static SUB_NAMESPACES: &[SubNamespaceDescriptor] = &[
    SubNamespaceDescriptor {
        name: "locus",
        sub_id: 1,
        functions: LOCUS_FUNCS,
    },
    SubNamespaceDescriptor {
        name: "manifold",
        sub_id: 2,
        functions: MANIFOLD_FUNCS,
    },
    SubNamespaceDescriptor {
        name: "surface",
        sub_id: 3,
        functions: SURFACE_FUNCS,
    },
    SubNamespaceDescriptor {
        name: "radiance",
        sub_id: 4,
        functions: RADIANCE_FUNCS,
    },
    SubNamespaceDescriptor {
        name: "glyph",
        sub_id: 5,
        functions: GLYPH_FUNCS,
    },
    SubNamespaceDescriptor {
        name: "layout",
        sub_id: 6,
        functions: LAYOUT_FUNCS,
    },
    SubNamespaceDescriptor {
        name: "kinetics",
        sub_id: 7,
        functions: KINETICS_FUNCS,
    },
    SubNamespaceDescriptor {
        name: "acoustics",
        sub_id: 8,
        functions: ACOUSTICS_FUNCS,
    },
    SubNamespaceDescriptor {
        name: "electromagnetics",
        sub_id: 9,
        functions: ELECTROMAGNETICS_FUNCS,
    },
    SubNamespaceDescriptor {
        name: "geodesy",
        sub_id: 10,
        functions: GEODESY_FUNCS,
    },
    SubNamespaceDescriptor {
        name: "discourse",
        sub_id: 11,
        functions: DISCOURSE_FUNCS,
    },
    SubNamespaceDescriptor {
        name: "chronicle",
        sub_id: 12,
        functions: CHRONICLE_FUNCS,
    },
];

pub static HYPERMEDIA_PROVIDER_DESCRIPTOR: ProviderDescriptor = ProviderDescriptor::const_new(
    "https://qualiadb.org/ext/hypermedia/production",
    "hypermedia",
    SemVer64::new(0, 2, 0, 0),
    SemVer64::new(0, 2, 0, 0),
    TargetProfileMask::ALL_CPU,
    SUB_NAMESPACES,
    256,
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
