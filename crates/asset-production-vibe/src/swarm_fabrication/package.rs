//! Native binary STL and 3MF Core export with detached evidence.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{
    fields, mesh_handle, required, text, Args, AssetHost, Value, MAX_MESH_FACES, MAX_MESH_VERTICES,
};
use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use crate::{list, Resource};
use qualia_core_db::{
    container_10d::decode_symbolic_rdf_cbor_ld,
    q42_volume::{
        OpfsSliceRangeSource, Q42RangeVolume, WasmSliceQ42Reader, FLAG_SANCTUARY, FLAG_VOLUME_ROOT,
    },
    specialized_libs::computer_vision::spatial::{
        export_3mf::mesh_ir_to_3mf, export_stl::mesh_ir_to_stl_binary, geometry_ir::MeshIR,
    },
};
use sha2::{Digest, Sha256};

const CONTEXT: &str = "urn:qualia:contexts:print-package-v1";
const PROVENANCE_LIMIT: usize = 4 * 1024 * 1024;

impl AssetHost<'_> {
    pub(crate) fn export_print_package(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "parts",
                "format",
                "license_q42",
                "license_sha256",
                "cbld_provenance",
            ],
        )?;
        let format = text(required(args, "format")?)?;
        if !matches!(format, "3mf_core" | "stl_binary") {
            return Err("Print package format must be 3mf_core or stl_binary".into());
        }
        let parts = list(required(args, "parts")?)?;
        if !(1..=128).contains(&parts.len()) {
            return Err("Print package requires 1 through 128 placed mesh parts".into());
        }
        let license_iri = if let Some(license) = args.get("license_q42") {
            let resource = evidence(self, license, "q42")?;
            let digest = format!("{:x}", Sha256::digest(&resource.bytes));
            if text(required(args, "license_sha256")?)? != digest {
                return Err("License Q42 digest does not match retained bytes".into());
            }
            let reader = WasmSliceQ42Reader::open(&resource.bytes).map_err(|e| e.to_string())?;
            let volume = Q42RangeVolume::open(OpfsSliceRangeSource::new(&resource.bytes))
                .map_err(|e| e.to_string())?;
            if !reader.is_unified_volume()
                || volume.header().flags & (FLAG_VOLUME_ROOT | FLAG_SANCTUARY) != 0
            {
                return Err(
                    "Print package evidence needs a self-contained, non-Sanctuary Q42 volume"
                        .into(),
                );
            }
            Some(format!("urn:sha256:{digest}"))
        } else {
            if args.contains_key("license_sha256") {
                return Err("license_sha256 requires license_q42".into());
            }
            None
        };
        let provenance_iri = if let Some(provenance) = args.get("cbld_provenance") {
            let bytes = &evidence(self, provenance, "cbld")?.bytes;
            decode_symbolic_rdf_cbor_ld(bytes).map_err(|e| e.to_string())?;
            Some(format!("urn:sha256:{:x}", Sha256::digest(bytes)))
        } else {
            None
        };

        let meshes = parts
            .iter()
            .map(|v| mesh_handle(self, v))
            .collect::<Result<Vec<_>, _>>()?;
        let vertices = meshes
            .iter()
            .map(|v| v.surface.mesh.positions.len())
            .sum::<usize>();
        let triangles = meshes
            .iter()
            .map(|v| v.surface.mesh.triangles.len())
            .sum::<usize>();
        if vertices > MAX_MESH_VERTICES || triangles > MAX_MESH_FACES {
            return Err(
                "Combined print package geometry exceeds the admitted geometry budget".into(),
            );
        }
        let bounded_bytes = if format == "3mf_core" {
            // The native writer emits one finite-f32 vertex and unsigned-index triangle per line.
            8192 + vertices * 240 + triangles * 96
        } else {
            84 + 50 * triangles
        };
        let limit = self.limits.resource_bytes.min(128 * 1024 * 1024);
        let bytes = bounded_bytes.min(limit);
        self.charge(
            vertices * 12 + triangles * 48 + bytes,
            3 * bounded_bytes + 12 * vertices + 12 * triangles,
        )?;

        let mut mesh_ir = MeshIR::empty();
        mesh_ir.positions.reserve_exact(vertices);
        mesh_ir.indices.reserve_exact(triangles * 3);
        let mut part_iris = Vec::with_capacity(parts.len());
        for mesh in meshes {
            let mesh = &mesh.surface.mesh;
            let mut hash = Sha256::new();
            hash.update(b"qualia:print-source-metre-mesh:v1\0");
            hash.update((mesh.positions.len() as u64).to_le_bytes());
            hash.update((mesh.triangles.len() as u64).to_le_bytes());
            let offset = mesh_ir.positions.len() as u32;
            for position in &mesh.positions {
                for axis in position {
                    hash.update(axis.to_le_bytes());
                }
                mesh_ir.positions.push(position.map(|axis| axis * 1000.0));
            }
            for triangle in &mesh.triangles {
                for index in triangle {
                    hash.update(index.to_le_bytes());
                    mesh_ir.indices.push(offset + index);
                }
            }
            part_iris.push(format!("urn:sha256:{:x}", hash.finalize()));
        }
        mesh_ir.recompute_bounds_and_hash();
        let mut output = vec![0; bytes];
        let used = if format == "3mf_core" {
            mesh_ir_to_3mf(&mesh_ir, &mut output)
        } else {
            mesh_ir_to_stl_binary(&mesh_ir, &mut output)
        }
        .map_err(|error| format!("Native print package writer: {error:?}"))?;
        output.truncate(used);

        let output_iri = format!("urn:sha256:{:x}", Sha256::digest(&output));
        let mut receipt = ReceiptGraph::new(CONTEXT);
        let kind = if format == "3mf_core" { "3mf" } else { "stl" };
        receipt.fact(
            &output_iri,
            "urn:qualia:print:encoding",
            &format!("urn:qualia:print:{format}"),
        )?;
        receipt.fact(
            &output_iri,
            "urn:qualia:print:coordinateUnit",
            "http://qudt.org/vocab/unit/MilliM",
        )?;
        for (index, part) in part_iris.iter().enumerate() {
            let occurrence = format!("{output_iri}:part:{index}");
            receipt.fact(&output_iri, "urn:qualia:print:placedPart", &occurrence)?;
            receipt.fact(
                &occurrence,
                "http://www.w3.org/ns/prov#wasDerivedFrom",
                part,
            )?;
        }
        if let Some(iri) = license_iri {
            receipt.fact(
                &output_iri,
                "urn:qualia:print:providedLicenseEvidence",
                &iri,
            )?;
        }
        if let Some(iri) = provenance_iri {
            receipt.fact(&output_iri, "urn:qualia:print:providedCbldEvidence", &iri)?;
        }
        let q42 = receipt.seal(self.limits.resource_bytes.min(PROVENANCE_LIMIT))?;
        let asset = self.admit_encoded(kind, output)?;
        let receipt = self.admit_encoded("q42", q42)?;
        Ok(Value::Record(
            [
                ("asset".into(), asset),
                ("receipt".into(), receipt),
                ("output_iri".into(), Value::String(output_iri)),
                ("part_count".into(), Value::U64(parts.len() as u64)),
            ]
            .into(),
        ))
    }
}

fn evidence<'a>(
    host: &'a AssetHost<'_>,
    value: &Value,
    kind: &str,
) -> Result<&'a Resource, String> {
    let resource = match value {
        Value::String(key) => host
            .inputs
            .iter()
            .find(|v| v.key == *key)
            .ok_or_else(|| format!("Unknown retained evidence key {key}"))?,
        _ => host.workspace.encoded(value)?,
    };
    if resource.kind != kind || resource.bytes.is_empty() || resource.bytes.len() > PROVENANCE_LIMIT
    {
        return Err(format!("Expected nonempty {kind} evidence under 4 MiB"));
    }
    Ok(resource)
}
