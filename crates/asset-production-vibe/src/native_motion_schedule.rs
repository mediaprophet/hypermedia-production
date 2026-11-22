//! Production adapters for planar and vector motion, block diagrams, and CPM.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::qualia_containers_governance_poet_ops::encoded_resource;
use crate::*;
use kinetics_motion_compositor::{
    GrayImage, ImagePyramid, TextAnimator, detect_shi_tomasi_corners, eval_layer_transform,
    evaluate_text_animators, parse_lottie, solve_corner_pin_homography, track_klt_point,
};
use serde_json::{Value as Json, json};

impl AssetHost<'_> {
    pub(crate) fn track_planar_and_animate_vector(&mut self, args: &Args) -> Result<Value, String> {
        let result = match text(required(args, "task")?)? {
            "corner_pin" => {
                fields(args, &["task", "corners", "frames"])?;
                self.planar_track(required(args, "corners")?, required(args, "frames")?)?
            }
            "lottie" => {
                fields(args, &["task", "document", "times_s"])?;
                let source =
                    native_document(self, required(args, "document")?, &["lottie", "json"])?;
                let animation = parse_lottie(&source)?;
                let times = sample_times(required(args, "times_s")?)?;
                if animation.fr <= 0.0
                    || animation.op <= animation.ip
                    || animation.layers.len() > 512
                {
                    return Err("Invalid Lottie frame extent, frame rate, or layer count".into());
                }
                self.charge(
                    source.len().saturating_mul(times.len()),
                    times.len() * animation.layers.len() * 320,
                )?;
                let mut samples = Vec::with_capacity(times.len());
                for time in times {
                    let frame = time.mul_add(animation.fr, animation.ip);
                    if frame < animation.ip || frame >= animation.op || !frame.is_finite() {
                        return Err(
                            "Lottie sample time is outside its half-open frame extent".into()
                        );
                    }
                    let layers = animation
                        .layers
                        .iter()
                        .enumerate()
                        .filter(|(_, layer)| layer.ip <= frame && frame < layer.op)
                        .map(|(index, layer)| {
                            if layer.sr <= 0.0 {
                                return Err("Lottie layer stretch must be positive");
                            }
                            let evaluated = eval_layer_transform(layer, frame);
                            if evaluated
                                .position
                                .iter()
                                .chain(&evaluated.anchor)
                                .chain(&evaluated.scale)
                                .any(|v| !v.is_finite())
                                || !evaluated.rotation_deg.is_finite()
                                || !evaluated.opacity.is_finite()
                            {
                                return Err("Lottie produced a non-finite transform");
                            }
                            Ok(json!({
                                "layer": index,
                                "anchor": evaluated.anchor,
                                "position": evaluated.position,
                                "scale": evaluated.scale,
                                "rotation_deg": evaluated.rotation_deg,
                                "opacity": evaluated.opacity
                            }))
                        })
                        .collect::<Result<Vec<_>, &str>>()?;
                    samples.push(json!({ "time_s": time, "frame": frame, "layers": layers }));
                }
                json!({ "schema": "qualia-lottie-transform-track/1", "samples": samples })
            }
            "kinetic_text" => {
                fields(args, &["task", "document", "text", "times_s"])?;
                let source = native_document(self, required(args, "document")?, &["json"])?;
                let animators: Vec<TextAnimator> = serde_json::from_str(&source)
                    .map_err(|err| format!("Text animator document: {err}"))?;
                let label = text(required(args, "text")?)?;
                let times = sample_times(required(args, "times_s")?)?;
                let characters = label.chars().count();
                if !(1..=4096).contains(&characters) || animators.len() > 64 {
                    return Err(
                        "Text animator requires 1 to 4096 characters and up to 64 animators".into(),
                    );
                }
                let samples = times.len() * characters;
                self.charge(samples * animators.len().max(1) * 64, samples * 352)?;
                let mut output = Vec::with_capacity(times.len());
                for time in times {
                    let transforms = evaluate_text_animators(label, &animators, time);
                    if transforms.iter().any(|x| {
                        x.position
                            .iter()
                            .chain(&x.rotation)
                            .chain(&x.scale)
                            .any(|v| !v.is_finite())
                            || !x.opacity.is_finite()
                    }) {
                        return Err("Text animator produced a non-finite transform".into());
                    }
                    output.push(json!({ "time_s": time, "transforms": transforms }));
                }
                json!({ "schema": "qualia-kinetic-text-track/1", "text": label, "samples": output })
            }
            task => return Err(format!("Unknown planar and vector motion task: {task}")),
        };
        self.encode(
            "json",
            serde_json::to_vec(&result).map_err(|err| err.to_string())?,
        )
    }

    fn planar_track(&mut self, corners: &Value, frames: &Value) -> Result<Json, String> {
        let corners = list(corners)?
            .iter()
            .map(vector::<2>)
            .collect::<Result<Vec<_>, _>>()?;
        let corners: [[f64; 2]; 4] = corners
            .try_into()
            .map_err(|_| "Corner pin requires four ordered points")?;
        let frames = list(frames)?;
        if !(2..=240).contains(&frames.len()) {
            return Err("Corner pin requires 2 to 240 RGBA8 frames".into());
        }
        let first = self.rgba(&frames[0])?;
        let (width, height) = (first.width as usize, first.height as usize);
        if width < 16 || height < 16 {
            return Err("Corner pin frames must be at least 16 by 16 pixels".into());
        }
        let pixels = width * height;
        let work = pixels.saturating_mul(128).saturating_add(
            frames
                .len()
                .saturating_sub(1)
                .saturating_mul(pixels.saturating_mul(12).saturating_add(45_000)),
        );
        self.charge(
            work,
            pixels.saturating_mul(64).saturating_add(frames.len() * 512),
        )?;

        let mut previous = None;
        let mut current_corners = corners;
        let mut track = Vec::with_capacity(frames.len());
        for (frame, value) in frames.iter().enumerate() {
            let img = self.rgba(value)?;
            if (img.width as usize, img.height as usize) != (width, height) {
                return Err("Corner pin frame dimensions differ".into());
            }
            let gray = GrayImage::from_luma(width, height, img.pixels.as_flattened());
            let within = |point: [f64; 2]| {
                point[0] >= 4.0
                    && point[1] >= 4.0
                    && point[0] < (width - 4) as f64
                    && point[1] < (height - 4) as f64
            };
            let pyramid = ImagePyramid::build(&gray, 3);
            if let Some(prior) = &previous {
                for point in &mut current_corners {
                    let (x, y, converged) =
                        track_klt_point(prior, &pyramid, point[0], point[1], 3, 24)
                            .ok_or_else(|| format!("Corner lost in frame {frame}"))?;
                    if !converged || !within([x, y]) {
                        return Err(format!("Corner tracking failed in frame {frame}"));
                    }
                    *point = [x, y];
                }
            } else {
                let features = detect_shi_tomasi_corners(&gray, 128, 0.01, 3.0);
                if current_corners.iter().any(|&point| !within(point)) || features.is_empty() {
                    return Err(
                        "Corner pin needs texture and four points inside the frame margin".into(),
                    );
                }
            }
            let homography = solve_corner_pin_homography(&corners, &current_corners)
                .ok_or_else(|| format!("Degenerate corner pin in frame {frame}"))?;
            if homography.0.iter().flatten().any(|v| !v.is_finite()) {
                return Err(format!("Non-finite corner pin in frame {frame}"));
            }
            track.push(
                json!({ "frame": frame, "homography": homography.0, "corners": current_corners }),
            );
            previous = Some(pyramid);
        }
        Ok(
            json!({ "schema": "qualia-corner-pin-track/1", "width": width, "height": height, "samples": track }),
        )
    }

    pub(crate) fn simulate_control_blocks_and_cpm(&mut self, args: &Args) -> Result<Value, String> {
        let result = match text(required(args, "mode")?)? {
            "control" => {
                fields(args, &["mode", "document"])?;
                let source = native_document(self, required(args, "document")?, &["json"])?;
                control_trajectory(self, &source)?
            }
            "cpm" => {
                fields(
                    args,
                    &["mode", "document", "level_resources", "trials", "seed"],
                )?;
                let source = native_document(self, required(args, "document")?, &["json"])?;
                project_schedule(self, args, &source)?
            }
            mode => return Err(format!("Unknown engineering simulation mode: {mode}")),
        };
        self.encode(
            "json",
            serde_json::to_vec(&result).map_err(|err| err.to_string())?,
        )
    }
}

