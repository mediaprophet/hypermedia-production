//! Native Etalon contact candidates for an authored mesh motion sequence.

use crate::{fields, integer, list, num, required, vector, Args, AssetHost, TypedArtifact, Value};
use qualia_core_db::{
    gpu_context::try_shared_gpu,
    render::{
        assets::Mesh,
        gpu::vbd_spatial_filter::{
            VbdContactCandidate, VbdContactKind, VbdSpatialFilter, VbdSpatialFilterError,
        },
    },
};
use serde_json::json;
use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    sync::{atomic::Ordering, mpsc},
    time::Duration,
};

const MAX_FRAMES: usize = 8;
const MAX_ELEMENTS: usize = 4096;
const MAX_TARGETS_PER_ROW: usize = 64;

pub(super) fn spatial_filter(host: &mut AssetHost<'_>, args: &Args) -> Result<Value, String> {
    fields(
        args,
        &["source", "frames", "margin_m", "max_candidates_per_query"],
    )?;
    let TypedArtifact::AttributedMesh(source) = host.workspace.typed(required(args, "source")?)?
    else {
        return Err("GPU spatial_filter requires a typed AttributedMesh source".into());
    };
    let source = source.clone();
    let mesh = &source.surface.mesh;
    let frames = list(required(args, "frames")?)?;
    let margin = num(required(args, "margin_m")?)?;
    let max_targets = args
        .get("max_candidates_per_query")
        .map(|v| integer(v, "max_candidates_per_query"))
        .transpose()?
        .unwrap_or(16);
    if !(1..=MAX_FRAMES).contains(&frames.len())
        || !(0.0..=1e6).contains(&margin)
        || !(1..=MAX_TARGETS_PER_ROW).contains(&max_targets)
    {
        return Err(
            "GPU frame count, margin or per-query capacity is outside its admitted range".into(),
        );
    }
    if !(1..=MAX_ELEMENTS).contains(&mesh.positions.len())
        || !(1..=MAX_ELEMENTS).contains(&mesh.triangles.len())
    {
        return Err("GPU spatial_filter source exceeds its vertex or face budget".into());
    }
    validate_positions(&mesh.positions)?;
    let edges = mesh
        .triangles
        .iter()
        .try_fold(BTreeSet::new(), |mut out, &[a, b, c]| {
            if [a, b, c]
                .iter()
                .any(|&index| index as usize >= mesh.positions.len())
                || a == b
                || b == c
                || a == c
            {
                return Err("GPU spatial_filter source contains an invalid triangle");
            }
            for (a, b) in [(a, b), (b, c), (c, a)] {
                out.insert([a.min(b), a.max(b)]);
            }
            if out.len() > MAX_ELEMENTS {
                return Err("GPU spatial_filter source exceeds its edge budget");
            }
            Ok(out)
        })?
        .into_iter()
        .collect::<Vec<_>>();

    let rows = mesh.positions.len() + edges.len();
    let maximum_pairs = rows * max_targets;
    let traversal = mesh.positions.len() * (2 * mesh.triangles.len().next_power_of_two() - 1)
        + edges.len() * (2 * edges.len().next_power_of_two() - 1);
    let pair_bytes = maximum_pairs * 64;
    host.charge(
        (traversal + maximum_pairs * 4) * frames.len(),
        pair_bytes * (frames.len() + 1),
    )?;
    for frame in frames {
        frame_positions(host, mesh, frame)?;
    }

    // A synchronous producer can be invoked inside a current-thread executor.
    let context = std::thread::scope(|scope| scope.spawn(try_shared_gpu).join())
        .map_err(|_| "CapabilityUnavailable: process Etalon device initialization failed")?
        .ok_or("CapabilityUnavailable: process Etalon device is unavailable")?;
    let mut filter = VbdSpatialFilter::new(
        &context.device,
        &context.queue,
        &mesh.positions,
        &mesh.triangles,
        &edges,
        max_targets,
        host.limits.resource_bytes,
    )
    .map_err(filter_error)?;
    let staging = filter.create_readback_buffer();
    let mut candidates = vec![
        VbdContactCandidate {
            kind: VbdContactKind::VertexFace,
            source: 0,
            target: 0
        };
        maximum_pairs
    ];
    let mut previous = Cow::Borrowed(mesh.positions.as_slice());
    let mut frame_outputs = Vec::with_capacity(frames.len());
    let mut vertex_face_total = 0u64;
    let mut edge_edge_total = 0u64;
    for (index, raw) in frames.iter().enumerate() {
        if host.cancel.load(Ordering::Acquire) {
            return Err("Production cancelled".into());
        }
        let current = frame_positions(host, mesh, raw)?;
        let submission = filter
            .refit_and_dispatch(&previous, &current, margin as f32, Some(&staging))
            .map_err(filter_error)?;
        let slice = staging.slice(0..filter.readback_bytes() as u64);
        let (send, receive) = mpsc::channel();
        slice.map_async(etalon::MapMode::Read, move |result| {
            let _ = send.send(result.is_ok());
        });
        if let Err(error) = context.device.poll(etalon::PollType::Wait {
            submission_index: Some(submission),
            timeout: Some(Duration::from_secs(30)),
        }) {
            staging.unmap();
            return Err(format!("Etalon VBD filter wait failed: {error}"));
        }
        if !matches!(receive.try_recv(), Ok(true)) {
            staging.unmap();
            return Err("Etalon VBD candidate readback did not complete".into());
        }
        let decoded = slice
            .get_mapped_range()
            .map_err(|error| error.to_string())
            .and_then(|mapped| {
                filter
                    .decode_candidates_into(&mapped, &mut candidates)
                    .map_err(filter_error)
            });
        staging.unmap();
        let counts = decoded?;
        vertex_face_total += counts.vertex_face as u64;
        edge_edge_total += counts.edge_edge as u64;
        let vertex_face = candidates[..counts.vertex_face]
            .iter()
            .map(|pair| [pair.source, pair.target])
            .collect::<Vec<_>>();
        let edge_edge = candidates[counts.vertex_face..counts.vertex_face + counts.edge_edge]
            .iter()
            .map(|pair| [pair.source, pair.target])
            .collect::<Vec<_>>();
        frame_outputs.push(json!({"previous_frame": index, "current_frame": index + 1, "vertex_face": vertex_face, "edge_edge": edge_edge}));
        previous = current;
    }
    drop(previous);
    let report = json!({
        "schema": "qualia.gpu-vbd-candidates/v1", "backend": "etalon_gpu",
        "method": "linearly_swept_primitive_aabb_broadphase", "margin_m": margin as f32,
        "vertex_count": mesh.positions.len(), "face_count": mesh.triangles.len(), "edge_vertices": edges,
        "frames": frame_outputs,
    });
    let bytes = serde_json::to_vec(&report).map_err(|error| error.to_string())?;
    Ok(Value::Record(BTreeMap::from([
        ("candidates".into(), host.encode("json", bytes)?),
        ("frame_count".into(), Value::U64(frames.len() as u64)),
        ("vertex_face_count".into(), Value::U64(vertex_face_total)),
        ("edge_edge_count".into(), Value::U64(edge_edge_total)),
    ])))
}

