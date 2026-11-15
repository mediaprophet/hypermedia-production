use qualia_core_db::q42::emf_profiles::*;
use qualia_core_db::q42::field_semantics::{NodeState, Scratch};
use std::mem::size_of;
/// Parameters of a declared synthetic snapshot pattern, not measured emitters
/// or a qualified Maxwell/near-field propagation solver.
#[derive(Clone, Debug)]
pub(crate) struct Emitter {
    pub position: [f64; 3],
    pub electric_amplitude: [f64; 3],
    pub wavelength_m: Option<f64>,
    pub phase_rad: f64,
    pub authored_stokes: Option<[f64; 3]>,
}
pub(crate) struct GenerationSpec<'a> {
    pub origin_m: [f64; 3],
    pub spacing_m: f64,
    pub brick_grid: [u32; 3],
    pub snapshot: super::clock::SnapshotClock,
    pub emitters: &'a [Emitter],
    pub lineage: &'a str,
}
#[derive(Clone, Copy)]
pub(crate) struct CompileLimits {
    pub file_bytes: usize,
    pub peak_bytes: usize,
    pub work_units: u64,
}
fn checked(n: Option<usize>) -> Result<usize, String> {
    n.ok_or_else(|| "EMF generation size overflow".into())
}
pub(crate) fn generate_peak(bricks: usize) -> Result<usize, String> {
    let payload = checked(bricks.checked_mul(512 * 128))?;
    checked(
        payload
            .checked_mul(3)
            .and_then(|n| n.checked_add(512 * size_of::<FieldSample>()))
            .and_then(|n| n.checked_add(bricks.checked_mul(512)?))
            .and_then(|n| n.checked_add(128 * 1024)),
    )
}
pub(crate) fn generate_work(bricks: usize, emitters: usize) -> Result<u64, String> {
    let cells = checked(bricks.checked_mul(512))?;
    u64::try_from(checked(
        cells.checked_mul(
            emitters
                .checked_mul(16)
                .and_then(|n| n.checked_add(128))
                .ok_or("EMF work overflow")?,
        ),
    )?)
    .map_err(|_| "EMF work overflow".into())
}
pub(crate) fn compile_generated(
    spec: &GenerationSpec<'_>,
    limits: CompileLimits,
) -> Result<Vec<u8>, String> {
    if spec.emitters.is_empty()
        || spec.emitters.len() > 16
        || spec.lineage.len() > 16384
        || spec.origin_m.iter().any(|v| !v.is_finite())
        || !spec.spacing_m.is_finite()
        || spec.spacing_m <= 0.0
        || spec.brick_grid.contains(&0)
    {
        return Err("Invalid finite synthetic field parameters".into());
    }
    for e in spec.emitters {
        if e.position
            .iter()
            .chain(&e.electric_amplitude)
            .any(|v| !v.is_finite())
            || !e.phase_rad.is_finite()
            || e.wavelength_m.is_some_and(|v| !v.is_finite() || v <= 0.0)
            || e.authored_stokes
                .is_some_and(|v| v.iter().any(|x| !x.is_finite()))
        {
            return Err("Invalid finite emitter parameters".into());
        }
        if e.wavelength_m.is_none() && e.phase_rad != 0.0 {
            return Err("Static synthetic emitters require phase0".into());
        }
    }
    let bricks = spec
        .brick_grid
        .iter()
        .try_fold(1usize, |n, &d| n.checked_mul(d as usize))
        .ok_or("EMF brick count overflow")?;
    if generate_peak(bricks)? > limits.peak_bytes
        || generate_work(bricks, spec.emitters.len())? > limits.work_units
    {
        return Err("EMF generation exceeds admitted peak/work budget".into());
    }
    let output_bound = checked(
        bricks
            .checked_mul(512 * 128 + 128)
            .and_then(|n| n.checked_add(64 * 1024)),
    )?;
    if output_bound > limits.file_bytes {
        return Err("EMF current output exceeds selected file budget".into());
    }
    let max = *spec.brick_grid.iter().max().unwrap();
    let order = if max <= 1 {
        0
    } else {
        32 - (max - 1).leading_zeros()
    };
    let dimensions = spec
        .brick_grid
        .map(|n| n.checked_mul(8).ok_or("EMF cell dimensions overflow"))
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;
    let grid = GridSpec {
        dimensions: [dimensions[0], dimensions[1], dimensions[2]],
        brick_dimensions: [8; 3],
        spacing_m: [spec.spacing_m; 3],
        indexing: GridIndexing::Hilbert { order },
    };
    let layout = grid.validate().map_err(|e| format!("EMF grid: {e:?}"))?;
    let common_carrier = common_carrier(spec.emitters)?;
    let mut payloads = Vec::with_capacity(bricks);
    let mut samples = Vec::with_capacity(512);
    for bz in 0..spec.brick_grid[2] {
        for by in 0..spec.brick_grid[1] {
            for bx in 0..spec.brick_grid[0] {
                let key = layout
                    .encode_key([bx, by, bz])
                    .map_err(|e| format!("EMF key: {e:?}"))?;
                let brick = layout.brick(key).map_err(|e| format!("EMF brick: {e:?}"))?;
                samples.clear();
                for i in 0..brick.cell_count() {
                    let local = brick
                        .local_position_m(i)
                        .map_err(|e| format!("EMF position: {e:?}"))?;
                    let world = std::array::from_fn(|axis| spec.origin_m[axis] + local[axis]);
                    if world.iter().any(|v| !v.is_finite()) {
                        return Err("Synthetic world position is not representable".into());
                    }
                    let (electric, magnetic) = evaluate_snapshot(world, spec.emitters)?;
                    samples.push(FieldSample {
                        electric_v_per_m: electric,
                        magnetic_tesla: magnetic,
                        position_m: local,
                        tick: spec.snapshot.start_tick,
                        carrier_hz: common_carrier,
                        stored_poynting_w_per_m2: None,
                        confidence: None,
                    });
                }
                let mut payload = vec![0; 512 * 128];
                encode_field_samples(&samples, &mut payload, Limits::default())
                    .map_err(|e| format!("EMF f64 samples: {e:?}"))?;
                payloads.push((key, payload));
            }
        }
    }
    payloads.sort_by_key(|item| item.0);
    let chunks: Vec<_> = payloads
        .iter()
        .map(|(key, payload)| FieldChunkInput {
            stream_id: 1,
            start_tick: spec.snapshot.start_tick,
            time_count: 1,
            spatial_key: Some(*key),
            payload,
        })
        .collect();
    let document = FieldDocument {
        clock: spec.snapshot.clock,
        frame: FieldFrame {
            origin_m: spec.origin_m,
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            grid: Some(grid),
        },
        provenance: FieldProvenance {
            role: 1,
            algorithm: Some("softened_radial_cosine_snapshot_v1"),
            algorithm_version: Some("1"),
            lineage: Some(spec.lineage),
        },
        temporal_step_ticks: Some(1),
        chunks: &chunks,
        align_payloads_to_4096: false,
    };
    let mut write_limits = FieldWriteLimits::default();
    write_limits.storage.file_bytes = limits.file_bytes as u64;
    write_limits.storage.aggregate_decoded_bytes = limits.file_bytes as u64;
    let mut metadata = vec![
        0;
        field_metadata_len(&document, write_limits)
            .map_err(|e| format!("EMF metadata: {e:?}"))?
    ];
    let mut entries = vec![field_entry_slot(); chunks.len()];
    let prepared = prepare_field_document(&document, &mut metadata, &mut entries, write_limits)
        .map_err(|e| format!("EMF prepare: {e:?}"))?;
    let size = prepared
        .encoded_len()
        .map_err(|e| format!("EMF size: {e:?}"))?;
    if size > limits.file_bytes {
        return Err("EMF serialized output exceeds budget".into());
    }
    let mut staging = vec![0; size];
    let mut output = vec![0; size];
    let mut nodes = [NodeState::default(); 5];
    let mut entry_nodes = vec![NodeState::default(); chunks.len()];
    prepared
        .encode_into(
            &mut staging,
            &mut output,
            &mut Scratch {
                metadata: &mut nodes,
                entries: &mut entry_nodes,
            },
        )
        .map_err(|e| format!("EMF current publication: {e:?}"))?;
    Ok(output)
}
fn common_carrier(emitters: &[Emitter]) -> Result<Option<f64>, String> {
    let Some(wavelength) = emitters[0].wavelength_m else {
        return Ok(None);
    };
    if emitters.iter().any(|e| e.wavelength_m != Some(wavelength)) {
        return Ok(None);
    }
    qualia_core_db::q42::field_spectrum::vacuum_wavelength_m_to_frequency_hz(wavelength)
        .map(Some)
        .map_err(|e| format!("Carrier is not representable: {e:?}"))
}
fn evaluate_snapshot(
    world: [f64; 3],
    emitters: &[Emitter],
) -> Result<([f64; 3], [f64; 3]), String> {
    let mut electric = [0.0; 3];
    let mut magnetic = [0.0; 3];
    let c = qualia_core_db::q42::field_spectrum::VACUUM_SPEED_OF_LIGHT_M_PER_S;
    for source in emitters {
        let delta: [f64; 3] = std::array::from_fn(|axis| world[axis] - source.position[axis]);
        if delta.iter().any(|v| !v.is_finite()) {
            return Err("Synthetic separation is not representable".into());
        }
        let r = delta[0].hypot(delta[1]).hypot(delta[2]);
        if !r.is_finite() {
            return Err("Synthetic distance is not representable".into());
        }
        let factor = if let Some(wavelength) = source.wavelength_m {
            let phase = (std::f64::consts::TAU / wavelength) * r + source.phase_rad;
            if !phase.is_finite() {
                return Err("Synthetic phase is not representable".into());
            }
            phase.cos()
        } else {
            1.0
        };
        let field: [f64; 3] = std::array::from_fn(|axis| {
            let a = source.electric_amplitude[axis];
            let softened = if r <= 1.0 {
                a / (1.0 + r * r)
            } else {
                let inverse = 1.0 / r;
                (a / r / r) / (1.0 + inverse * inverse)
            };
            softened * factor
        });
        for axis in 0..3 {
            electric[axis] += field[axis];
        }
        if source.wavelength_m.is_some() {
            let direction = if r == 0.0 {
                [0.0, 0.0, 1.0]
            } else {
                delta.map(|v| v / r)
            };
            let cross = [
                direction[1] * (field[2] / c) - direction[2] * (field[1] / c),
                direction[2] * (field[0] / c) - direction[0] * (field[2] / c),
                direction[0] * (field[1] / c) - direction[1] * (field[0] / c),
            ];
            for axis in 0..3 {
                magnetic[axis] += cross[axis];
            }
        }
    }
    if electric.iter().chain(&magnetic).any(|v| !v.is_finite()) {
        return Err("Synthetic f64 field is not representable".into());
    }
    Ok((electric, magnetic))
}
