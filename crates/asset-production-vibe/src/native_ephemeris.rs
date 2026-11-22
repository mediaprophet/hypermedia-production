//! Earth-fixed trajectory sampling through the native secular J2 propagator.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::sync::Arc;

use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use crate::*;
use chronicle_tabular_engine::{CellRef, Value as CellValue, Workbook};
use qualia_core_db::domains::geospatial::geodesy::ephemeris_orbit::{
    KeplerElements, propagate_orbit_j2,
};
use sha2::{Digest, Sha256};

const FRAME: &str = "http://www.opengis.net/def/crs/EPSG/0/4978";

impl AssetHost<'_> {
    pub(crate) fn ephemeris_propagate(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["kepler", "sample_unix_s", "reference_frame"])?;
        if args.get("reference_frame").map(text).transpose()?.unwrap_or("ecef") != "ecef" {
            return Err("Earth J2 propagation supports ECEF output".into());
        }
        let input = record(required(args, "kepler")?)?;
        fields(input, &["semi_major_axis_m", "eccentricity", "inclination_deg", "raan_deg",
            "arg_perigee_deg", "mean_anomaly_epoch_deg", "epoch_unix_s"])?;
        let value = |key| num(required(input, key)?);
        let elements = KeplerElements {
            semi_major_axis_m: value("semi_major_axis_m")?,
            eccentricity: value("eccentricity")?,
            inclination_deg: value("inclination_deg")?,
            raan_deg: value("raan_deg")?,
            arg_perigee_deg: value("arg_perigee_deg")?,
            mean_anomaly_epoch_deg: value("mean_anomaly_epoch_deg")?,
            epoch_unix_s: value("epoch_unix_s")?,
        };
        let perigee = elements.semi_major_axis_m * (1.0 - elements.eccentricity);
        if !(0.0..=0.95).contains(&elements.eccentricity)
            || !(0.0..=180.0).contains(&elements.inclination_deg)
            || !(6_388_137.0..=1e9).contains(&perigee)
            || elements.semi_major_axis_m > 1e9
            || elements.epoch_unix_s.abs() > 1e10
            || [elements.raan_deg, elements.arg_perigee_deg, elements.mean_anomaly_epoch_deg]
                .iter().any(|angle| angle.abs() > 1e6)
        {
            return Err("Kepler data falls outside the admitted bound Earth orbit domain".into());
        }
        let times = list(required(args, "sample_unix_s")?)?;
        if !(1..=4096).contains(&times.len()) {
            return Err("Earth orbit trajectory needs 1 to 4096 requested sample times".into());
        }
        let times = times.iter().map(num).collect::<Result<Vec<_>, _>>()?;
        if times.windows(2).any(|pair| pair[0] >= pair[1])
            || times.iter().any(|v| (v - elements.epoch_unix_s).abs() > 31_557_600.0)
        {
            return Err("Samples require strictly increasing times within one year of the source epoch".into());
        }
        self.charge(times.len() * 2048, times.len() * 2048)?;
        let mut trajectory = Workbook::new();
        let sheet = trajectory.sheet_mut(0).ok_or("Could not initialize trajectory worksheet")?;
        sheet.name = "Earth fixed trajectory".into();
        let columns = ["unix_s", "x_m", "y_m", "z_m", "vx_m_s", "vy_m_s", "vz_m_s",
            "latitude_deg", "longitude_deg", "altitude_m"];
        for (column, name) in columns.into_iter().enumerate() {
            sheet.set_value(CellRef::new(0, column as u32), CellValue::text(name));
        }
        let mut identity = Sha256::new();
        identity.update(b"qualia:secular-earth-j2-with-unix-clock:v1");
        for scalar in [elements.semi_major_axis_m, elements.eccentricity, elements.inclination_deg,
            elements.raan_deg, elements.arg_perigee_deg, elements.mean_anomaly_epoch_deg, elements.epoch_unix_s] {
            identity.update(scalar.to_bits().to_le_bytes());
        }
        for (index, unix_s) in times.iter().copied().enumerate() {
            let state = propagate_orbit_j2(&elements, unix_s);
            let [x,y,z] = state.pos_ecef_m;
            let [vx,vy,vz] = state.vel_ecef_m_s;
            let values = [unix_s, x, y, z, vx, vy, vz, state.subpoint_lat_deg, state.subpoint_lon_deg, state.altitude_m];
            if values.iter().any(|v| !v.is_finite()) {
                return Err("Earth orbit solver returned a nonfinite state".into());
            }
            identity.update(unix_s.to_bits().to_le_bytes());
            for (column, v) in values.into_iter().enumerate() {
                sheet.set_value(CellRef::new((index + 1) as u32, column as u32), CellValue::Number(v));
            }
        }
        let iri = format!("urn:qualia:trajectory:{:x}", identity.finalize());
        let mut receipt = ReceiptGraph::new("urn:qualia:trajectory:earth-j2");
        receipt.fact(&iri, "urn:qualia:ephemeris:referenceFrame", FRAME)?;
        receipt.fact(&iri, "urn:qualia:ephemeris:propagator", "urn:qualia:ephemeris:earth-j2-secular")?;
        receipt.fact(&iri, "urn:qualia:ephemeris:clock", "urn:qualia:clock:unix-earth-rotation-approximation")?;
        let receipt = self.encode("q42", receipt.seal(self.limits.resource_bytes)?)?;
        let trajectory = self.admit_artifact(TypedArtifact::TabularCube(Arc::new(trajectory)))?;
        Ok(Value::Record(Args::from([
            ("trajectory".into(), trajectory), ("trajectory_iri".into(), Value::String(iri)),
            ("reference_frame".into(), Value::String(FRAME.into())), ("receipt".into(), receipt),
        ])))
    }
}
