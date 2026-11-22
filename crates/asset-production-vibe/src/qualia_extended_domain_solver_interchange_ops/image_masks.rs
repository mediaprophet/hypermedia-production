//! Deterministic CIELAB superpixels and halo-correct scalar guided filtering.

use crate::{
    artifact::{FieldLattice, LatticeSamples, RadianceLattice},
    fields, integer, num, required, Args, AssetHost, TypedArtifact, Value,
};
use color_transforms::{icc::math::xyz_to_lab, to_linear, Transfer};
use std::{collections::BTreeMap, sync::Arc};

const MAX_PIXELS: usize = 1_048_576;
const MAX_DIMENSION: usize = 4096;
const REC709: &str = "http://www.w3.org/ns/ttml/parameter/rec709";
const SRGB: &str = "http://www.w3.org/ns/ttml/parameter/sRGB";
const LINEAR: &str = "http://www.w3.org/ns/ttml/parameter/linear";
const UNASSIGNED: u32 = u32::MAX;
const PENDING: u32 = u32::MAX - 1;
const FALLBACK_COMPARISONS_PER_PIXEL: usize = 8;

pub(super) fn evaluate(host: &mut AssetHost<'_>, op: &str, args: &Args) -> Result<Value, String> {
    match op {
        "slic_superpixels" => superpixels(host, args),
        "guided_filter" => guided_filter(host, args),
        _ => Err(format!("Unsupported image mask operation {op}")),
    }
}

#[derive(Clone, Copy)]
struct Grid {
    width: usize,
    height: usize,
}

impl Grid {
    fn for_lattice(lattice: &RadianceLattice) -> Result<Self, String> {
        let width = lattice.width as usize;
        let height = lattice.height as usize;
        if !(1..=MAX_DIMENSION).contains(&width)
            || !(1..=MAX_DIMENSION).contains(&height)
            || width * height > MAX_PIXELS
        {
            return Err("Image exceeds the bounded RadianceLattice dimensions".into());
        }
        Ok(Self { width, height })
    }

    fn len(self) -> usize {
        self.width * self.height
    }
    fn axes(self) -> Vec<u32> {
        vec![self.height as u32, self.width as u32]
    }
}

fn lattice<'a>(host: &'a AssetHost<'_>, value: &Value) -> Result<&'a RadianceLattice, String> {
    match host.workspace.typed(value)? {
        TypedArtifact::RadianceLattice(lattice) => Ok(lattice),
        _ => Err("Expected a typed RadianceLattice handle".into()),
    }
}

fn scalar(params: &Args, key: &str, default: f64, bounds: [f64; 2]) -> Result<f64, String> {
    let value = params.get(key).map(num).transpose()?.unwrap_or(default);
    (bounds[0]..=bounds[1])
        .contains(&value)
        .then_some(value)
        .ok_or_else(|| format!("{key} falls outside its supported range"))
}

fn count(params: &Args, key: &str, default: usize, bounds: [usize; 2]) -> Result<usize, String> {
    let value = params
        .get(key)
        .map(|v| integer(v, key))
        .transpose()?
        .unwrap_or(default);
    (bounds[0]..=bounds[1])
        .contains(&value)
        .then_some(value)
        .ok_or_else(|| format!("{key} falls outside its supported range"))
}

fn sample_len(samples: &LatticeSamples) -> usize {
    match samples {
        LatticeSamples::U8(values) => values.len(),
        LatticeSamples::U16(values) | LatticeSamples::F16Bits(values) => values.len(),
        LatticeSamples::F32(values) => values.len(),
    }
}

fn half_to_f32(bits: u16) -> f32 {
    let sign = u32::from(bits & 0x8000) << 16;
    let exponent = u32::from((bits >> 10) & 31);
    let fraction = u32::from(bits & 1023);
    match exponent {
        0 => f32::from_bits(sign | 0x3f80_0000) * (fraction as f32 / 16_777_216.0),
        31 => f32::from_bits(sign | 0x7f80_0000 | (fraction << 13)),
        exponent => f32::from_bits(sign | ((exponent + 112) << 23) | (fraction << 13)),
    }
}

