use qualia_core_db::container_10d::{
    read_material_section_into, MaterialRecord, OpacityMode, SubmeshRange,
};
use qualia_core_db::render::botanical_anatomy::{
    build_scale3_floral_and_gumnut_dissection_10d, build_scale4_leaf_venation_and_histology_10d,
    HistologicalTissueLayer,
};

#[test]
fn sealed_leaf_keeps_opaque_lamina_and_translucent_tissue_draws_separate() {
    let organ = build_scale4_leaf_venation_and_histology_10d("corymbia-aparrerinja", 0.9).unwrap();
    let mut materials = vec![MaterialRecord::legacy_default(); 64];
    let mut ranges = vec![
        SubmeshRange {
            first_index: 0,
            index_count: 0,
            material_id: 0,
            semantic_id: 0
        };
        128
    ];
    read_material_section_into(
        &organ.container_bytes,
        organ.indices.len() as u32,
        &mut materials,
        &mut ranges,
    )
    .unwrap();
    let cuticle = organ
        .tissue_layer_slice(HistologicalTissueLayer::WaxyCuticle)
        .unwrap();
    let mut opaque = 0;
    let mut blended = 0;
    for range in ranges.iter().filter(|r| r.index_count > 0) {
        if range.first_index < cuticle.index_start
            || range.first_index >= cuticle.index_start + cuticle.index_count
        {
            continue;
        }
        let material = materials
            .iter()
            .find(|m| m.id == range.material_id)
            .unwrap();
        let indices = &organ.indices
            [range.first_index as usize..(range.first_index + range.index_count) as usize];
        let translucent = indices.iter().any(|i| organ.colors[*i as usize][3] < 255);
        if translucent {
            assert_eq!(material.opacity_mode, OpacityMode::Blend);
            assert!(
                !material.casts_shadow,
                "the sheet must not cast an opaque rectangle"
            );
            blended += range.index_count;
        } else {
            assert_eq!(material.opacity_mode, OpacityMode::Opaque);
            opaque += range.index_count;
        }
    }
    assert!(opaque > 0 && blended > 0);
    assert_eq!(
        opaque + blended,
        cuticle.index_count,
        "every cuticle triangle remains covered exactly once"
    );
    assert_eq!(
        ranges.iter().map(|r| r.index_count).sum::<u32>(),
        organ.indices.len() as u32
    );
}

#[test]
fn flower_capsule_layout_is_separate_in_front_and_side_projections() {
    let organ =
        build_scale3_floral_and_gumnut_dissection_10d("corymbia-aparrerinja", 0.75, 0.45).unwrap();
    let bounds = |layer| {
        let slice = organ.tissue_layer_slice(layer).unwrap();
        let points = &organ.vertices
            [slice.vertex_start as usize..(slice.vertex_start + slice.vertex_count) as usize];
        let min: [f32; 3] =
            std::array::from_fn(|k| points.iter().map(|p| p[k]).fold(f32::INFINITY, f32::min));
        let max: [f32; 3] = std::array::from_fn(|k| {
            points
                .iter()
                .map(|p| p[k])
                .fold(f32::NEG_INFINITY, f32::max)
        });
        (min, max)
    };
    let (flower_min, flower_max) = bounds(HistologicalTissueLayer::HypanthiumCalyxTube);
    let (capsule_min, capsule_max) =
        bounds(HistologicalTissueLayer::WoodyPericarpAndDehiscingValves);
    assert!(
        capsule_min[0] > flower_max[0],
        "front view must separate capsule from flower cups"
    );
    assert!(
        capsule_max[2] < flower_min[2],
        "side view must separate capsule from flower cups"
    );
}
