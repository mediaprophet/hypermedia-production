//! Bodymovin / Lottie v5.7.4 JSON vector animation export.

use super::string;
use crate::*;
use vibe::Value;

struct LottieShapeDesc {
    name: String,
    color: [f64; 4],
    start_pos: [f64; 2],
    end_pos: [f64; 2],
    size: [f64; 2],
}

impl AssetHost<'_> {
    pub(super) fn export_lottie(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["width", "height", "fps", "frames", "shapes", "name"])?;

        let width = count(a, "width", 512, 4096)?;
        let height = count(a, "height", 512, 4096)?;
        let fps = count(a, "fps", 30, 120)?;
        let frames = count(a, "frames", 60, 1800)?;
        let name = string(a, "name", "hypermedia_kinetics")?;

        let mut shape_layers = Vec::new();
        if let Some(s_val) = a.get("shapes") {
            let s_list = list(s_val)?;
            for (idx, item) in s_list.iter().enumerate() {
                let rec = record(item)?;
                fields(rec, &["name", "color", "start_pos", "end_pos", "size"])?;
                let s_name = match rec.get("name") {
                    Some(v) => text(v)?.to_string(),
                    None => format!("shape_{idx}"),
                };
                let color = match rec.get("color") {
                    Some(v) => {
                        let cl = list(v)?;
                        if cl.len() == 4 {
                            [
                                num(&cl[0])?.clamp(0.0, 1.0),
                                num(&cl[1])?.clamp(0.0, 1.0),
                                num(&cl[2])?.clamp(0.0, 1.0),
                                num(&cl[3])?.clamp(0.0, 1.0),
                            ]
                        } else if cl.len() == 3 {
                            [
                                num(&cl[0])?.clamp(0.0, 1.0),
                                num(&cl[1])?.clamp(0.0, 1.0),
                                num(&cl[2])?.clamp(0.0, 1.0),
                                1.0,
                            ]
                        } else {
                            return Err("color must have 3 or 4 components".into());
                        }
                    }
                    None => [0.2, 0.6, 1.0, 1.0],
                };
                let start_pos = match rec.get("start_pos") {
                    Some(v) => vector::<2>(v)?,
                    None => [width as f64 * 0.25, height as f64 * 0.5],
                };
                let end_pos = match rec.get("end_pos") {
                    Some(v) => vector::<2>(v)?,
                    None => [width as f64 * 0.75, height as f64 * 0.5],
                };
                let size = match rec.get("size") {
                    Some(v) => {
                        let sz = vector::<2>(v)?;
                        [sz[0].max(0.0), sz[1].max(0.0)]
                    }
                    None => [100.0, 100.0],
                };
                shape_layers.push(LottieShapeDesc {
                    name: s_name,
                    color,
                    start_pos,
                    end_pos,
                    size,
                });
            }
        }

        if shape_layers.is_empty() {
            shape_layers.push(LottieShapeDesc {
                name: "shape_0".into(),
                color: [0.2, 0.6, 1.0, 1.0],
                start_pos: [width as f64 * 0.25, height as f64 * 0.5],
                end_pos: [width as f64 * 0.75, height as f64 * 0.5],
                size: [100.0, 100.0],
            });
        }

        self.charge(frames * 8, 4096)?;

        let layers: Vec<serde_json::Value> = shape_layers
            .iter()
            .enumerate()
            .map(|(idx, shape)| {
                serde_json::json!({
                    "ddd": 0,
                    "ind": (idx + 1) as i64,
                    "ty": 4,
                    "nm": shape.name,
                    "sr": 1,
                    "ks": {
                        "a": { "a": 0, "k": [0.0, 0.0, 0.0] },
                        "p": {
                            "a": 1,
                            "k": [
                                {
                                    "t": 0,
                                    "s": [shape.start_pos[0], shape.start_pos[1], 0.0],
                                    "e": [shape.end_pos[0], shape.end_pos[1], 0.0],
                                    "o": { "x": [0.167], "y": [0.167] },
                                    "i": { "x": [0.833], "y": [0.833] }
                                },
                                {
                                    "t": frames as i64,
                                    "s": [shape.end_pos[0], shape.end_pos[1], 0.0]
                                }
                            ]
                        },
                        "s": { "a": 0, "k": [100.0, 100.0, 100.0] },
                        "r": { "a": 0, "k": 0.0 },
                        "o": { "a": 0, "k": 100.0 }
                    },
                    "ao": 0,
                    "shapes": [
                        {
                            "ty": "rc",
                            "nm": "Rectangle",
                            "d": 1,
                            "s": { "a": 0, "k": [shape.size[0], shape.size[1]] },
                            "p": { "a": 0, "k": [0.0, 0.0] },
                            "r": { "a": 0, "k": 0.0 }
                        },
                        {
                            "ty": "fl",
                            "nm": "Fill",
                            "c": { "a": 0, "k": [shape.color[0], shape.color[1], shape.color[2], 1.0] },
                            "o": { "a": 0, "k": shape.color[3] * 100.0 },
                            "r": 1
                        },
                        {
                            "ty": "tr",
                            "p": { "a": 0, "k": [0.0, 0.0] },
                            "a": { "a": 0, "k": [0.0, 0.0] },
                            "s": { "a": 0, "k": [100.0, 100.0] },
                            "r": { "a": 0, "k": 0.0 },
                            "o": { "a": 0, "k": 100.0 },
                            "sk": { "a": 0, "k": 0.0 },
                            "sa": { "a": 0, "k": 0.0 },
                            "nm": "Transform"
                        }
                    ],
                    "ip": 0,
                    "op": frames as i64,
                    "st": 0,
                    "bm": 0
                })
            })
            .collect();

        let lottie_json = serde_json::json!({
            "v": "5.7.4",
            "fr": fps as f64,
            "ip": 0,
            "op": frames as i64,
            "w": width as f64,
            "h": height as f64,
            "nm": name,
            "ddd": 0,
            "assets": [],
            "layers": layers
        });

        let bytes = serde_json::to_vec(&lottie_json)
            .map_err(|e| format!("Failed to serialize Lottie JSON: {e}"))?;

        self.encode("json", bytes)
    }
}