fn sample(samples: &LatticeSamples, index: usize) -> Result<f32, String> {
    let signal = match samples {
        LatticeSamples::U8(values) => f32::from(values[index]) / 255.0,
        LatticeSamples::U16(values) => f32::from(values[index]) / 65_535.0,
        LatticeSamples::F16Bits(values) => half_to_f32(values[index]),
        LatticeSamples::F32(values) => values[index],
    };
    (0.0..=1.0)
        .contains(&signal)
        .then_some(signal)
        .ok_or_else(|| "Image or mask sample is outside finite [0, 1]".into())
}

fn colour(input: &RadianceLattice, grid: Grid) -> Result<(Transfer, usize), String> {
    let channels = usize::from(input.channels);
    if !matches!(channels, 3 | 4)
        || input.primaries_iri != REC709
        || sample_len(&input.samples) != grid.len() * channels
    {
        return Err("Expected an RGB or opaque RGBA RadianceLattice with Rec.709 primaries".into());
    }
    let transfer = match input.transfer_iri.as_str() {
        SRGB => Transfer::Srgb,
        LINEAR => Transfer::Linear,
        _ => return Err("Image mask colour requires sRGB or linear transfer".into()),
    };
    Ok((transfer, channels))
}

fn linear_rgb(
    input: &RadianceLattice,
    index: usize,
    transfer: Transfer,
    channels: usize,
) -> Result<[f64; 3], String> {
    let base = index * channels;
    if channels == 4 && sample(&input.samples, base + 3)? != 1.0 {
        return Err("Composite transparent image pixels before computing masks".into());
    }
    let [r, g, b] = [
        sample(&input.samples, base)?,
        sample(&input.samples, base + 1)?,
        sample(&input.samples, base + 2)?,
    ];
    Ok([r, g, b].map(|v| f64::from(to_linear(v, transfer))))
}

fn field(host: &mut AssetHost<'_>, grid: Grid, samples: Vec<f32>) -> Result<Value, String> {
    host.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
        FieldLattice::DenseF32 {
            axes: grid.axes(),
            samples,
        },
    )))
}

fn superpixels(host: &mut AssetHost<'_>, args: &Args) -> Result<Value, String> {
    fields(
        args,
        &["image", "target_count", "compactness", "iterations"],
    )?;
    let image = required(args, "image")?;
    let grid = Grid::for_lattice(lattice(host, image)?)?;
    let target_count = count(
        args,
        "target_count",
        256.min(grid.len()),
        [1, 4096.min(grid.len())],
    )?;
    let compactness = scalar(args, "compactness", 10.0, [0.1, 100.0])?;
    let iterations = count(args, "iterations", 10, [1, 20])?;
    let step = ((grid.len() as f64 / target_count as f64).sqrt().round() as usize).max(1);
    let x_count = grid.width.div_ceil(step);
    let y_count = grid.height.div_ceil(step);
    let centre_count = x_count * y_count;
    if centre_count > 8192 {
        return Err("SLIC seed grid exceeds the centre budget".into());
    }
    let window = (2 * step + 1).min(grid.width) * (2 * step + 1).min(grid.height);
    host.charge(
        centre_count * window * iterations * 12
            + grid.len() * (48 + 8 * iterations + 12 * FALLBACK_COMPARISONS_PER_PIXEL),
        grid.len() * 56 + centre_count * 128,
    )?;
    let input = lattice(host, image)?;
    let (transfer, channels) = colour(input, grid)?;
    let mut lab = Vec::with_capacity(grid.len());
    for index in 0..grid.len() {
        let [r, g, b] = linear_rgb(input, index, transfer, channels)?;
        let xyz = [
            0.412_456_4 * r + 0.357_576_1 * g + 0.180_437_5 * b,
            0.212_672_9 * r + 0.715_152_2 * g + 0.072_175 * b,
            0.019_333_9 * r + 0.119_192 * g + 0.950_304_1 * b,
        ];
        lab.push(xyz_to_lab(xyz, [0.950_47, 1.0, 1.088_83]));
    }
    let (labels, count, uncovered) = slic(grid, &lab, step, compactness, iterations)?;
    let labels = field(
        host,
        grid,
        labels.into_iter().map(|label| label as f32).collect(),
    )?;
    Ok(Value::Record(BTreeMap::from([
        ("labels".into(), labels),
        ("superpixel_count".into(), Value::U64(u64::from(count))),
        ("seed_count".into(), Value::U64(centre_count as u64)),
        (
            "window_fallback_assignments".into(),
            Value::U64(uncovered as u64),
        ),
    ])))
}

