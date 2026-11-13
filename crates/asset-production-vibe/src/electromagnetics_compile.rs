//! Bounded physical field generation; optional v3 metadata/audio stays in its own adapter.
use crate::*;
use qualia_core_db::q42::emf_container::{emf_brick_voxel_linear_index,EmfCellRecord64,EmfContainerWriter,EMF_BRICK_CELLS,SPEED_OF_LIGHT};
use qualia_core_db::q42::m64_matter::brick3d::encode_hilbert3d_u64;
use std::f64::consts::PI;

/// A wave/dipole emitter in 3D space with polarization and coherence parameters.
#[derive(Clone, Debug)]
struct WaveEmitter {
    position: [f64; 3],
    e_amplitude: [f64; 3],
    wavelength_nm: f64,
    phase: f64,
    stokes: [f64; 3],
}

impl AssetHost<'_> {
    /// Synthesizes a 3D wave-optical & electromagnetic field volume and seals it
    /// into QualiaDB's native `Q42EMF64` (`.emf`) binary container.
    pub(crate) fn compile_emf(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["origin", "voxel_size", "grid_dims", "epoch_s", "sources", "interpretation", "audio_tracks"])?;

        let origin = match a.get("origin") {
            Some(v) => {
                let o = vector::<3>(v)?;
                [o[0] as f32, o[1] as f32, o[2] as f32]
            }
            None => [0.0f32, 0.0f32, 0.0f32],
        };

        let voxel_size = number(a, "voxel_size", 0.5)? as f32;
        if !(0.001..=1000.0).contains(&voxel_size) {
            return Err("voxel_size outside 0.001..=1000.0".into());
        }

        let grid_dims = match a.get("grid_dims") {
            Some(v) => {
                let l = list(v)?;
                if l.len() != 3 {
                    return Err("grid_dims must be a list of 3 integers".into());
                }
                let bx = integer(&l[0], "grid_dims[0]")?;
                let by = integer(&l[1], "grid_dims[1]")?;
                let bz = integer(&l[2], "grid_dims[2]")?;
                if !(1..=4).contains(&bx) || !(1..=4).contains(&by) || !(1..=4).contains(&bz) {
                    return Err("grid_dims components must be in 1..=4".into());
                }
                [bx as u32, by as u32, bz as u32]
            }
            None => [1u32, 1u32, 1u32],
        };

        let num_bricks=(grid_dims[0]*grid_dims[1]*grid_dims[2]) as usize;
        if !a.contains_key("interpretation") {
            if self.workspace.entries.len()>=self.limits.resources{return Err("EMF output requires an available resource slot".into());}
            let cells=num_bricks.checked_mul(512*64).ok_or("EMF field size overflow")?;
            let output=num_bricks.checked_mul(512*64+128).and_then(|n|n.checked_add(64)).ok_or("EMF output size overflow")?;
            if output>self.limits.resource_bytes{return Err("EMF serialized output exceeds selected resource budget".into());}
            let scratch=cells.checked_mul(2).and_then(|n|n.checked_add(output))
                .and_then(|n|n.checked_add(num_bricks.checked_mul(128)?))
                .and_then(|n|n.checked_add(16*std::mem::size_of::<WaveEmitter>())).ok_or("EMF peak size overflow")?;
            self.emf_peak_admit(scratch)?;
        }
        let v3=self.emf_v3_request(a,num_bricks)?;

        let epoch_s = match &v3 {
            Some(request)=>{
                let epoch=request.field.timebase.epoch.header_seconds();
                if a.contains_key("epoch_s")&&!request.field.timebase.epoch.matches_header_seconds(number(a,"epoch_s",epoch)?) {
                    return Err("epoch_s contradicts the authoritative v3 exact epoch".into());
                }
                if grid_dims.iter().any(|v|*v>(1u32<<request.field.hilbert_order)){return Err("grid_dims exceed explicit v3 Hilbert order".into());}
                epoch
            },None=>number(a,"epoch_s",1_700_000_000.0)?,
        };

        let sources_val = required(a, "sources")?;
        let sources_list = list(sources_val)?;
        if sources_list.is_empty() || sources_list.len() > 16 {
            return Err("sources must contain 1..=16 emitters".into());
        }

        let mut emitters = Vec::with_capacity(sources_list.len());
        for s_val in sources_list {
            let s_rec = record(s_val)?;
            fields(
                s_rec,
                &["position", "e_amplitude", "wavelength_nm", "phase", "stokes"],
            )?;
            let pos = vector::<3>(required(s_rec, "position")?)?;
            let e_amp = vector::<3>(required(s_rec, "e_amplitude")?)?;
            let wavelength_nm = number(s_rec, "wavelength_nm", 550.0)?;
            let range=if v3.is_some(){1e-9..=1e18}else{1.0..=1e9};
            if !range.contains(&wavelength_nm){return Err("wavelength_nm exceeds selected v2/v3 physical emitter range".into());}
            let phase = number(s_rec, "phase", 0.0)?;
            let stokes = match s_rec.get("stokes") {
                Some(v) => vector::<3>(v)?,
                None => [0.0, 0.0, 0.0],
            };
            emitters.push(WaveEmitter {
                position: pos,
                e_amplitude: e_amp,
                wavelength_nm,
                phase,
                stokes,
            });
        }

        let total_voxels =
            (grid_dims[0] as usize) * (grid_dims[1] as usize) * (grid_dims[2] as usize) * EMF_BRICK_CELLS;
        let work = total_voxels * emitters.len();
        let scratch = total_voxels * 64;
        self.charge(work, scratch)?;

        let mut writer = EmfContainerWriter::new(origin, voxel_size, grid_dims, epoch_s);

        let c_light = SPEED_OF_LIGHT as f64;
        let order=v3.as_ref().map(|request|u32::from(request.field.hilbert_order)).unwrap_or(2);
        let mut bricks = Vec::with_capacity(num_bricks);

        for bz in 0..grid_dims[2] {
            for by in 0..grid_dims[1] {
                for bx in 0..grid_dims[0] {
                    let hilbert_key = encode_hilbert3d_u64(bx, by, bz, order);
                    let mut brick_cells = [EmfCellRecord64::default(); EMF_BRICK_CELLS];

                    for lz in 0..8u8 {
                        for ly in 0..8u8 {
                            for lx in 0..8u8 {
                                let gx = (bx * 8 + lx as u32) as f64;
                                let gy = (by * 8 + ly as u32) as f64;
                                let gz = (bz * 8 + lz as u32) as f64;

                                let px = origin[0] as f64 + gx * (voxel_size as f64);
                                let py = origin[1] as f64 + gy * (voxel_size as f64);
                                let pz = origin[2] as f64 + gz * (voxel_size as f64);

                                let mut total_e = [0.0f64; 3];
                                let mut total_b = [0.0f64; 3];
                                let mut sum_weight = 0.0f64;
                                let mut sum_wavelength = 0.0f64;
                                let mut sum_q = 0.0f64;
                                let mut sum_u = 0.0f64;
                                let mut sum_v = 0.0f64;
                                let mut phasor_x = 0.0f64;
                                let mut phasor_y = 0.0f64;

                                for emitter in &emitters {
                                    let dx = px - emitter.position[0];
                                    let dy = py - emitter.position[1];
                                    let dz = pz - emitter.position[2];
                                    let r = (dx * dx + dy * dy + dz * dz).sqrt();

                                    let atten = 1.0 / (1.0 + r * r);

                                    let lambda_m = emitter.wavelength_nm * 1e-9;
                                    let k = 2.0 * PI / lambda_m;

                                    let wave_phase = k * r + emitter.phase;
                                    let cos_phase = wave_phase.cos();
                                    let sin_phase = wave_phase.sin();

                                    let e_contrib = [
                                        emitter.e_amplitude[0] * atten * cos_phase,
                                        emitter.e_amplitude[1] * atten * cos_phase,
                                        emitter.e_amplitude[2] * atten * cos_phase,
                                    ];

                                    total_e[0] += e_contrib[0];
                                    total_e[1] += e_contrib[1];
                                    total_e[2] += e_contrib[2];

                                    let k_hat = if r > 1e-9 {
                                        [dx / r, dy / r, dz / r]
                                    } else {
                                        [0.0, 0.0, 1.0]
                                    };

                                    let k_cross_e = [
                                        k_hat[1] * e_contrib[2] - k_hat[2] * e_contrib[1],
                                        k_hat[2] * e_contrib[0] - k_hat[0] * e_contrib[2],
                                        k_hat[0] * e_contrib[1] - k_hat[1] * e_contrib[0],
                                    ];

                                    total_b[0] += k_cross_e[0] / c_light;
                                    total_b[1] += k_cross_e[1] / c_light;
                                    total_b[2] += k_cross_e[2] / c_light;

                                    let amp_mag = (emitter.e_amplitude[0] * emitter.e_amplitude[0]
                                        + emitter.e_amplitude[1] * emitter.e_amplitude[1]
                                        + emitter.e_amplitude[2] * emitter.e_amplitude[2])
                                        .sqrt();
                                    let weight = (amp_mag * atten).max(1e-12);

                                    sum_weight += weight;
                                    sum_wavelength += weight * emitter.wavelength_nm;
                                    sum_q += weight * emitter.stokes[0];
                                    sum_u += weight * emitter.stokes[1];
                                    sum_v += weight * emitter.stokes[2];

                                    phasor_x += weight * cos_phase;
                                    phasor_y += weight * sin_phase;
                                }

                                let (eff_wavelength, eff_q, eff_u, eff_v, eff_phase) =
                                    if sum_weight > 0.0 {
                                        (
                                            (sum_wavelength / sum_weight) as f32,
                                            (sum_q / sum_weight) as f32,
                                            (sum_u / sum_weight) as f32,
                                            (sum_v / sum_weight) as f32,
                                            phasor_y.atan2(phasor_x) as f32,
                                        )
                                    } else {
                                        (550.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32)
                                    };

                                let mut cell = EmfCellRecord64 {
                                    e_field: [total_e[0] as f32, total_e[1] as f32, total_e[2] as f32],
                                    b_field: [total_b[0] as f32, total_b[1] as f32, total_b[2] as f32],
                                    poynting_flux: 0.0,
                                    wavelength_nm: eff_wavelength,
                                    refractive_n: 1.0,
                                    extinction_k: 0.0,
                                    coherence_phase: eff_phase,
                                    stokes_q: eff_q,
                                    stokes_u: eff_u,
                                    stokes_v: eff_v,
                                    spectral_sigma: ((eff_wavelength - 400.0) / 300.0).clamp(0.0, 1.0),
                                    confidence_alpha: 1.0,
                                };
                                cell.poynting_flux = cell.poynting_flux_magnitude();

                                let voxel_idx = emf_brick_voxel_linear_index(lx, ly, lz);
                                brick_cells[voxel_idx] = cell;
                            }
                        }
                    }

                    bricks.push((hilbert_key, brick_cells));
                }
            }
        }

        bricks.sort_by_key(|(h, _)| *h);
        for (h, cells) in bricks {
            if v3.is_some()&&bytemuck::cast_slice::<EmfCellRecord64,f32>(&cells).iter().any(|v|!v.is_finite()) {
                return Err("Generated v3 physical cells overflow finite f32 representation".into());
            }
            writer
                .add_brick_chunk(h, &cells, 0, 1, 0)
                .map_err(|e| format!("Failed to add brick chunk: {e}"))?;
        }

        let bytes=match &v3 {
            None=>writer.to_bytes(),Some(request)=>{
                let tracks:Vec<_>=request.tracks.iter().map(|track|qualia_core_db::q42::emf_container::EmfAudioTrack{spec:track.spec.clone(),samples:&track.samples}).collect();
                writer.to_bytes_v3(&request.field,&tracks,qualia_core_db::q42::emf_container::EmfExtensionLimits{
                    max_file_bytes:self.limits.resource_bytes,max_audio_bytes:self.limits.resource_bytes,max_metadata_bytes:64*1024,max_tracks:16})
            },
        }.map_err(|e|format!("Failed to serialize EMF container: {e}"))?;
        self.encode("emf", bytes)
    }

}
