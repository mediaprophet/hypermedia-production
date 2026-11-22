//! Ordered datum registration, nearest-surface deviations and guard-band receipts.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{
    add, bounded_number, cross, dot, fields, mesh_handle, mul, norm, point, required, sub, vector,
    Args, AssetHost, TypedArtifact, Value,
};
use crate::{list, qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph};
use chronicle_tabular_engine::{CellRef, Value as CellValue, Workbook};
use qualia_core_db::specialized_libs::computational_geometry::{
    point_triangle_distance_sq_3d, Point3,
};
use sha2::{Digest, Sha256};
use std::sync::Arc;

const CONTEXT: &str = "urn:qualia:contexts:authored-nominal-metrology-v1";
const MAX_PROBES: usize = 4096;
const MAX_COMPARISONS: usize = 20_000_000;

struct DatumFrame {
    origin: [f64; 3],
    basis: [[f64; 3]; 3],
}

impl DatumFrame {
    fn local(&self, point: [f64; 3]) -> [f64; 3] {
        self.basis.map(|axis| dot(sub(point, self.origin), axis))
    }

    fn world(&self, p: [f64; 3]) -> [f64; 3] {
        (0..3).fold(self.origin, |out, axis| {
            add(out, mul(self.basis[axis], p[axis]))
        })
    }
}

impl AssetHost<'_> {
    pub(crate) fn certify_cmm_metrology_gdt(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "nominal_mesh",
                "nominal_datum_m",
                "measured_datum_m",
                "probes_m",
                "tolerance_zone_m",
                "expanded_uncertainty_m",
            ],
        )?;
        let tolerance = bounded_number(args, "tolerance_zone_m", None, 1e-7, 0.1)?;
        let uncertainty = bounded_number(args, "expanded_uncertainty_m", None, 0.0, 0.1)?;
        let nominal_datum = datum_probes(required(args, "nominal_datum_m")?)?;
        let measured_datum = datum_probes(required(args, "measured_datum_m")?)?;
        let nominal = datum_frame(nominal_datum)?;
        let measured = datum_frame(measured_datum)?;
        let probes = list(required(args, "probes_m")?)?;
        if !(1..=MAX_PROBES).contains(&probes.len()) {
            return Err(format!(
                "Metrology requires 1 through {MAX_PROBES} measured probes"
            ));
        }
        let mesh = mesh_handle(self, required(args, "nominal_mesh")?)?;
        let triangle_count = mesh.surface.mesh.triangles.len();
        let comparisons = probes
            .len()
            .checked_mul(triangle_count)
            .filter(|&count| count <= MAX_COMPARISONS)
            .ok_or("Metrology exceeds the nearest-surface comparison budget")?;
        self.charge(
            comparisons * 40,
            probes.len() * 1024 + triangle_count * size_of::<[Point3; 3]>(),
        )?;
        let mut nominal_hash = Sha256::new();
        nominal_hash.update(b"qualia:nominal-metrology-surface:v1\0");
        nominal_hash.update((mesh.surface.mesh.positions.len() as u64).to_le_bytes());
        nominal_hash.update((triangle_count as u64).to_le_bytes());
        let mut triangles = Vec::with_capacity(triangle_count);
        for &v in &mesh.surface.mesh.positions {
            for c in v {
                nominal_hash.update(c.to_le_bytes());
            }
        }
        for &ids in &mesh.surface.mesh.triangles {
            for id in ids {
                nominal_hash.update(id.to_le_bytes());
            }
            let [a, b, c] = ids.map(|id| mesh.surface.mesh.positions[id as usize].map(f64::from));
            if norm(cross(sub(b, a), sub(c, a))) == 0.0 {
                return Err("The nominal surface contains a degenerate face".into());
            }
            triangles.push([point(a), point(b), point(c)]);
        }
        let nominal_iri = format!("urn:sha256:{:x}", nominal_hash.finalize());
        let mut measurement_hash = Sha256::new();
        measurement_hash.update(b"qualia:guard-band-probe-inputs:v1\0");
        measurement_hash.update(nominal_iri.as_bytes());
        measurement_hash.update((probes.len() as u64).to_le_bytes());
        measurement_hash.update(tolerance.to_le_bytes());
        measurement_hash.update(uncertainty.to_le_bytes());
        for v in nominal_datum.into_iter().chain(measured_datum).flatten() {
            measurement_hash.update(v.to_le_bytes());
        }

        let mut table = Workbook::new();
        let index = table.add_sheet("Probes");
        let sheet = table
            .sheet_mut(index)
            .ok_or("Metrology workbook could not open its probe sheet")?;
        for (col, title) in [
            "Probe",
            "Registered X (m)",
            "Registered Y (m)",
            "Registered Z (m)",
            "Nearest triangle index",
            "Deviation (m)",
            "Observed decision",
        ]
        .into_iter()
        .enumerate()
        {
            sheet.set_value(CellRef::new(0, col as u32), CellValue::text(title));
        }
        let mut max = 0.0_f64;
        let mut sum_sq = 0.0;
        let mut max_probe = 0;
        for (i, probe) in probes.iter().enumerate() {
            self.charge(0, 0)?;
            let p = measured_point(probe)?;
            for v in p {
                measurement_hash.update(v.to_le_bytes());
            }
            let registered = nominal.world(measured.local(p));
            let p = point(registered);
            let mut best = (f64::INFINITY, 0);
            for (triangle, &[a, b, c]) in triangles.iter().enumerate() {
                let distance = point_triangle_distance_sq_3d(p, a, b, c);
                if distance < best.0 {
                    best = (distance, triangle);
                }
            }
            if !best.0.is_finite() || best.0 < 0.0 {
                return Err(format!("Nominal mesh distance is undefined for probe {i}"));
            }
            let distance = best.0.sqrt();
            if distance > max {
                max = distance;
                max_probe = i;
            }
            sum_sq += best.0;
            let row = i as u32 + 1;
            for (column, value) in [
                i as f64,
                registered[0],
                registered[1],
                registered[2],
                best.1 as f64,
                distance,
            ]
            .into_iter()
            .enumerate()
            {
                sheet.set_value(CellRef::new(row, column as u32), CellValue::Number(value));
            }
            sheet.set_value(
                CellRef::new(row, 6),
                CellValue::text(decision(distance, uncertainty, tolerance)),
            );
        }
        let observed = decision(max, uncertainty, tolerance);
        let evidence = format!("urn:sha256:{:x}", measurement_hash.finalize());
        let mut receipt = ReceiptGraph::new(CONTEXT);
        receipt.fact(
            &evidence,
            "urn:qualia:metrology:nominalSurface",
            &nominal_iri,
        )?;
        receipt.fact(
            &evidence,
            "urn:qualia:metrology:observedDecision",
            &format!("urn:qualia:metrology:{observed}"),
        )?;
        receipt.fact(
            &evidence,
            "urn:qualia:metrology:measurementModel",
            "urn:qualia:metrology:ordered-datum-nearest-surface",
        )?;
        for (key, value) in [
            ("maxDeviationM", max),
            ("toleranceM", tolerance),
            ("uncertaintyM", uncertainty),
        ] {
            receipt.fact(
                &evidence,
                &format!("urn:qualia:metrology:{key}"),
                &format!("urn:qualia:si:metre:f64:{:016x}", value.to_bits()),
            )?;
        }
        let q42 = receipt.seal(self.limits.resource_bytes.min(1024 * 1024))?;
        let cube = self.admit_artifact(TypedArtifact::TabularCube(Arc::new(table)))?;
        let receipt = self.admit_encoded("q42", q42)?;
        Ok(Value::Record(
            [
                ("cube".into(), cube),
                ("receipt".into(), receipt),
                ("decision".into(), Value::String(observed.into())),
                ("max_deviation_m".into(), Value::F64(max)),
                ("max_deviation_probe".into(), Value::U64(max_probe as u64)),
                (
                    "rms_deviation_m".into(),
                    Value::F64((sum_sq / probes.len() as f64).sqrt()),
                ),
            ]
            .into(),
        ))
    }
}