#[derive(Clone, Copy)]
struct Centre {
    lab: [f64; 3],
    x: f64,
    y: f64,
}

impl Centre {
    fn distance(self, colour: [f64; 3], x: usize, y: usize, spatial_weight: f64) -> f64 {
        let spatial = (self.x - x as f64).powi(2) + (self.y - y as f64).powi(2);
        difference(self.lab, colour) + spatial_weight * spatial
    }
}

fn difference(a: [f64; 3], b: [f64; 3]) -> f64 {
    a.into_iter().zip(b).map(|(a, b)| (a - b).powi(2)).sum()
}

fn near_gradient(grid: Grid, lab: &[[f64; 3]], x: usize, y: usize) -> f64 {
    let west = y * grid.width + x.saturating_sub(1);
    let east = y * grid.width + (x + 1).min(grid.width - 1);
    let north = y.saturating_sub(1) * grid.width + x;
    let south = (y + 1).min(grid.height - 1) * grid.width + x;
    difference(lab[west], lab[east]) + difference(lab[north], lab[south])
}

fn slic(
    grid: Grid,
    lab: &[[f64; 3]],
    step: usize,
    compactness: f64,
    iterations: usize,
) -> Result<(Vec<u32>, u32, usize), String> {
    let mut centres = Vec::new();
    for y in (0..grid.height).step_by(step) {
        for x in (0..grid.width).step_by(step) {
            let (ix, iy) = (
                (x + step / 2).min(grid.width - 1),
                (y + step / 2).min(grid.height - 1),
            );
            let (mut cx, mut cy) = (ix, iy);
            let mut gradient = near_gradient(grid, lab, ix, iy);
            for ny in iy.saturating_sub(1)..=(iy + 1).min(grid.height - 1) {
                for nx in ix.saturating_sub(1)..=(ix + 1).min(grid.width - 1) {
                    let current = near_gradient(grid, lab, nx, ny);
                    if current < gradient {
                        (cx, cy, gradient) = (nx, ny, current);
                    }
                }
            }
            centres.push(Centre {
                lab: lab[cy * grid.width + cx],
                x: cx as f64,
                y: cy as f64,
            });
        }
    }
    let spatial_weight = (compactness / step as f64).powi(2);
    let mut labels = vec![UNASSIGNED; grid.len()];
    let mut distances = vec![f64::INFINITY; grid.len()];
    let mut counts = vec![0u32; centres.len()];
    let mut totals = vec![[0.0; 5]; centres.len()];
    let mut uncovered = 0;
    let mut fallback_budget = grid.len() * FALLBACK_COMPARISONS_PER_PIXEL;
    for _ in 0..iterations {
        labels.fill(UNASSIGNED);
        distances.fill(f64::INFINITY);
        counts.fill(0);
        totals.fill([0.0; 5]);
        for (id, centre) in centres.iter().enumerate() {
            let (cx, cy) = (centre.x.round() as usize, centre.y.round() as usize);
            for y in cy.saturating_sub(step)..=(cy + step).min(grid.height - 1) {
                for x in cx.saturating_sub(step)..=(cx + step).min(grid.width - 1) {
                    let index = y * grid.width + x;
                    let distance = centre.distance(lab[index], x, y, spatial_weight);
                    if distance < distances[index] {
                        distances[index] = distance;
                        labels[index] = id as u32;
                    }
                }
            }
        }
        for (index, id) in labels.iter_mut().enumerate() {
            let x = index % grid.width;
            let y = index / grid.width;
            if *id == UNASSIGNED {
                // A cluster can move far enough to leave a pixel outside every
                // local window. Search all admitted centres for those pixels.
                fallback_budget = fallback_budget
                    .checked_sub(centres.len())
                    .ok_or("SLIC uncovered-pixel search exceeds its comparison budget")?;
                let mut best = f64::INFINITY;
                for (candidate, centre) in centres.iter().enumerate() {
                    let distance = centre.distance(lab[index], x, y, spatial_weight);
                    if distance < best {
                        best = distance;
                        *id = candidate as u32;
                    }
                }
                uncovered += 1;
            }
            counts[*id as usize] += 1;
            let features = [
                lab[index][0],
                lab[index][1],
                lab[index][2],
                x as f64,
                y as f64,
            ];
            for (total, feature) in totals[*id as usize].iter_mut().zip(features) {
                *total += feature;
            }
        }
        for (id, centre) in centres.iter_mut().enumerate() {
            if counts[id] != 0 {
                let [l, a, b, x, y] = totals[id].map(|v| v / f64::from(counts[id]));
                *centre = Centre {
                    lab: [l, a, b],
                    x,
                    y,
                };
            }
        }
    }
    let (labels, count) = connected_labels(grid, &labels, (step * step / 4).max(1));
    Ok((labels, count, uncovered))
}

