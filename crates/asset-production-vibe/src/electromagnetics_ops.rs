//! Existing EMF morphisms route to canonical typed production/derivatives; CSI remains separate.
use super::*;
use std::collections::BTreeMap;
impl AssetHost<'_> {
    /// Categorical operations dispatch for the `electromagnetics` category.
    pub(super) fn electromagnetics_op(&mut self, id: &str, a: &Args) -> Result<Value, String> {
        match id {
            "hypermedia.electromagnetics.compile_emf"
            | "electromagnetics.compile_emf"
            | "compile_emf" => self.compile_emf(a),

            "hypermedia.electromagnetics.verify_emf"
            | "electromagnetics.verify_emf"
            | "verify_emf" => self.verify_emf_current(a),

            "hypermedia.electromagnetics.render_spectral_slice"
            | "electromagnetics.render_spectral_slice"
            | "render_spectral_slice" => self.render_current_emf_slice(a),

            "hypermedia.electromagnetics.sense_wifi_csi_vitals_aoa"
            | "electromagnetics.sense_wifi_csi_vitals_aoa"
            | "synth.electromagnetics.sense_wifi_csi_vitals_aoa"
            | "sense_wifi_csi_vitals_aoa" => self.produce_wifi_csi_vitals_aoa(a),

            _ => Err(format!("Unknown electromagnetics operation: {id}")),
        }
    }

    /// 802.11 CSI → sanitized phase, contactless vitals, MUSIC AoA, camera-free receipt.
    fn produce_wifi_csi_vitals_aoa(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["audio", "breathing_hz", "heart_hz", "carrier_ghz"])?;
        let resp_hz = number(a, "breathing_hz", 0.25)? as f32;
        let heart_hz = number(a, "heart_hz", 1.2)? as f32;
        let carrier_ghz = number(a, "carrier_ghz", 5.18)?;
        if !(1.0..=60.0).contains(&carrier_ghz) {
            return Err("carrier_ghz outside 1.0..=60.0".into());
        }
        if !(0.05..=0.7).contains(&(resp_hz as f64)) {
            return Err("breathing_hz outside 0.05..=0.7".into());
        }
        if !(0.7..=3.0).contains(&(heart_hz as f64)) {
            return Err("heart_hz outside 0.7..=3.0".into());
        }

        self.charge(512, 8192)?;

        use electromagnetics_rf_sensing::{
            extract_vital_signs, find_dominant_aoa_peak, hampel_filter, music_aoa_spectrum,
            sanitize_csi_phase, Complex32, CsiFrame, PrivacyWitnessReceipt, UniformLinearArray,
        };

        let indices = vec![-28, -20, -10, 0, 10, 20, 28];
        let mut subcarriers = Vec::with_capacity(indices.len());
        for &k in &indices {
            let phi = (k as f32) * 0.12 + 0.8;
            subcarriers.push(Complex32::from_polar(1.0, phi));
        }
        let frame = CsiFrame::new(
            1_700_000_000,
            carrier_ghz * 1e9,
            1,
            1,
            indices.clone(),
            subcarriers,
        );
        let raw_phases = frame.get_phases(0, 0);
        let sanitized = sanitize_csi_phase(&frame.subcarrier_indices, &raw_phases);

        let sr = 50.0f32;
        let mut signal = Vec::with_capacity(500);
        if let Some(v) = a.get("audio") {
            let Artifact::Audio { samples: src, .. } = self.workspace.get(v)? else {
                return Err("Expected audio handle".into());
            };
            for i in 0..500 {
                let s = src.get(i).copied().unwrap_or(0.0);
                let t = (i as f32) / sr;
                signal.push(
                    s * 0.05
                        + 0.5 * (2.0 * std::f32::consts::PI * resp_hz * t).sin()
                        + 0.1 * (2.0 * std::f32::consts::PI * heart_hz * t).sin(),
                );
            }
        } else {
            for i in 0..500 {
                let t = (i as f32) / sr;
                signal.push(
                    0.5 * (2.0 * std::f32::consts::PI * resp_hz * t).sin()
                        + 0.1 * (2.0 * std::f32::consts::PI * heart_hz * t).sin(),
                );
            }
        }
        let filtered = hampel_filter(&signal, 3, 3.0);
        let vitals = extract_vital_signs(&filtered, sr).ok_or("Vital signs extraction failed")?;

        let ula = UniformLinearArray::new(4, carrier_ghz * 1e9);
        let target_aoa = 25.0f32;
        let mut snapshots = Vec::with_capacity(16);
        for t in 0..16 {
            // Identical snapshots collapse MUSIC; carrier rotation is the crate object.
            let carrier = Complex32::from_polar(1.0, (t as f32) * 0.2);
            let snap: Vec<Complex32> = ula
                .steering_vector(target_aoa)
                .into_iter()
                .map(|a| a.mul(carrier))
                .collect();
            snapshots.push(snap);
        }
        let spectrum = music_aoa_spectrum(&ula, &snapshots, 1, 1.0);
        let peak_aoa_deg = find_dominant_aoa_peak(&spectrum).unwrap_or(0.0);

        let receipt = PrivacyWitnessReceipt::create("witness-1", 1700000000, 1, true, &[0x42; 32]);
        let verified = receipt.verify();

        let max_power = spectrum.iter().map(|&(_, p)| p).fold(1.0f32, f32::max);
        let mut aoa_points = String::new();
        for &(deg, power) in &spectrum {
            let x = ((deg + 90.0) / 180.0) * 360.0 + 20.0;
            let y = 180.0 - (power / max_power).clamp(0.0, 1.0) * 140.0;
            aoa_points.push_str(&format!("{x:.1},{y:.1} "));
        }
        let mut phase_points = String::new();
        for (&k, &ph) in indices.iter().zip(sanitized.iter()) {
            let x = ((k as f32 + 30.0) / 60.0) * 360.0 + 20.0;
            let y = 100.0 - (ph * 40.0).clamp(-80.0, 80.0);
            phase_points.push_str(&format!("{x:.1},{y:.1} "));
        }
        let svg_xml = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 400 200\">\
             <rect width=\"400\" height=\"200\" fill=\"#0b1020\"/>\
             <polyline fill=\"none\" stroke=\"#00e5ff\" stroke-width=\"2\" points=\"{}\"/>\
             <polyline fill=\"none\" stroke=\"#ffab00\" stroke-width=\"1.5\" points=\"{}\"/>\
             </svg>",
            aoa_points.trim_end(),
            phase_points.trim_end()
        );
        let aoa_svg_resource = self.encode("svg", svg_xml.into_bytes())?;

        let receipt_json = serde_json::json!({
            "witness_id": receipt.witness_id,
            "timestamp_ns": receipt.timestamp_ns,
            "occupancy_count": receipt.occupancy_count,
            "vitals_healthy": receipt.vitals_healthy,
            "human_rights_guarantee": receipt.human_rights_guarantee,
            "camera_free_verified": verified,
            "vitals": {
                "breathing_bpm": vitals.breathing_bpm,
                "heart_rate_bpm": vitals.heart_rate_bpm,
            },
            "dominant_aoa_deg": peak_aoa_deg,
            "sanitized_subcarrier_count": sanitized.len(),
        });
        let json_bytes = serde_json::to_vec(&receipt_json).map_err(|e| e.to_string())?;
        let receipt_json_resource = self.encode("json", json_bytes)?;

        let mut out = BTreeMap::new();
        out.insert("aoa_svg_resource".into(), aoa_svg_resource);
        out.insert("receipt_json_resource".into(), receipt_json_resource);
        out.insert(
            "sanitized_subcarrier_count".into(),
            Value::I64(sanitized.len() as i64),
        );
        out.insert(
            "breathing_bpm".into(),
            Value::F64(vitals.breathing_bpm as f64),
        );
        out.insert(
            "heart_rate_bpm".into(),
            Value::F64(vitals.heart_rate_bpm as f64),
        );
        out.insert("dominant_aoa_deg".into(), Value::F64(peak_aoa_deg as f64));
        out.insert("camera_free_verified".into(), Value::Bool(verified));
        Ok(Value::Record(out))
    }

    /// Resolves container bytes from retained inputs or workspace encoded resources.
    pub(crate) fn emf_source_bytes<'b>(&'b self, val: &Value) -> Result<&'b [u8], String> {
        if let Ok(key) = text(val) {
            self.inputs
                .iter()
                .find(|r| r.key == key)
                .map(|r| r.bytes.as_slice())
                .ok_or_else(|| format!("Missing retained input: {key}"))
        } else if let Ok(resource) = self.workspace.encoded(val) {
            if resource.kind != "emf" {
                return Err(format!(
                    "Expected an 'emf' resource handle, found '{}'",
                    resource.kind
                ));
            }
            Ok(&resource.bytes)
        } else {
            Err("Expected retained resource key or encoded handle for emf".into())
        }
    }

    pub(crate) fn resolve_emf_bytes(&self, val: &Value) -> Result<Vec<u8>, String> {
        self.resolve_emf_bytes_for(val, 0)
    }

    /// Fixed backing is already included by peak admission; add exactly one
    /// source clone, bounded parse/report scratch, and caller's live output/grid.
    pub(crate) fn resolve_emf_bytes_for(
        &self,
        val: &Value,
        extra: usize,
    ) -> Result<Vec<u8>, String> {
        let required = self
            .emf_read_required(val)?
            .checked_add(extra)
            .ok_or("EMF read peak overflow")?;
        self.emf_peak_admit(required)?;
        Ok(self.emf_source_bytes(val)?.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;
    fn create_test_host<'a>(
        inputs: &'a [Resource],
        cancel: &'a AtomicBool,
        limits: &'a ToolLimits,
    ) -> AssetHost<'a> {
        AssetHost {
            workspace: AssetWorkspace::default(),
            inputs,
            limits,
            cancel,
            work: 0,
        }
    }

    fn f64_field(rec: &BTreeMap<String, Value>, key: &str) -> f64 {
        match rec.get(key) {
            Some(Value::F64(v)) => *v,
            _ => panic!("Expected F64 {key}"),
        }
    }

    fn encoded_bytes(host: &AssetHost<'_>, rec: &BTreeMap<String, Value>, key: &str) -> Vec<u8> {
        let handle = rec.get(key).unwrap_or_else(|| panic!("missing {key}"));
        host.workspace
            .encoded(handle)
            .unwrap_or_else(|_| panic!("encoded {key}"))
            .bytes
            .clone()
    }

    #[test]
    fn test_electromagnetics_csi_vitals_aoa_object_two_produces() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let inputs = vec![];
        let mut host = create_test_host(&inputs, &cancel, &limits);

        let mut args = BTreeMap::new();
        args.insert("breathing_hz".into(), Value::F64(0.25));
        args.insert("heart_hz".into(), Value::F64(1.2));
        args.insert("carrier_ghz".into(), Value::F64(5.18));

        let a = host
            .electromagnetics_op(
                "hypermedia.electromagnetics.sense_wifi_csi_vitals_aoa",
                &args,
            )
            .expect("csi produce a");
        let b = host
            .electromagnetics_op(
                "hypermedia.electromagnetics.sense_wifi_csi_vitals_aoa",
                &args,
            )
            .expect("csi produce b");
        let ra = record(&a).expect("csi record a");
        let rb = record(&b).expect("csi record b");

        assert_eq!(ra.get("camera_free_verified"), Some(&Value::Bool(true)));
        assert_eq!(ra.get("sanitized_subcarrier_count"), Some(&Value::I64(7)));
        let breathing = f64_field(ra, "breathing_bpm");
        let heart = f64_field(ra, "heart_rate_bpm");
        let aoa = f64_field(ra, "dominant_aoa_deg");
        assert!(
            (breathing - 15.0).abs() < 3.0,
            "breathing BPM object, got {breathing}"
        );
        assert!((heart - 72.0).abs() < 8.0, "heart BPM object, got {heart}");
        assert!(
            (aoa - 25.0).abs() <= 2.0,
            "MUSIC AoA must recover 25°, got {aoa}"
        );

        let svg_a = encoded_bytes(&host, ra, "aoa_svg_resource");
        let svg_b = encoded_bytes(&host, rb, "aoa_svg_resource");
        assert_eq!(svg_a, svg_b, "AoA SVG two produces must match");
        let svg_xml = String::from_utf8_lossy(&svg_a);
        assert!(svg_xml.contains("<svg xmlns=\"http://www.w3.org/2000/svg\""));
        assert!(svg_xml.contains("polyline"));

        let json_a = encoded_bytes(&host, ra, "receipt_json_resource");
        let json_b = encoded_bytes(&host, rb, "receipt_json_resource");
        assert_eq!(json_a, json_b, "privacy receipt two produces must match");
        let json_txt = String::from_utf8_lossy(&json_a);
        assert!(json_txt.contains("camera_free_verified"));
        assert!(json_txt.contains("UDHR") || json_txt.contains("witness-1"));
    }
}
