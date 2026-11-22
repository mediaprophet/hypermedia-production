//! Bounded scene-linear planar and deep compositing.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::sync::Arc;

use crate::artifact::{AovPlane, FieldLattice, LatticeSamples, RadianceLattice};
use crate::radiance_compositor_gi::cryptomatte_murmur3;
use crate::*;
use color_transforms::to_linear;
use raster_blend_operators::{BlendMode, composite};

const REC709: &str = "http://www.w3.org/ns/ttml/parameter/rec709";
const LINEAR: &str = "urn:qualia:color:transfer:linear";
const MAX_NODES: usize = 32;
const MAX_DEEP_SAMPLES: usize = 64;
type Pixel = [f32; 4];

impl AssetHost<'_> {
    pub(crate) fn composite_aov_graph(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &["width", "height", "planes", "bundle", "nodes", "output"],
        )?;
        let width = count(args, "width", 0, 4096)?;
        let height = count(args, "height", 0, 4096)?;
        let count_pixels = width * height;
        let nodes = list(required(args, "nodes")?)?;
        if !(1..=MAX_NODES).contains(&nodes.len()) {
            return Err("AOV graph requires 1 to 32 ordered nodes".into());
        }
        let planes = plane_source(self, args)?;
        let mut max_deep_work = 0usize;
        let mut shutter_work = 0usize;
        for raw in nodes {
            let node = record(raw)?;
            match text(required(node, "op")?)? {
                "deep_flatten" => {
                    max_deep_work = max_deep_work.saturating_add(planes.deep_extent(node)?);
                }
                "vector_blur" => {
                    let shutter_samples = count(node, "samples", 9, 32)?;
                    shutter_work = shutter_work
                        .saturating_add(shutter_samples.saturating_mul(count_pixels).saturating_mul(64));
                }
                _ => {}
            }
        }
        self.charge(
            nodes.len().saturating_mul(count_pixels).saturating_mul(32)
                .saturating_add(shutter_work)
                .saturating_add(max_deep_work.saturating_mul(64)),
            nodes.len().saturating_mul(count_pixels).saturating_mul(20)
                .saturating_add(max_deep_work.saturating_mul(8)),
        )?;
        let planes = plane_source(self, args)?;
        let mut named = BTreeMap::<&str, Vec<Pixel>>::new();
        for raw in nodes {
            let node = record(raw)?;
            let id = text(required(node, "id")?)?;
            if id.is_empty() || id.len() > 128 || named.contains_key(id) {
                return Err("Compositor node IDs must be nonempty, bounded, and unique".into());
            }
            let image = |key| {
                let name = text(required(node, key)?)?;
                named
                    .get(name)
                    .map(Vec::as_slice)
                    .ok_or_else(|| format!("Unknown earlier node {name}"))
            };
            let pixels = match text(required(node, "op")?)? {
                "plane" => {
                    fields(node, &["id", "op", "plane"])?;
                    let selected = text(required(node, "plane")?)?;
                    planes.planar_color(self, selected, width, height)?
                }
                "blend" => {
                    fields(node, &["id", "op", "back", "front", "mode"])?;
                    let mode = match node.get("mode").map(text).transpose()?.unwrap_or("over") {
                        "over" => BlendMode::Normal,
                        "multiply" => BlendMode::Multiply,
                        "screen" => BlendMode::Screen,
                        "difference" => BlendMode::Difference,
                        _ => return Err("Unknown AOV blend mode".into()),
                    };
                    image("back")?
                        .iter()
                        .zip(image("front")?)
                        .map(|(&back, &front)| composite(mode, back, front))
                        .collect()
                }
                "cryptomatte" => {
                    fields(node, &["id", "op", "input", "ids", "name"])?;
                    let ids =
                        planes.data_plane(self, text(required(node, "ids")?)?, width, height)?;
                    if ids.channels == 0 || ids.channels > 16 || ids.channels % 2 != 0 {
                        return Err(
                            "Cryptomatte requires one or more float ID/coverage pairs per pixel"
                                .into(),
                        );
                    }
                    let target = cryptomatte_id(text(required(node, "name")?)?);
                    let mut output = Vec::with_capacity(count_pixels);
                    for (&pixel, pairs) in image("input")?
                        .iter()
                        .zip(ids.values.chunks_exact(ids.channels))
                    {
                        let mut coverage = 0.0;
                        for pair in pairs.chunks_exact(2) {
                            if !pair[0].is_finite() || !(0.0..=1.0).contains(&pair[1]) {
                                return Err("Invalid Cryptomatte ID or coverage".into());
                            }
                            if pair[0].to_bits() == target {
                                coverage += pair[1];
                            }
                        }
                        let [r, g, b, a] = pixel;
                        output.push([r, g, b, a * coverage.min(1.0)]);
                    }
                    output
                }
                "vector_blur" => {
                    fields(
                        node,
                        &["id", "op", "input", "velocity", "samples", "shutter"],
                    )?;
                    let velocity = planes.data_plane(
                        self,
                        text(required(node, "velocity")?)?,
                        width,
                        height,
                    )?;
                    if velocity.channels != 2
                        || velocity
                            .values
                            .iter()
                            .any(|v| !v.is_finite() || v.abs() > 8192.0)
                    {
                        return Err(
                            "Motion vectors need finite XY displacement in pixel units".into()
                        );
                    }
                    let samples = count(node, "samples", 9, 32)?;
                    let shutter = number(node, "shutter", 1.0)? as f32;
                    if !(0.0..=2.0).contains(&shutter) {
                        return Err("Shutter interval is outside 0 to 2 frame units".into());
                    }
                    vector_blur(
                        image("input")?,
                        velocity.values,
                        width,
                        height,
                        samples,
                        shutter,
                    )
                }
                "deep_flatten" => {
                    fields(node, &["id", "op", "plane"])?;
                    planes.deep_flatten(text(required(node, "plane")?)?, count_pixels)?
                }
                op => return Err(format!("Unknown AOV graph operation {op}")),
            };
            validate_pixels(&pixels)?;
            named.insert(id, pixels);
        }
        let output = text(required(args, "output")?)?;
        let pixels = named
            .remove(output)
            .ok_or("AOV graph output does not name an evaluated node")?;
        self.admit_artifact(TypedArtifact::RadianceLattice(Arc::new(RadianceLattice {
            width: width as u32,
            height: height as u32,
            channels: 4,
            primaries_iri: REC709.into(),
            transfer_iri: LINEAR.into(),
            samples: LatticeSamples::F32(pixels.into_flattened()),
        })))
    }
}