fn measured_point(v: &Value) -> Result<[f64; 3], String> {
    let p = vector::<3>(v)?;
    if p.iter().any(|v| !(-100.0..=100.0).contains(v)) {
        return Err(
            "Datum and probe points must be within 100 metres of their input origin".into(),
        );
    }
    Ok(p)
}

fn datum_probes(value: &Value) -> Result<[[f64; 3]; 6], String> {
    let [a, b, c, d, e, f] = list(value)? else {
        return Err("A datum requires six ordered 3-2-1 locator points".into());
    };
    Ok([
        measured_point(a)?,
        measured_point(b)?,
        measured_point(c)?,
        measured_point(d)?,
        measured_point(e)?,
        measured_point(f)?,
    ])
}

fn datum_frame([a, b, c, d, e, f]: [[f64; 3]; 6]) -> Result<DatumFrame, String> {
    let normal = cross(sub(b, a), sub(c, a));
    let normal_length = norm(normal);
    let edge = sub(e, d);
    if normal_length < 1e-12 || norm(edge) < 1e-6 {
        return Err("The ordered primary or secondary datum references are degenerate".into());
    }
    let z = mul(normal, normal_length.recip());
    let tangent = sub(edge, mul(z, dot(edge, z)));
    let tangent_length = norm(tangent);
    if tangent_length < 1e-6 {
        return Err(
            "The two secondary locators do not constrain rotation about the primary normal".into(),
        );
    }
    let x = mul(tangent, tangent_length.recip());
    let y = cross(z, x);
    let origin = add(add(mul(x, dot(f, x)), mul(y, dot(d, y))), mul(z, dot(a, z)));
    Ok(DatumFrame {
        origin,
        basis: [x, y, z],
    })
}

fn decision(distance: f64, uncertainty: f64, tolerance: f64) -> &'static str {
    if distance + uncertainty <= tolerance {
        "within_guard_band_observed_probes"
    } else if distance - uncertainty > tolerance {
        "outside_guard_band_observed_probe"
    } else {
        "indeterminate_uncertainty_overlap"
    }
}