fn native_document(
    host: &mut AssetHost<'_>,
    value: &Value,
    kinds: &[&str],
) -> Result<String, String> {
    let input = encoded_resource(host, value)?;
    if !kinds.contains(&input.kind.as_str()) || input.bytes.len() > 2 * 1024 * 1024 {
        return Err("Unexpected engineering document format or size".into());
    }
    let bytes = input.bytes.len();
    host.charge(bytes.saturating_mul(8), bytes.saturating_mul(8))?;
    String::from_utf8(encoded_resource(host, value)?.bytes.clone())
        .map_err(|_| "Engineering document must use UTF-8".into())
}

fn sample_times(value: &Value) -> Result<Vec<f64>, String> {
    let values = list(value)?;
    if !(1..=1024).contains(&values.len()) {
        return Err("Supply 1 to 1024 sample times".into());
    }
    let times = values.iter().map(num).collect::<Result<Vec<_>, _>>()?;
    if times[0] < 0.0 || times.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err("Sample times must be nonnegative and strictly increasing".into());
    }
    Ok(times)
}

fn control_trajectory(host: &mut AssetHost<'_>, source: &str) -> Result<Json, String> {
    use kinetics_block_simulation::{Document, Sim, Solver, StandardBlockFactory};

    let document = Document::parse(source).map_err(|err| err.to_string())?;
    let options = document.solver;
    let step_count = (options.tstop / options.dt).ceil();
    if options.dt < 1e-9 || options.tstop < 1e-9 || !step_count.is_finite() || step_count > 10_000.0 {
        return Err(
            "Control simulation requires positive dt and tstop and at most 10000 steps".into(),
        );
    }
    if !matches!(options.solver, Solver::Euler | Solver::Rk4) {
        return Err("Control trajectory supports Euler and RK4".into());
    }
    let model = document
        .to_model(&StandardBlockFactory)
        .map_err(|err| err.to_string())?;
    let compiled = model.compile().map_err(|err| err.to_string())?;
    let rows = step_count as usize + 2;
    let lanes = compiled.total_states;
    let blocks = compiled.blocks.len();
    host.charge(
        rows.saturating_mul(lanes + blocks).saturating_mul(128),
        rows.saturating_mul(lanes + 1).saturating_mul(48),
    )?;
    let spans: Vec<_> = compiled
        .blocks
        .iter()
        .enumerate()
        .filter_map(|(index, block)| {
            let width = block.n_states();
            (width != 0).then(|| json!({
            "block": compiled.names[index], "offset": compiled.state_offsets[index], "width": width
        }))
        })
        .collect();
    let mut states = Vec::with_capacity(rows);
    let mut sim = Sim::new(compiled, options).map_err(|err| err.to_string())?;
    let result = sim
        .run_with_state_observer(|_, state| {
            if state.iter().any(|v| !v.is_finite()) {
                return Err(kinetics_block_simulation::SimError::Msg(
                    "Non-finite control state".into(),
                ));
            }
            states.push(state.to_vec());
            Ok(())
        })
        .map_err(|err| err.to_string())?;
    Ok(json!({
        "schema": "qualia-control-trajectory/1", "time_s": result.t, "state_spans": spans,
        "states": states, "solver": format!("{:?}", result.solver), "steps": result.steps,
    }))
}