enum PlaneSource<'a> {
    Inputs(&'a Args),
    Native(&'a BTreeMap<String, AovPlane>),
}

fn plane_source<'a>(host: &'a AssetHost<'_>, args: &'a Args) -> Result<PlaneSource<'a>, String> {
    match (args.get("planes"), args.get("bundle")) {
        (Some(planes), None) => Ok(PlaneSource::Inputs(record(planes)?)),
        (None, Some(handle)) => match host.workspace.typed(handle)? {
            TypedArtifact::AovBundle(bundle) => Ok(PlaneSource::Native(&bundle.planes)),
            _ => Err("Expected typed AOV bundle handle".into()),
        },
        _ => Err("Supply named planes or a typed bundle".into()),
    }
}

struct Data<'a> {
    channels: usize,
    values: &'a [f32],
}

impl PlaneSource<'_> {
    fn planar<'a>(
        &'a self,
        host: &'a AssetHost<'_>,
        name: &str,
    ) -> Result<&'a RadianceLattice, String> {
        match self {
            Self::Inputs(planes) => {
                let TypedArtifact::RadianceLattice(lattice) =
                    host.workspace.typed(required(planes, name)?)?
                else {
                    return Err(format!(
                        "AOV plane {name} requires a typed radiance lattice"
                    ));
                };
                Ok(lattice)
            }
            Self::Native(planes) => {
                let Some(AovPlane::Planar(lattice)) = planes.get(name) else {
                    return Err(format!("AOV plane {name} is absent or not planar"));
                };
                Ok(lattice)
            }
        }
    }

    fn planar_color(
        &self,
        host: &AssetHost<'_>,
        name: &str,
        w: usize,
        h: usize,
    ) -> Result<Vec<Pixel>, String> {
        if let Self::Inputs(planes) = self {
            let handle = required(planes, name)?;
            if let Ok(Artifact::Rgba(image)) = host.workspace.get(handle) {
                if image.width as usize != w || image.height as usize != h {
                    return Err("RGBA8 AOV dimensions differ from compositor dimensions".into());
                }
                return Ok(image
                    .pixels
                    .iter()
                    .map(|p| {
                        let [r, g, b, a] = p.map(|v| f32::from(v) / 255.0);
                        let tf = color_transforms::Transfer::Srgb;
                        [to_linear(r, tf), to_linear(g, tf), to_linear(b, tf), a]
                    })
                    .collect());
            }
        }
        let lattice = self.planar(host, name)?;
        require_color_frame(lattice)?;
        let data = data(lattice, w, h)?;
        if data.channels != 4 {
            return Err("Color AOVs require four linear RGBA channels".into());
        }
        Ok(data
            .values
            .chunks_exact(4)
            .map(|row| [row[0], row[1], row[2], row[3]])
            .collect())
    }

    fn data_plane<'a>(
        &'a self,
        host: &'a AssetHost<'_>,
        name: &str,
        w: usize,
        h: usize,
    ) -> Result<Data<'a>, String> {
        if let Self::Inputs(planes) = self {
            let value = required(planes, name)?;
            if let Ok(TypedArtifact::FieldLattice(lattice)) = host.workspace.typed(value) {
                let FieldLattice::DenseF32 { axes, samples } = lattice.as_ref() else {
                    return Err("Data AOVs require dense float32 fields".into());
                };
                let [height, width, channels] = axes.as_slice() else {
                    return Err(
                        "Data AOV fields require height, width and channel dimensions".into(),
                    );
                };
                let channels = *channels as usize;
                if *width as usize != w
                    || *height as usize != h
                    || channels == 0
                    || channels > 16
                    || samples.len() != w * h * channels
                {
                    return Err(
                        "Data AOV field dimensions differ from the compositor extent".into(),
                    );
                }
                return Ok(Data {
                    channels,
                    values: samples,
                });
            }
        }
        data(self.planar(host, name)?, w, h)
    }

    fn deep<'a>(&'a self, name: &str) -> Result<(&'a [u32], &'a RadianceLattice), String> {
        let Self::Native(planes) = self else {
            return Err("Deep flattening requires a typed deep AOV plane".into());
        };
        let Some(AovPlane::Deep {
            sample_offsets,
            samples,
        }) = planes.get(name)
        else {
            return Err(format!("Missing deep AOV plane {name}"));
        };
        Ok((sample_offsets, samples))
    }

    fn deep_extent(&self, node: &Args) -> Result<usize, String> {
        let (_, lattice) = self.deep(text(required(node, "plane")?)?)?;
        let LatticeSamples::F32(values) = &lattice.samples else {
            return Err("Deep AOV requires a float32 sample stream".into());
        };
        Ok(values.len())
    }

    fn deep_flatten(&self, name: &str, count: usize) -> Result<Vec<Pixel>, String> {
        let (offsets, lattice) = self.deep(name)?;
        require_color_frame(lattice)?;
        let LatticeSamples::F32(values) = &lattice.samples else {
            return Err("Deep AOV requires a float32 sample stream".into());
        };
        if lattice.channels != 5
            || offsets.len() != count + 1
            || offsets[0] != 0
            || (offsets[count] as usize).checked_mul(5) != Some(values.len())
        {
            return Err("Deep AOV requires per-pixel prefix offsets and RGBAZ records".into());
        }
        let mut result = Vec::with_capacity(count);
        let mut layer = [([0.0; 4], 0.0_f32); MAX_DEEP_SAMPLES];
        for span in offsets.windows(2) {
            let (first, end) = (span[0] as usize, span[1] as usize);
            if end < first || end - first > MAX_DEEP_SAMPLES || end > values.len() / 5 {
                return Err("Deep AOV sample span exceeds the layer budget".into());
            }
            let layer = &mut layer[..end - first];
            for (item, source) in layer
                .iter_mut()
                .zip(values[first * 5..end * 5].chunks_exact(5))
            {
                if !source[4].is_finite() || source[4] < 0.0 {
                    return Err("Deep AOV depth must be finite and nonnegative".into());
                }
                *item = ([source[0], source[1], source[2], source[3]], source[4]);
                validate_pixels(std::slice::from_ref(&item.0))?;
            }
            layer.sort_by(|a, b| b.1.total_cmp(&a.1));
            result.push(layer.iter().fold([0.0; 4], |back, &(front, _)| {
                composite(BlendMode::Normal, back, front)
            }));
        }
        Ok(result)
    }
}