fn frame_positions<'a>(
    host: &'a AssetHost<'_>,
    mesh: &Mesh,
    value: &Value,
) -> Result<Cow<'a, [[f32; 3]]>, String> {
    if let Ok(TypedArtifact::AttributedMesh(frame)) = host.workspace.typed(value) {
        let frame = &frame.surface.mesh;
        if frame.triangles != mesh.triangles || frame.positions.len() != mesh.positions.len() {
            return Err(
                "GPU spatial_filter mesh frames must preserve the source topology and vertex order"
                    .into(),
            );
        }
        validate_positions(&frame.positions)?;
        return Ok(Cow::Borrowed(&frame.positions));
    }
    let frame = list(value)?;
    if frame.len() != mesh.positions.len() {
        return Err("GPU spatial_filter frame vertex count differs from source".into());
    }
    let frame = frame
        .iter()
        .map(|point| vector::<3>(point).map(|point| point.map(|v| v as f32)))
        .collect::<Result<Vec<_>, _>>()?;
    validate_positions(&frame)?;
    Ok(Cow::Owned(frame))
}

fn validate_positions(positions: &[[f32; 3]]) -> Result<(), String> {
    if positions
        .iter()
        .flatten()
        .any(|v| !v.is_finite() || v.abs() > 1e8)
    {
        return Err("GPU spatial_filter position exceeds its finite coordinate domain".into());
    }
    Ok(())
}

fn filter_error(error: VbdSpatialFilterError) -> String {
    match error {
        VbdSpatialFilterError::CapabilityUnavailable => format!("CapabilityUnavailable: {error}"),
        _ => error.to_string(),
    }
}
