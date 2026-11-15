use super::{compile_generated, CompileLimits, Emitter, GenerationSpec};
use crate::*;
fn validate_dipoles(value: &Value) -> Result<(Vec<serde_json::Value>, Value), String> {
    let entries = list(value)?;
    if entries.is_empty() || entries.len() > 128 {
        return Err("dipoles requires 1..=128 bioelectric dipole records".into());
    }
    let mut retained = Vec::with_capacity(entries.len());
    let mut total_norm = 0.0f64;
    let mut max_norm = 0.0f64;
    for entry in entries {
        let rec = record(entry)?;
        fields(
            rec,
            &[
                "position",
                "moment",
                "dipole_moment",
                "current_dipole_moment_a_m",
                "carrier_hz",
                "potential_v",
                "label",
            ],
        )?;
        let position = vector::<3>(required(rec, "position")?)?;
        let moment_keys: Vec<_> = ["moment", "dipole_moment", "current_dipole_moment_a_m"]
            .into_iter()
            .filter_map(|k| rec.get(k))
            .collect();
        if moment_keys.len() != 1 {
            return Err("Bioelectric dipole requires exactly one moment vector field".into());
        }
        let moment = vector::<3>(moment_keys[0])?;
        let carrier_hz = rec.get("carrier_hz").map(num).transpose()?;
        if carrier_hz.is_some_and(|f| !f.is_finite() || f <= 0.0) {
            return Err("Bioelectric dipole carrier_hz must be positive finite".into());
        }
        let potential_v = rec.get("potential_v").map(num).transpose()?;
        let label = rec.get("label").map(text).transpose()?;
        if label.is_some_and(|l| l.len() > 256) {
            return Err("Bioelectric dipole label exceeds 256 bytes".into());
        }
        let norm = moment[0].hypot(moment[1]).hypot(moment[2]);
        if !norm.is_finite() {
            return Err("Bioelectric dipole moment norm is not finite".into());
        }
        total_norm += norm;
        max_norm = max_norm.max(norm);
        retained.push(serde_json::json!({
            "position_m": position,
            "current_dipole_moment_a_m": moment,
            "carrier_hz": carrier_hz,
            "extracellular_potential_v": potential_v,
            "label": label,
        }));
    }
    let summary = Value::Record(BTreeMap::from([
        ("dipole_count".into(), Value::U64(retained.len() as u64)),
        ("total_moment_magnitude_a_m".into(), Value::F64(total_norm)),
        ("max_moment_magnitude_a_m".into(), Value::F64(max_norm)),
        (
            "schema".into(),
            Value::String("bioelectric_dipole_v1".into()),
        ),
    ]));
    Ok((retained, summary))
}

fn validate_constitutive(value: &Value) -> Result<(serde_json::Value, Value), String> {
    let rec = record(value)?;
    fields(
        rec,
        &[
            "epsilon_r",
            "epsilon_relative",
            "mu_r",
            "mu_relative",
            "conductivity_s_per_m",
            "model",
            "label",
        ],
    )?;
    let eps = match (rec.get("epsilon_r"), rec.get("epsilon_relative")) {
        (Some(_), Some(_)) => {
            return Err("Specify either epsilon_r or epsilon_relative, not both".into());
        }
        (Some(v), None) | (None, Some(v)) => num(v)?,
        (None, None) => 1.0,
    };
    let mu = match (rec.get("mu_r"), rec.get("mu_relative")) {
        (Some(_), Some(_)) => return Err("Specify either mu_r or mu_relative, not both".into()),
        (Some(v), None) | (None, Some(v)) => num(v)?,
        (None, None) => 1.0,
    };
    let sigma = number(rec, "conductivity_s_per_m", 0.0)?;
    if !eps.is_finite() || eps <= 0.0 || !mu.is_finite() || mu <= 0.0 || !sigma.is_finite() || sigma < 0.0 {
        return Err("Constitutive parameters must satisfy finite epsilon_r>0, mu_r>0, conductivity>=0".into());
    }
    let model = rec.get("model").map(text).transpose()?;
    let label = rec.get("label").map(text).transpose()?;
    if model.is_some_and(|m| m.len() > 256) || label.is_some_and(|l| l.len() > 256) {
        return Err("Constitutive model/label exceeds 256 bytes".into());
    }
    let retained = serde_json::json!({
        "epsilon_relative": eps,
        "mu_relative": mu,
        "conductivity_s_per_m": sigma,
        "model": model,
        "label": label,
    });
    let mut summary = BTreeMap::from([
        ("epsilon_relative".into(), Value::F64(eps)),
        ("mu_relative".into(), Value::F64(mu)),
        ("conductivity_s_per_m".into(), Value::F64(sigma)),
        (
            "schema".into(),
            Value::String("constitutive_model_v1".into()),
        ),
    ]);
    if let Some(m) = model {
        summary.insert("model".into(), Value::String(m.into()));
    }
    if let Some(l) = label {
        summary.insert("label".into(), Value::String(l.into()));
    }
    Ok((retained, Value::Record(summary)))
}