fn data(lattice: &RadianceLattice, w: usize, h: usize) -> Result<Data<'_>, String> {
    let channels = usize::from(lattice.channels);
    let LatticeSamples::F32(values) = &lattice.samples else {
        return Err("This AOV plane requires float32 samples".into());
    };
    if lattice.width as usize != w
        || lattice.height as usize != h
        || channels == 0
        || values.len() != w * h * channels
    {
        return Err("AOV plane does not match the compositor dimensions".into());
    }
    Ok(Data { channels, values })
}

fn require_color_frame(lattice: &RadianceLattice) -> Result<(), String> {
    if lattice.primaries_iri != REC709 || lattice.transfer_iri != LINEAR {
        return Err(
            "Convert typed color planes to the linear BT.709 compositor space first".into(),
        );
    }
    Ok(())
}

fn validate_pixels(pixels: &[Pixel]) -> Result<(), String> {
    if pixels.iter().any(|p| {
        p[..3].iter().any(|v| !v.is_finite() || v.abs() > 1e12) || !(0.0..=1.0).contains(&p[3])
    }) {
        return Err("Invalid scene-linear color or straight alpha in AOV graph".into());
    }
    Ok(())
}

fn cryptomatte_id(name: &str) -> u32 {
    let hash = cryptomatte_murmur3(name.as_bytes());
    let exponent = (hash >> 23) & 0xff;
    if exponent == 0 || exponent == 255 {
        hash ^ (1 << 23)
    } else {
        hash
    }
}

