//! Production section descriptors share the native SDFB decoder with QualiaPortal.

use crate::{
    artifact::{FieldLattice, HmcWorkspaceView},
    fields, integer, list, number, required, text, vector, Args, AssetHost, TypedArtifact, Value,
};
use qualia_core_db::{
    bundle::{BundleReadLimits, BundleReader},
    render::portal_load_10d::{
        decode_hmc_leaf_cutaway, decode_sdfb_cutaway, resolve_hmc_section_iris, PickSurfaceRegion,
        SectionLeafOccurrence, SectionPlane,
    },
};
use serde_json::{json, Value as Json};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

impl AssetHost<'_> {
    pub(crate) fn configure_interactive_cutaway(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "source",
                "plane_abcd",
                "slab_thickness_m",
                "assembly_key",
                "leaf_slots",
            ],
        )?;
        let source = required(args, "source")?;
        let authored_plane = vector::<4>(required(args, "plane_abcd")?)?;
        let thickness = number(args, "slab_thickness_m", 0.0)?;
        let plane = SectionPlane::new(authored_plane, thickness)?;
        let assembly_key = args.get("assembly_key").map(text).transpose()?;
        let slots = args.get("leaf_slots").map(list).transpose()?.unwrap_or(&[]);
        if slots.len() > self.limits.resources {
            return Err("Cutaway leaf mask exceeds the resource count limit".into());
        }
        let slots = slots
            .iter()
            .map(|slot| {
                u32::try_from(integer(slot, "leaf slot")?)
                    .map_err(|_| "Leaf slot exceeds u32".into())
            })
            .collect::<Result<Vec<_>, String>>()?;
        let source_bytes = cutaway_source(self, source)?.1.len();
        self.charge(source_bytes, 0)?;
        let (kind, bytes) = cutaway_source(self, source)?;
        let (vertex_count, groups, occurrences) = match (kind, assembly_key) {
            ("10d", None) if slots.is_empty() => {
                let (vertex_count, groups) = standalone(bytes, plane)?;
                (vertex_count, groups, Vec::new())
            }
            ("hmc", Some(key)) => assembly(bytes, key, plane, &slots, self.limits.resources)?,
            _ => return Err("Use a standalone SDFB 10D or an HMC with an assembly_key".into()),
        };
        let triangle_count = groups
            .iter()
            .map(|group| u64::from(group.index_count / 3))
            .sum::<u64>();
        let triangles = |region| {
            groups
                .iter()
                .filter(|group| group.region == region)
                .map(|group| u64::from(group.index_count / 3))
                .sum::<u64>()
        };
        let groups_json = groups.iter().map(PickGroup::json).collect::<Vec<_>>();
        let descriptor = json!({
            "schema": "qualia.sdfb-cutaway/v1",
            "source_kind": kind,
            "source_sha256": format!("{:x}", Sha256::digest(bytes)),
            "assembly_key": assembly_key,
            "plane_abcd": authored_plane,
            "slab_thickness_m": thickness,
            "leaf_slots": slots,
            "leaf_occurrences": occurrences,
            "vertex_count": vertex_count,
            "triangle_count": triangle_count,
            "cut_wall_triangles": triangles(PickSurfaceRegion::CutTissueWall),
            "lumen_triangles": triangles(PickSurfaceRegion::InternalCavityLumen),
            "pick_groups": groups_json,
        });
        let encoded = serde_json::to_vec(&descriptor).map_err(|error| error.to_string())?;
        Ok(Value::Record(BTreeMap::from([
            ("descriptor".into(), self.encode("json", encoded)?),
            ("vertex_count".into(), Value::U64(vertex_count)),
            ("triangle_count".into(), Value::U64(triangle_count)),
        ])))
    }
}

fn cutaway_source<'a>(
    host: &'a AssetHost<'_>,
    value: &Value,
) -> Result<(&'a str, &'a [u8]), String> {
    match host.workspace.typed(value) {
        Ok(TypedArtifact::HmcBundle(bundle)) => match bundle.as_ref() {
            HmcWorkspaceView::Sealed(bytes) => return Ok(("hmc", bytes)),
            HmcWorkspaceView::Staged(_) => return Err("Cutaway requires a sealed HMC".into()),
        },
        Ok(TypedArtifact::FieldLattice(field)) => {
            if let FieldLattice::TenD(view) = field.as_ref() {
                return Ok(("10d", view.as_10d_bytes()));
            }
        }
        _ => {}
    }
    let resource = crate::qualia_containers_governance_poet_ops::encoded_resource(host, value)?;
    Ok((&resource.kind, &resource.bytes))
}

struct PickGroup {
    slot: u32,
    first_index: u32,
    index_count: u32,
    region: PickSurfaceRegion,
    hash: u64,
    iri: String,
}

impl PickGroup {
    fn json(&self) -> Json {
        json!({
            "instance_slot": self.slot,
            "first_index": self.first_index,
            "index_count": self.index_count,
            "surface_region": self.region as u8,
            "semantic_id_hex": format!("{:016x}", self.hash),
            "iri": self.iri,
        })
    }
}

fn standalone(bytes: &[u8], plane: SectionPlane) -> Result<(u64, Vec<PickGroup>), String> {
    let surface = decode_sdfb_cutaway(bytes, plane)?;
    let groups = surface
        .ranges
        .iter()
        .zip(surface.regions)
        .map(|(range, region)| {
            let hash = range.semantic_id;
            let iri = surface
                .semantic_iris
                .get(&hash)
                .ok_or_else(|| format!("Unresolved SDFB pick {hash:#018x}"))?;
            Ok(PickGroup {
                slot: 0,
                first_index: range.first_index,
                index_count: range.index_count,
                region,
                hash,
                iri: iri.clone(),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok((surface.mesh.positions.len() as u64, groups))
}

fn assembly(
    bytes: &[u8],
    key: &str,
    plane: SectionPlane,
    slots: &[u32],
    max_entries: usize,
) -> Result<(u64, Vec<PickGroup>, Vec<SectionLeafOccurrence>), String> {
    let bundle = BundleReader::parse_with_limits(
        bytes,
        BundleReadLimits {
            file_bytes: bytes.len(),
            entries: max_entries,
            ..BundleReadLimits::default()
        },
    )
    .map_err(|error| error.to_string())?;
    let section = decode_hmc_leaf_cutaway(&bundle, key, plane, slots)?;
    let lexicon = resolve_hmc_section_iris(&bundle, &section)?;
    let groups = section
        .draw
        .descriptors
        .iter()
        .zip(section.pick_metadata)
        .map(
            |(&[first_index, index_count, instance, _], (slot, region))| {
                let instance = section
                    .draw
                    .instances
                    .get(instance as usize)
                    .ok_or("Section draw descriptor references a missing instance")?;
                let hash = instance.semantic_id();
                let iri = lexicon
                    .get(&hash)
                    .ok_or_else(|| format!("Unresolved HMC section pick {hash:#018x}"))?;
                Ok(PickGroup {
                    slot,
                    first_index,
                    index_count,
                    region,
                    hash,
                    iri: iri.clone(),
                })
            },
        )
        .collect::<Result<Vec<_>, String>>()?;
    Ok((
        u64::from(section.draw.vertex_count),
        groups,
        section.leaf_occurrences,
    ))
}