fn connected_labels(grid: Grid, input: &[u32], minimum: usize) -> (Vec<u32>, u32) {
    let mut output = vec![UNASSIGNED; input.len()];
    let mut queue = Vec::new();
    let mut label_count = 0;
    for start in 0..input.len() {
        if output[start] != UNASSIGNED {
            continue;
        }
        let mut already_connected = None;
        queue.clear();
        queue.push(start);
        output[start] = PENDING;
        let mut cursor = 0;
        while cursor < queue.len() {
            let index = queue[cursor];
            cursor += 1;
            let (x, y) = (index % grid.width, index / grid.width);
            let neighbors = [
                index.checked_sub(1).filter(|_| x > 0),
                (x + 1 < grid.width).then_some(index + 1),
                index.checked_sub(grid.width).filter(|_| y > 0),
                (y + 1 < grid.height).then_some(index + grid.width),
            ];
            for next in neighbors.into_iter().flatten() {
                match output[next] {
                    UNASSIGNED if input[next] == input[start] => {
                        output[next] = PENDING;
                        queue.push(next);
                    }
                    label if label < PENDING => {
                        already_connected = Some(already_connected.unwrap_or(label).min(label));
                    }
                    _ => {}
                }
            }
        }
        let merged = already_connected.filter(|_| queue.len() < minimum);
        let label = merged.unwrap_or_else(|| {
            let id = label_count;
            label_count += 1;
            id
        });
        for &index in &queue {
            output[index] = label;
        }
    }
    (output, label_count)
}

#[derive(Clone, Copy)]
struct Rect {
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
}

impl Rect {
    fn expand(self, r: usize, grid: Grid) -> Self {
        Self {
            x0: self.x0.saturating_sub(r),
            y0: self.y0.saturating_sub(r),
            x1: (self.x1 + r).min(grid.width),
            y1: (self.y1 + r).min(grid.height),
        }
    }
    fn pixels(self) -> usize {
        (self.x1 - self.x0) * (self.y1 - self.y0)
    }
    fn prefix_cells(self) -> usize {
        (self.x1 - self.x0 + 1) * (self.y1 - self.y0 + 1)
    }
}

struct Integral<const N: usize> {
    bounds: Rect,
    stride: usize,
    cells: Vec<[f64; N]>,
}

impl<const N: usize> Integral<N> {
    fn new(bounds: Rect, sample: impl Fn(usize, usize) -> [f64; N]) -> Self {
        let stride = bounds.x1 - bounds.x0 + 1;
        let mut cells = vec![[0.0; N]; bounds.prefix_cells()];
        for y in bounds.y0..bounds.y1 {
            let row = (y - bounds.y0 + 1) * stride;
            let mut total = [0.0; N];
            for x in bounds.x0..bounds.x1 {
                let index = row + x - bounds.x0 + 1;
                for (sum, value) in total.iter_mut().zip(sample(x, y)) {
                    *sum += value;
                }
                cells[index] = std::array::from_fn(|i| total[i] + cells[index - stride][i]);
            }
        }
        Self {
            bounds,
            stride,
            cells,
        }
    }

    fn mean(&self, window: Rect) -> [f64; N] {
        let index = |x, y| (y - self.bounds.y0) * self.stride + x - self.bounds.x0;
        let top_left = self.cells[index(window.x0, window.y0)];
        let top_right = self.cells[index(window.x1, window.y0)];
        let bottom_left = self.cells[index(window.x0, window.y1)];
        let bottom_right = self.cells[index(window.x1, window.y1)];
        let count = window.pixels() as f64;
        std::array::from_fn(|i| {
            (bottom_right[i] - top_right[i] - bottom_left[i] + top_left[i]) / count
        })
    }
}

