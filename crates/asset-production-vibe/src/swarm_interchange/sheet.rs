//! Development of a constant-width single-axis sheet strip.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::*;
use std::fmt::Write;

impl AssetHost<'_> {
    pub(crate) fn unfold_sheet_and_cam_25d(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "mode",
                "flats_mm",
                "bends",
                "thickness_mm",
                "width_mm",
                "sheet_mm",
                "k_factor",
                "gap_mm",
                "count",
            ],
        )?;
        if text(required(a, "mode")?)? != "parallel_bend_dxf" {
            return Err("Expected parallel_bend_dxf with authored tangent-to-tangent flats".into());
        }
        let flats = list(required(a, "flats_mm")?)?;
        let bends = list(required(a, "bends")?)?;
        if !(1..=64).contains(&bends.len()) || flats.len() != bends.len() + 1 {
            return Err("Supply one more flat than bend, with 1 to 64 bends".into());
        }
        let sheet: [f64; 2] = vector(required(a, "sheet_mm")?)?;
        let thickness = num(required(a, "thickness_mm")?)?;
        let width = num(required(a, "width_mm")?)?;
        let k = number(a, "k_factor", 0.4)?;
        let gap = number(a, "gap_mm", 3.0)?;
        let count = a
            .get("count")
            .map(|v| integer(v, "count"))
            .transpose()?
            .unwrap_or(1);
        if sheet.iter().any(|v| !(1.0..=100_000.0).contains(v))
            || !(0.05..=100.0).contains(&thickness)
            || !(0.01..=100_000.0).contains(&width)
            || !(0.0..=0.5).contains(&k)
            || !(0.0..=1_000.0).contains(&gap)
            || !(1..=256).contains(&count)
        {
            return Err("Invalid sheet dimensions, gap, K-factor or copy count".into());
        }
        self.charge(
            count * (bends.len() + 8) * 64,
            count * (bends.len() + 8) * 500,
        )?;
        let flat = |v: &Value| {
            num(v).and_then(|n| {
                if (0.01..=100_000.0).contains(&n) {
                    Ok(n)
                } else {
                    Err("Invalid tangent-to-tangent flat length".into())
                }
            })
        };
        let mut length = flat(&flats[0])?;
        let mut centers = Vec::with_capacity(bends.len());
        let mut data = Vec::with_capacity(bends.len());
        for (i, value) in bends.iter().enumerate() {
            let bend = record(value)?;
            fields(bend, &["angle_deg", "inner_radius_mm"])?;
            let angle = num(required(bend, "angle_deg")?)?;
            let radius = num(required(bend, "inner_radius_mm")?)?;
            if !(0.01..=179.9).contains(&angle.abs()) || !(0.0..=100_000.0).contains(&radius) {
                return Err("A developed bend needs a finite nonzero rotation below 180° and nonnegative radius".into());
            }
            let allowance = angle.abs().to_radians() * (radius + k * thickness);
            if allowance < 1e-6 {
                return Err("Bend allowance is below the admitted resolution".into());
            }
            let center = length + allowance * 0.5;
            centers.push((center, angle > 0.0));
            data.push(Value::Record(Args::from([
                ("center_mm".into(), Value::F64(center)),
                ("allowance_mm".into(), Value::F64(allowance)),
                ("angle_deg".into(), Value::F64(angle)),
            ])));
            length += allowance + flat(&flats[i + 1])?;
        }
        let layout = [[length, width], [width, length]]
            .into_iter()
            .enumerate()
            .filter_map(|(turn, size)| {
                let capacity = |axis: usize| {
                    ((sheet[axis] - gap).max(0.0) / (size[axis] + gap)).floor() as usize
                };
                let across = capacity(0);
                (across.saturating_mul(capacity(1)) >= count).then_some((turn == 1, size, across))
            })
            .next()
            .ok_or("Developed blanks do not fit within the sheet and gap in either orientation")?;
        let (rotated, size, across) = layout;
        let mut dxf = String::from(
            "0\nSECTION\n2\nHEADER\n9\n$ACADVER\n1\nAC1015\n9\n$INSUNITS\n70\n4\n0\nENDSEC\n0\nSECTION\n2\nENTITIES\n",
        );
        let mut profiles = Vec::with_capacity(count);
        for copy in 0..count {
            let x = gap + (copy % across) as f64 * (size[0] + gap);
            let y = gap + (copy / across) as f64 * (size[1] + gap);
            let put = |u: f64, v: f64| {
                if rotated {
                    [x + width - v, y + u]
                } else {
                    [x + u, y + v]
                }
            };
            let corners = [
                put(0.0, 0.0),
                put(length, 0.0),
                put(length, width),
                put(0.0, width),
            ];
            dxf.push_str(
                "0\nLWPOLYLINE\n100\nAcDbEntity\n8\nOUTLINE\n100\nAcDbPolyline\n90\n4\n70\n1\n",
            );
            for [u, v] in corners {
                writeln!(dxf, "10\n{u:.9}\n20\n{v:.9}").map_err(|e| e.to_string())?;
            }
            for &(at, positive) in &centers {
                let [u, v] = put(at, 0.0);
                let [end_u, end_v] = put(at, width);
                let layer = if positive { "BEND_UP" } else { "BEND_DOWN" };
                writeln!(dxf, "0\nLINE\n100\nAcDbEntity\n8\n{layer}\n100\nAcDbLine\n10\n{u:.9}\n20\n{v:.9}\n30\n0\n11\n{end_u:.9}\n21\n{end_v:.9}\n31\n0").map_err(|e| e.to_string())?;
            }
            let outline = corners
                .into_iter()
                .chain(corners.first().copied())
                .map(|p| p.map(|v| v * 0.001))
                .collect();
            profiles.push(
                self.workspace
                    .insert(Artifact::Profile(outline), self.limits)?,
            );
        }
        dxf.push_str("0\nENDSEC\n0\nEOF\n");
        let resource = self.encode("dxf", dxf.into_bytes())?;
        Ok(Value::Record(Args::from([
            ("profiles".into(), Value::List(profiles)),
            ("dxf".into(), resource),
            ("developed_length_mm".into(), Value::F64(length)),
            ("bend_schedule".into(), Value::List(data)),
            ("rotated".into(), Value::Bool(rotated)),
        ])))
    }
}