impl AssetHost<'_> {
    pub(crate) fn compile_emf_current(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "origin",
                "voxel_size",
                "grid_dims",
                "epoch_s",
                "sources",
                "dipoles",
                "constitutive",
                "interpretation",
                "audio_tracks",
            ],
        )?;
        if args.contains_key("interpretation") || args.contains_key("audio_tracks") {
            return Err("Current instantaneous-field producer does not yet emit acquisition/audio/other interpretation profiles; source parameters were not consumed".into());
        }
        let dipole_summary = args.get("dipoles").map(validate_dipoles).transpose()?;
        let constitutive_summary = args
            .get("constitutive")
            .map(validate_constitutive)
            .transpose()?;
        if self.workspace.entries.len() >= self.limits.resources {
            return Err("EMF output requires an available resource slot".into());
        }
        self.emf_peak_admit(64 * 1024)?;
        let origin = args
            .get("origin")
            .map(vector::<3>)
            .transpose()?
            .unwrap_or([0.0; 3]);
        let spacing = number(args, "voxel_size", 0.5)?;
        if origin.iter().any(|v| !v.is_finite()) || !spacing.is_finite() || spacing <= 0.0 {
            return Err("Origin/voxel_size must be finite and spacing positive".into());
        }
        let brick_grid = if let Some(value) = args.get("grid_dims") {
            let values = list(value)?;
            if values.len() != 3 {
                return Err("grid_dims requires3 positive dimensions".into());
            }
            let mut dims = [0; 3];
            for (axis, value) in values.iter().enumerate() {
                dims[axis] = u32::try_from(integer(value, "grid_dims")?)
                    .map_err(|_| "grid_dims exceeds u32")?;
                if dims[axis] == 0 {
                    return Err("grid_dims must be positive".into());
                }
            }
            dims
        } else {
            [1; 3]
        };
        let epoch = number(args, "epoch_s", 1_700_000_000.0)?;
        let snapshot = super::clock::from_f64_epoch(epoch)?;
        let sources = list(required(args, "sources")?)?;
        if sources.is_empty() || sources.len() > 16 {
            return Err("sources requires1..16 synthetic emitters".into());
        }
        let mut emitters = Vec::with_capacity(sources.len());
        let mut retained = Vec::with_capacity(sources.len());
        for source in sources {
            let source = record(source)?;
            fields(
                source,
                &[
                    "position",
                    "e_amplitude",
                    "wavelength_nm",
                    "phase",
                    "stokes",
                ],
            )?;
            let position = vector::<3>(required(source, "position")?)?;
            let amplitude = vector::<3>(required(source, "e_amplitude")?)?;
            let phase = number(source, "phase", 0.0)?;
            let wavelength_nm = match source.get("wavelength_nm") {
                Some(Value::Null) => None,
                _ => Some(number(source, "wavelength_nm", 550.0)?),
            };
            let wavelength_m = if let Some(nm) = wavelength_nm {
                let m = nm * 1e-9;
                if !nm.is_finite() || nm <= 0.0 || !m.is_finite() || m <= 0.0 {
                    return Err("Physical wavelength is positive finite or explicit null for static; conversion must be representable".into());
                }
                Some(m)
            } else {
                None
            };
            let stokes = source.get("stokes").map(vector::<3>).transpose()?;
            if position.iter().chain(&amplitude).any(|v| !v.is_finite())
                || !phase.is_finite()
                || stokes.is_some_and(|v| v.iter().any(|x| !x.is_finite()))
            {
                return Err("Synthetic source values must be finite".into());
            }
            emitters.push(Emitter {
                position,
                electric_amplitude: amplitude,
                wavelength_m,
                phase_rad: phase,
                authored_stokes: stokes,
            });
            retained.push(serde_json::json!({"position_m":position,"electric_amplitude_v_per_m":amplitude,"wavelength_nm":wavelength_nm,"phase_rad":phase,"authored_stokes_parameters_retained_only":stokes}));
        }
        let mut lineage_obj = serde_json::json!({"model":"softened_radial_cosine_snapshot_v1","epistemic_class":"model_estimate","qualification":"synthetic snapshot pattern; not a Maxwell/near-field solver or measured source","dynamic_B_rule":"radial_direction cross E divided by exact vacuum c; pattern coefficient, no constitutive/energy inference","static_rule":"softened electric pattern, B0, no carrier","phase_origin":"emitter coefficient at snapshot epoch; no temporal evolution solved","origin_m":origin,"spacing_m":spacing,"grid_dims_bricks":brick_grid,"authored_epoch_s":epoch,"encoded_epoch_seconds":snapshot.clock.epoch_seconds,"encoded_start_tick":snapshot.start_tick,"clock_num":snapshot.clock.ticks_per_second_num,"clock_den":snapshot.clock.ticks_per_second_den,"sources":retained});
        if let Some((ref dipole_json, _)) = dipole_summary {
            lineage_obj["dipoles"] = serde_json::Value::Array(dipole_json.clone());
        }
        if let Some((ref constitutive_json, _)) = constitutive_summary {
            lineage_obj["constitutive"] = constitutive_json.clone();
        }
        let lineage = serde_json::to_string(&lineage_obj).map_err(|e| e.to_string())?;
        if lineage.len() > 16384 {
            return Err("Synthetic source provenance exceeds16KiB".into());
        }
        let bricks = brick_grid
            .iter()
            .try_fold(1usize, |n, &d| n.checked_mul(d as usize))
            .ok_or("EMF brick count overflow")?;
        let peak = super::model::generate_peak(bricks)?;
        let work = super::model::generate_work(bricks, emitters.len())?;
        self.emf_peak_admit(peak)?;
        self.charge(
            usize::try_from(work).map_err(|_| "EMF work exceeds usize")?,
            0,
        )?;
        let fixed = self
            .inputs
            .iter()
            .try_fold(self.workspace.bytes, |n, r| n.checked_add(r.bytes.len()))
            .ok_or("EMF fixed byte overflow")?;
        let peak_available = self
            .limits
            .total_resource_bytes
            .checked_sub(fixed)
            .ok_or("EMF fixed bytes exceed budget")?;
        let spec = GenerationSpec {
            origin_m: origin,
            spacing_m: spacing,
            brick_grid,
            snapshot,
            emitters: &emitters,
            lineage: &lineage,
        };
        let output = compile_generated(
            &spec,
            CompileLimits {
                file_bytes: self.limits.resource_bytes,
                peak_bytes: peak_available,
                work_units: work,
            },
        )?;
        let mut handle = self.encode("emf", output)?;
        if let Value::Record(meta) = &mut handle {
            meta.insert("container_version".into(), Value::U64(4));
            meta.insert(
                "profile".into(),
                Value::String("instantaneous_E_B_v1".into()),
            );
            meta.insert(
                "generation_model".into(),
                Value::String("softened_radial_cosine_snapshot_v1".into()),
            );
            meta.insert("source_role".into(), Value::String("model_estimate".into()));
            if let Some((dipole_json, summary)) = dipole_summary {
                meta.insert(
                    "dipole_records".into(),
                    Value::U64(dipole_json.len() as u64),
                );
                meta.insert("bioelectric_dipole_summary".into(), summary);
            }
            if let Some((_, summary)) = constitutive_summary {
                meta.insert("constitutive_summary".into(), summary);
            }
        }
        Ok(handle)
    }
    /// Conservative logical live-payload admission, including immutable sources.
    pub(crate) fn emf_peak_admit(&self, scratch: usize) -> Result<(), String> {
        let fixed = self
            .inputs
            .iter()
            .try_fold(self.workspace.bytes, |n, input| {
                n.checked_add(input.bytes.len())
            })
            .ok_or("EMF fixed byte overflow")?;
        let peak = fixed.checked_add(scratch).ok_or("EMF peak overflow")?;
        if self.cancel.load(std::sync::atomic::Ordering::Acquire) {
            return Err("Production cancelled".into());
        }
        if peak > self.limits.total_resource_bytes || self.work > self.limits.asset_work_units {
            return Err("EMF combined live payload/work exceeds budget".into());
        }
        Ok(())
    }
}