fn coverage(host: &AssetHost<'_>, value: &Value, grid: Grid) -> Result<Vec<f32>, String> {
    match host.workspace.typed(value)? {
        TypedArtifact::FieldLattice(field) => {
            let FieldLattice::DenseF32 { axes, samples } = field.as_ref() else {
                return Err("Guided mask requires a dense scalar field".into());
            };
            if *axes != grid.axes()
                || samples.len() != grid.len()
                || samples.iter().any(|value| !(0.0..=1.0).contains(value))
            {
                return Err(
                    "Guided mask must have matching dimensions and finite [0, 1] coverage".into(),
                );
            }
            Ok(samples.clone())
        }
        TypedArtifact::RadianceLattice(mask)
            if mask.width as usize == grid.width
                && mask.height as usize == grid.height
                && mask.channels == 1
                && mask.transfer_iri == LINEAR
                && sample_len(&mask.samples) == grid.len() =>
        {
            (0..grid.len())
                .map(|index| sample(&mask.samples, index))
                .collect()
        }
        _ => Err("Expected matching linear one-channel coverage or a dense scalar field".into()),
    }
}

fn guided_filter(host: &mut AssetHost<'_>, args: &Args) -> Result<Value, String> {
    fields(
        args,
        &["guide", "mask", "radius_px", "epsilon", "tile_size"],
    )?;
    let guide = required(args, "guide")?;
    let grid = Grid::for_lattice(lattice(host, guide)?)?;
    let radius = count(args, "radius_px", 8, [1, 64])?;
    let epsilon = scalar(args, "epsilon", 1e-4, [1e-8, 1.0])?;
    let tile = count(args, "tile_size", 256, [64, 512])?;
    let mut tiles = Vec::new();
    let mut work = grid.len() * 48;
    let mut scratch = 0;
    for y0 in (0..grid.height).step_by(tile) {
        for x0 in (0..grid.width).step_by(tile) {
            let core = Rect {
                x0,
                y0,
                x1: (x0 + tile).min(grid.width),
                y1: (y0 + tile).min(grid.height),
            };
            let coefficients = core.expand(radius, grid);
            let source = coefficients.expand(radius, grid);
            work += source.pixels() * 16 + coefficients.pixels() * 48;
            scratch = scratch.max(source.prefix_cells() * 32 + coefficients.prefix_cells() * 16);
            tiles.push((core, coefficients, source));
        }
    }
    host.charge(work, scratch + grid.len() * 12)?;
    let image = lattice(host, guide)?;
    let (transfer, channels) = colour(image, grid)?;
    let mut intensity = Vec::with_capacity(grid.len());
    for index in 0..grid.len() {
        let [r, g, b] = linear_rgb(image, index, transfer, channels)?;
        intensity.push((0.212_672_9 * r + 0.715_152_2 * g + 0.072_175 * b) as f32);
    }
    let coverage = coverage(host, required(args, "mask")?, grid)?;
    let pixel_window = |x, y| {
        Rect {
            x0: x,
            y0: y,
            x1: x + 1,
            y1: y + 1,
        }
        .expand(radius, grid)
    };
    let mut output = vec![0.0f32; grid.len()];
    for (core, coefficients, source) in tiles {
        let moments = Integral::<4>::new(source, |x, y| {
            let index = y * grid.width + x;
            let i = f64::from(intensity[index]);
            let p = f64::from(coverage[index]);
            [i, p, i * i, i * p]
        });
        let coefficients = Integral::<2>::new(coefficients, |x, y| {
            let [i, p, i2, ip] = moments.mean(pixel_window(x, y));
            let a = (ip - i * p) / ((i2 - i * i).max(0.0) + epsilon);
            [a, p - a * i]
        });
        for y in core.y0..core.y1 {
            for x in core.x0..core.x1 {
                let index = y * grid.width + x;
                let [a, b] = coefficients.mean(pixel_window(x, y));
                output[index] = (a * f64::from(intensity[index]) + b).clamp(0.0, 1.0) as f32;
            }
        }
    }
    if output.iter().any(|value| !value.is_finite()) {
        return Err("Guided filter produced invalid coverage".into());
    }
    Ok(Value::Record(BTreeMap::from([(
        "mask".into(),
        field(host, grid, output)?,
    )])))
}