fn project_schedule(host: &mut AssetHost<'_>, args: &Args, source: &str) -> Result<Json, String> {
    use kinetics_schedule_cpm::{from_json, level_resources, run_monte_carlo, schedule_project};

    let mut project = from_json(source).map_err(|err| err.to_string())?;
    let trials = args
        .get("trials")
        .map(|value| integer(value, "trials"))
        .transpose()?
        .unwrap_or(0);
    if trials > 20_000 {
        return Err("CPM supports at most 20000 Monte Carlo trials".into());
    }
    let seed = match args.get("seed") {
        None => 0,
        Some(Value::U64(seed)) => *seed,
        Some(Value::I64(seed)) if *seed >= 0 => *seed as u64,
        _ => return Err("CPM seed must be a nonnegative integer".into()),
    };
    let level = boolean(args, "level_resources", false)?;
    let tasks = project.tasks.len();
    if tasks == 0 || tasks > 4096 {
        return Err("CPM requires 1 to 4096 tasks".into());
    }
    let cost = tasks + project.dependencies.len() + project.assignments.len();
    let iterations = trials + if level { (tasks * 8).clamp(8, 256) } else { 1 };
    host.charge(
        cost.saturating_mul(iterations).saturating_mul(64),
        cost * 512 + trials * 16,
    )?;
    let leveling = if level {
        let result = level_resources(&mut project).map_err(|err| err.to_string())?;
        Some(json!({
            "before_overallocated_days": result.before_overallocated_days,
            "after_overallocated_days": result.after_overallocated_days,
            "actions": result.actions.len(),
        }))
    } else {
        None
    };
    let summary = schedule_project(&mut project).map_err(|err| err.to_string())?;
    let risk = if trials != 0 {
        let result = run_monte_carlo(&project, trials, seed).map_err(|err| err.to_string())?;
        Some(json!({
            "trials": result.trials, "seed": result.seed, "mean_days": result.mean_duration_days,
            "std_dev_days": result.std_dev_days, "p10_days": result.p10_days, "p50_days": result.p50_days,
            "p80_days": result.p80_days, "p90_days": result.p90_days,
        }))
    } else {
        None
    };
    Ok(json!({
        "schema": "qualia-cpm-pert/1", "duration_days": summary.project_duration_days,
        "critical_task_ids": summary.critical_task_ids, "topological_order": summary.topological_order,
        "planned_cost": summary.total_planned_cost, "resource_leveling": leveling,
        "risk": risk, "tasks": project.tasks,
    }))
}
