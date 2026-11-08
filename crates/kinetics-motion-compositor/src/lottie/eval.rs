//! Evaluation of Lottie animated properties at arbitrary frame times.

use serde_json::Value;

use crate::lottie::schema::{LottieLayer, LottiePropMulti, LottiePropScalar, LottieTransform};

/// Evaluated 2D affine transform of a layer at a specific frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EvaluatedTransform {
    pub position: [f64; 2],
    pub anchor: [f64; 2],
    pub scale: [f64; 2],
    pub rotation_deg: f64,
    pub opacity: f64,
}

/// Evaluate a scalar property at frame `f`.
pub fn eval_scalar_prop(prop: &LottiePropScalar, frame: f64) -> f64 {
    if prop.a == 0 {
        if let Some(v) = prop.k.as_f64() {
            return v;
        }
        if let Some(arr) = prop.k.as_array() {
            if let Some(v) = arr.first().and_then(Value::as_f64) {
                return v;
            }
        }
        return 0.0;
    }

    // Keyframed scalar
    if let Some(keys) = prop.k.as_array() {
        if keys.is_empty() {
            return 0.0;
        }
        let first_t = keys[0].get("t").and_then(Value::as_f64).unwrap_or(0.0);
        if frame <= first_t {
            return extract_val(&keys[0], "s");
        }
        let last_idx = keys.len() - 1;
        let last_t = keys[last_idx].get("t").and_then(Value::as_f64).unwrap_or(first_t);
        if frame >= last_t {
            return extract_val(&keys[last_idx], "s");
        }

        for i in 0..last_idx {
            let t0 = keys[i].get("t").and_then(Value::as_f64).unwrap_or(0.0);
            let t1 = keys[i + 1].get("t").and_then(Value::as_f64).unwrap_or(t0);
            if frame >= t0 && frame <= t1 {
                let dt = t1 - t0;
                let progress = if dt > 0.0 { (frame - t0) / dt } else { 0.0 };
                let v0 = extract_val(&keys[i], "s");
                let v1 = extract_val(&keys[i], "e");
                return v0 + (v1 - v0) * progress;
            }
        }
    }

    0.0
}

fn extract_val(key: &Value, prop_name: &str) -> f64 {
    if let Some(v) = key.get(prop_name) {
        if let Some(f) = v.as_f64() {
            return f;
        }
        if let Some(arr) = v.as_array() {
            if let Some(f) = arr.first().and_then(Value::as_f64) {
                return f;
            }
        }
    }
    0.0
}

/// Evaluate a 2D multi-dimensional property at frame `f`.
pub fn eval_multi_prop(prop: &LottiePropMulti, frame: f64) -> [f64; 2] {
    if prop.a == 0 {
        if let Some(arr) = prop.k.as_array() {
            let x = arr.get(0).and_then(Value::as_f64).unwrap_or(0.0);
            let y = arr.get(1).and_then(Value::as_f64).unwrap_or(0.0);
            return [x, y];
        }
        return [0.0, 0.0];
    }

    // Keyframed multi
    if let Some(keys) = prop.k.as_array() {
        if keys.is_empty() {
            return [0.0, 0.0];
        }
        let first_t = keys[0].get("t").and_then(Value::as_f64).unwrap_or(0.0);
        if frame <= first_t {
            return extract_multi_val(&keys[0], "s");
        }
        let last_idx = keys.len() - 1;
        let last_t = keys[last_idx].get("t").and_then(Value::as_f64).unwrap_or(first_t);
        if frame >= last_t {
            return extract_multi_val(&keys[last_idx], "s");
        }

        for i in 0..last_idx {
            let t0 = keys[i].get("t").and_then(Value::as_f64).unwrap_or(0.0);
            let t1 = keys[i + 1].get("t").and_then(Value::as_f64).unwrap_or(t0);
            if frame >= t0 && frame <= t1 {
                let dt = t1 - t0;
                let progress = if dt > 0.0 { (frame - t0) / dt } else { 0.0 };
                let v0 = extract_multi_val(&keys[i], "s");
                let v1 = extract_multi_val(&keys[i], "e");
                return [
                    v0[0] + (v1[0] - v0[0]) * progress,
                    v0[1] + (v1[1] - v0[1]) * progress,
                ];
            }
        }
    }

    [0.0, 0.0]
}

fn extract_multi_val(key: &Value, prop_name: &str) -> [f64; 2] {
    if let Some(arr) = key.get(prop_name).and_then(Value::as_array) {
        let x = arr.get(0).and_then(Value::as_f64).unwrap_or(0.0);
        let y = arr.get(1).and_then(Value::as_f64).unwrap_or(0.0);
        return [x, y];
    }
    [0.0, 0.0]
}

/// Evaluate the transform of a layer at frame `f`.
pub fn eval_layer_transform(layer: &LottieLayer, frame: f64) -> EvaluatedTransform {
    let local_frame = (frame - layer.st) * layer.sr;
    let anchor = eval_multi_prop(&layer.ks.a, local_frame);
    let position = eval_multi_prop(&layer.ks.p, local_frame);
    let mut scale = eval_multi_prop(&layer.ks.s, local_frame);
    if scale == [0.0, 0.0] {
        scale = [100.0, 100.0];
    }
    let rotation = eval_scalar_prop(&layer.ks.r, local_frame);
    let mut opacity = eval_scalar_prop(&layer.ks.o, local_frame);
    if opacity == 0.0 && layer.ks.o.k.is_null() {
        opacity = 100.0;
    }

    EvaluatedTransform {
        position,
        anchor,
        scale,
        rotation_deg: rotation,
        opacity,
    }
}