fn vector_blur(
    pixels: &[Pixel],
    flow: &[f32],
    w: usize,
    h: usize,
    samples: usize,
    shutter: f32,
) -> Vec<Pixel> {
    flow.chunks_exact(2)
        .enumerate()
        .map(|(i, velocity)| {
            let (x, y) = ((i % w) as f32, (i / w) as f32);
            let mut sum = [0.0_f64; 4];
            for j in 0..samples {
                let t = ((j as f32 + 0.5) / samples as f32 - 0.5) * shutter;
                let x = (x + t * velocity[0]).clamp(0.0, (w - 1) as f32);
                let y = (y + t * velocity[1]).clamp(0.0, (h - 1) as f32);
                let (x0, y0) = (x.floor() as usize, y.floor() as usize);
                let (x1, y1) = ((x0 + 1).min(w - 1), (y0 + 1).min(h - 1));
                let (dx, dy) = (f64::from(x - x0 as f32), f64::from(y - y0 as f32));
                for (p, weight) in [
                    (pixels[y0 * w + x0], (1.0 - dx) * (1.0 - dy)),
                    (pixels[y0 * w + x1], dx * (1.0 - dy)),
                    (pixels[y1 * w + x0], (1.0 - dx) * dy),
                    (pixels[y1 * w + x1], dx * dy),
                ] {
                    let weighted_alpha = f64::from(p[3]) * weight;
                    for lane in 0..3 {
                        sum[lane] += f64::from(p[lane]) * weighted_alpha;
                    }
                    sum[3] += weighted_alpha;
                }
            }
            if sum[3] > 0.0 {
                let rgb = [sum[0], sum[1], sum[2]].map(|v| (v / sum[3]) as f32);
                [
                    rgb[0],
                    rgb[1],
                    rgb[2],
                    (sum[3] / samples as f64).clamp(0.0, 1.0) as f32,
                ]
            } else {
                [0.0; 4]
            }
        })
        .collect()
}
