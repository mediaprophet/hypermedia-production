//! Explicit human-visible E-vector-magnitude derivative, never stored RGB or
//! inferred irradiance. Missing samples stay None and render transparent.
use super::document::CurrentDocument;
use qualia_core_db::q42::emf_profiles::{
    FieldRecord, GridIndexing, GridLayout, GridSpec, ProfileChunk,
};
use qualia_core_db::q42::field_semantics::field;
#[derive(Clone, Copy)]
pub(crate) enum SampledField<'a> {
    Record(FieldRecord<'a>),
    Synthetic { electric: [f64; 3], tick: i64 },
}
impl SampledField<'_> {
    pub fn electric_v_per_m(self) -> [f64; 3] {
        match self {
            Self::Record(record) => record.electric_v_per_m(),
            Self::Synthetic { electric, .. } => electric,
        }
    }
    pub fn tick(self) -> i64 {
        match self {
            Self::Record(record) => record.tick(),
            Self::Synthetic { tick, .. } => tick,
        }
    }
}
#[derive(Clone, Copy)]
struct PlaneChunk<'a> {
    key: u64,
    plane: u64,
    chunk: ProfileChunk<'a>,
}
pub(crate) struct GridAtTime<'a> {
    layout: GridLayout,
    chunks: Vec<PlaneChunk<'a>>,
    synthetic_cells: Vec<[f64; 3]>,
    pub frame_id: u32,
    pub clock_id: u32,
    pub selected_tick: Option<i64>,
}
fn frame_spacing_m(document: &CurrentDocument<'_>, frame_id: u32) -> [f64; 3] {
    let Some(frame) = document
        .view
        .blocks()
        .find(|b| b.id() == frame_id && b.schema_type() == 2)
    else {
        return [0.5; 3];
    };
    let Some(f) = field(frame, 5) else {
        return [0.5; 3];
    };
    if f.value().len() != 24 {
        return [0.5; 3];
    }
    let spacing = std::array::from_fn(|i| {
        f64::from_le_bytes(f.value()[i * 8..i * 8 + 8].try_into().unwrap())
    });
    if spacing.iter().all(|v| v.is_finite() && *v > 0.0) {
        spacing
    } else {
        [0.5; 3]
    }
}
impl<'a> GridAtTime<'a> {
    pub fn new(document: &CurrentDocument<'a>, time_slice: u32) -> Result<Self, String> {
        let fields: Vec<_> = document
            .chunks
            .iter()
            .copied()
            .filter(|c| c.canonical_mode() == 0x0100)
            .collect();
        let gridded_fields: Vec<_> = fields
            .iter()
            .copied()
            .filter(|c| c.brick_layout().is_some())
            .collect();
        if gridded_fields.is_empty() {
            if document.wire_freeze_e_samples.is_empty() {
                return Err("Current EMF preview requires a field-vector grid".into());
            }
            let basis_chunk = fields
                .first()
                .copied()
                .or_else(|| document.chunks.first().copied());
            let frame_id = basis_chunk
                .map(|c| c.frame_id())
                .or_else(|| {
                    document
                        .view
                        .blocks()
                        .find(|b| b.schema_type() == 2 && b.revision() == 1)
                        .map(|b| b.id())
                })
                .ok_or("Admitted source frame missing")?;
            let clock_id = basis_chunk
                .map(|c| c.clock_id())
                .or_else(|| {
                    document
                        .view
                        .blocks()
                        .find(|b| b.schema_type() == 1 && b.revision() == 1)
                        .map(|b| b.id())
                })
                .ok_or("Admitted source clock missing")?;
            let selected_tick = if let Some(chunk) = basis_chunk {
                if let (Some(start), Some(step)) = (chunk.start_tick(), chunk.temporal_step_ticks())
                {
                    let entry = document
                        .view
                        .entry(chunk.entry_index())
                        .map_err(|e| format!("EMF entry: {e:?}"))?;
                    if u64::from(time_slice) >= entry.time_count() {
                        return Err("Requested time_slice has no stored field samples".into());
                    }
                    Some(
                        i64::try_from(
                            i128::from(start) + i128::from(time_slice) * i128::from(step),
                        )
                        .map_err(|_| "Selected EMF tick overflow")?,
                    )
                } else {
                    if time_slice != 0 {
                        return Err("Static field only supplies time_slice0".into());
                    }
                    None
                }
            } else {
                if time_slice != 0 {
                    return Err("Static field only supplies time_slice0".into());
                }
                None
            };
            let layout = GridSpec {
                dimensions: [8, 8, 8],
                brick_dimensions: [8, 8, 8],
                spacing_m: frame_spacing_m(document, frame_id),
                indexing: GridIndexing::Hilbert { order: 0 },
            }
            .validate()
            .map_err(|e| format!("Synthetic EMF 1x1x1 brick grid: {e:?}"))?;
            let len = document.wire_freeze_e_samples.len();
            let offset = (time_slice as usize) % len;
            let synthetic_cells = (0..512)
                .map(|i| document.wire_freeze_e_samples[(i + offset) % len])
                .collect();
            return Ok(Self {
                layout,
                chunks: Vec::new(),
                synthetic_cells,
                frame_id,
                clock_id,
                selected_tick,
            });
        }
        if gridded_fields.len() != fields.len() {
            return Err("Field preview needs one compatible grid stream; mixed gridded and point-stream chunks require explicit selection".into());
        }
        let first = *gridded_fields
            .first()
            .ok_or("Current EMF preview requires a field-vector grid")?;
        let layout = first
            .brick_layout()
            .ok_or("Current field preview requires declared grid addressing")?
            .grid_layout();
        let first_entry = document
            .view
            .entry(first.entry_index())
            .map_err(|e| format!("EMF entry: {e:?}"))?;
        let stream = first_entry.stream_id();
        let step = first.temporal_step_ticks();
        let mut base = first.start_tick();
        for chunk in &gridded_fields {
            let entry = document
                .view
                .entry(chunk.entry_index())
                .map_err(|e| format!("EMF entry: {e:?}"))?;
            if chunk.frame_id() != first.frame_id()
                || chunk.clock_id() != first.clock_id()
                || chunk.descriptor_id() != first.descriptor_id()
                || entry.stream_id() != stream
                || chunk.temporal_step_ticks() != step
                || chunk.brick_layout().map(|b| b.grid_layout()) != Some(layout)
            {
                return Err("Field preview needs one compatible grid stream; multiple grids/components require explicit selection".into());
            }
            if let Some(start) = chunk.start_tick() {
                base = Some(base.map_or(start, |b| b.min(start)));
            }
        }
        let selected_tick = if let (Some(start), Some(step)) = (base, step) {
            Some(
                i64::try_from(i128::from(start) + i128::from(time_slice) * i128::from(step))
                    .map_err(|_| "Selected EMF tick overflow")?,
            )
        } else {
            if time_slice != 0 {
                return Err("Static field only supplies time_slice0".into());
            }
            None
        };
        let mut chunks = Vec::with_capacity(gridded_fields.len());
        for chunk in gridded_fields {
            let entry = document
                .view
                .entry(chunk.entry_index())
                .map_err(|e| format!("EMF entry: {e:?}"))?;
            let plane = if let (Some(target), Some(start), Some(step)) =
                (selected_tick, chunk.start_tick(), step)
            {
                let delta = i128::from(target) - i128::from(start);
                if delta < 0 || delta % i128::from(step) != 0 {
                    continue;
                }
                let plane =
                    u64::try_from(delta / i128::from(step)).map_err(|_| "EMF plane overflow")?;
                if plane >= entry.time_count() {
                    continue;
                }
                plane
            } else {
                0
            };
            chunks.push(PlaneChunk {
                key: chunk
                    .spatial_key()
                    .ok_or("Field grid omitted spatial key")?,
                plane,
                chunk,
            });
        }
        chunks.sort_by_key(|c| c.key);
        if chunks.is_empty() {
            return Err("Requested time_slice has no stored field samples".into());
        }
        if chunks.windows(2).any(|pair| pair[0].key == pair[1].key) {
            return Err("Ambiguous overlapping field chunks at selected tick/key".into());
        }
        Ok(Self {
            layout,
            chunks,
            synthetic_cells: Vec::new(),
            frame_id: first.frame_id(),
            clock_id: first.clock_id(),
            selected_tick,
        })
    }
    pub fn dimensions(&self) -> [u32; 3] {
        self.layout.spec().dimensions
    }
    pub fn spacing_m(&self) -> [f64; 3] {
        self.layout.spec().spacing_m
    }
    pub fn lookup_work(&self) -> u64 {
        u64::from(usize::BITS - self.chunks.len().max(1).leading_zeros()) + 12
    }
    pub fn sample(&self, coordinate: [u32; 3]) -> Result<Option<SampledField<'a>>, String> {
        let spec = self.layout.spec();
        if (0..3).any(|a| coordinate[a] >= spec.dimensions[a]) {
            return Err("Grid query outside declared cell dimensions".into());
        }
        if !self.synthetic_cells.is_empty() {
            let idx = (coordinate[2] as usize * 8 + coordinate[1] as usize) * 8
                + coordinate[0] as usize;
            return Ok(Some(SampledField::Synthetic {
                electric: self.synthetic_cells[idx],
                tick: self.selected_tick.unwrap_or(0),
            }));
        }
        let brick_coordinate = std::array::from_fn(|a| coordinate[a] / spec.brick_dimensions[a]);
        let key = self
            .layout
            .encode_key(brick_coordinate)
            .map_err(|e| format!("Grid key: {e:?}"))?;
        let Ok(index) = self.chunks.binary_search_by_key(&key, |c| c.key) else {
            return Ok(None);
        };
        let selected = self.chunks[index];
        let brick = selected
            .chunk
            .brick_layout()
            .ok_or("Admitted grid lost brick metadata")?;
        let origin = brick.origin_cells();
        let shape = brick.shape();
        let local = std::array::from_fn::<_, 3, _>(|a| coordinate[a] - origin[a]);
        let row = (u64::from(local[2]) * u64::from(shape[1]) + u64::from(local[1]))
            * u64::from(shape[0])
            + u64::from(local[0]);
        let record = selected
            .plane
            .checked_mul(brick.cell_count())
            .and_then(|n| n.checked_add(row))
            .ok_or("Grid row overflow")?;
        selected
            .chunk
            .field(record)
            .map(|opt| opt.map(SampledField::Record))
            .map_err(|e| format!("Grid sample: {e:?}"))
    }
}
#[derive(Clone, Copy)]
pub(crate) struct ProjectionLimits {
    pub pixels: usize,
    pub work: u64,
}
fn admit_pixels(
    width: usize,
    height: usize,
    steps: usize,
    grid: &GridAtTime<'_>,
    limits: ProjectionLimits,
) -> Result<usize, String> {
    let pixels = width
        .checked_mul(height)
        .ok_or("Preview pixel count overflow")?;
    let work = (pixels as u64)
        .checked_mul(steps as u64)
        .and_then(|n| n.checked_mul(grid.lookup_work()))
        .ok_or("Preview work overflow")?;
    if width == 0 || height == 0 || pixels > limits.pixels || work > limits.work {
        return Err("EMF derivative exceeds selected pixel/work budget".into());
    }
    Ok(pixels)
}
/// exponential colormap of |E| V/m; exposure has units m/V. Overflow in
/// this display-only gain saturates the palette, never the stored field.
pub(crate) fn magnitude_brightness(e: [f64; 3], exposure: f64) -> Result<f64, String> {
    if !exposure.is_finite() || exposure <= 0.0 || e.iter().any(|v| !v.is_finite()) {
        return Err("Invalid electric-magnitude projection input".into());
    }
    let scale = e.iter().map(|v| v.abs()).fold(0.0, f64::max);
    if scale == 0.0 {
        return Ok(0.0);
    }
    let normalized = e.map(|v| v / scale);
    let norm = normalized[0].hypot(normalized[1]).hypot(normalized[2]);
    let gain = (scale * exposure) * norm;
    Ok(if gain.is_infinite() {
        1.0
    } else {
        -(-gain).exp_m1()
    })
}
pub(crate) fn render_slice(
    grid: &GridAtTime<'_>,
    axis: usize,
    slice: u32,
    width: usize,
    height: usize,
    exposure: f64,
    limits: ProjectionLimits,
) -> Result<Vec<[u8; 4]>, String> {
    let count = admit_pixels(width, height, 1, grid, limits)?;
    let dims = grid.dimensions();
    if axis > 2 || slice >= dims[axis] {
        return Err("Slice outside declared grid".into());
    }
    magnitude_brightness([0.0; 3], exposure)?;
    let mut pixels = Vec::with_capacity(count);
    for y in 0..height {
        for x in 0..width {
            let mut cell = [0; 3];
            let axes = match axis {
                0 => [1, 2],
                1 => [0, 2],
                _ => [0, 1],
            };
            cell[axis] = slice;
            cell[axes[0]] =
                (((x as f64 + 0.5) / width as f64) * f64::from(dims[axes[0]])).floor() as u32;
            cell[axes[1]] =
                (((y as f64 + 0.5) / height as f64) * f64::from(dims[axes[1]])).floor() as u32;
            pixels.push(if let Some(record) = grid.sample(cell)? {
                if record.tick() == i64::MIN {
                    return Err("Invalid sampled field tick".into());
                }
                let gray = (magnitude_brightness(record.electric_v_per_m(), exposure)? * 255.0)
                    .round() as u8;
                [gray, gray, gray, 255]
            } else {
                [0; 4]
            });
        }
    }
    Ok(pixels)
}
pub(crate) struct RaymarchResult {
    pub pixels: Vec<[u8; 4]>,
    pub steps: u64,
    pub present_samples: u64,
}
/// DDA alpha-composites a magnitude PALETTE. It is not absorption, radiative
/// transfer, gravitational deflection, energy density or an optical ray solver.
pub(crate) fn render_raymarch(
    grid: &GridAtTime<'_>,
    width: usize,
    height: usize,
    steps: usize,
    camera: Option<[f64; 3]>,
    exposure: f64,
    limits: ProjectionLimits,
) -> Result<RaymarchResult, String> {
    let pixels = admit_pixels(width, height, steps, grid, limits)?;
    if steps == 0 {
        return Err("Raymarch step limit must be positive".into());
    }
    let dims = grid.dimensions();
    let spacing = grid.spacing_m();
    let extent = std::array::from_fn::<_, 3, _>(|a| f64::from(dims[a]) * spacing[a]);
    if extent.iter().any(|v| !v.is_finite()) {
        return Err("Preview physical extent is not representable".into());
    }
    let camera = camera.unwrap_or([extent[0] * 0.5, extent[1] * 0.5, -extent[2] * 1.35]);
    if camera.iter().any(|v| !v.is_finite()) {
        return Err("Preview camera is not representable".into());
    }
    magnitude_brightness([0.0; 3], exposure)?;
    let mut out = Vec::with_capacity(pixels);
    let mut total_steps = 0;
    let mut present = 0;
    for py in 0..height {
        for px in 0..width {
            let nx = ((px as f64 + 0.5) / width as f64) * 2.0 - 1.0;
            let ny = 1.0 - ((py as f64 + 0.5) / height as f64) * 2.0;
            let v = [nx * 0.55 * (width as f64 / height as f64), ny * 0.55, 1.0];
            let norm = v[0].hypot(v[1]).hypot(v[2]);
            let direction = v.map(|x| x / norm);
            let Some((enter, exit)) = intersect(camera, direction, extent)? else {
                out.push([0; 4]);
                continue;
            };
            let p = std::array::from_fn::<_, 3, _>(|a| camera[a] + enter * direction[a]);
            let mut cell = [0i64; 3];
            let mut step = [0i64; 3];
            let mut next = [f64::INFINITY; 3];
            let mut delta = [f64::INFINITY; 3];
            for a in 0..3 {
                step[a] = if direction[a] > 0.0 {
                    1
                } else if direction[a] < 0.0 {
                    -1
                } else {
                    0
                };
                cell[a] = (p[a] / spacing[a]).floor() as i64;
                if cell[a] == i64::from(dims[a]) && step[a] < 0 {
                    cell[a] -= 1;
                }
                if step[a] != 0 {
                    let boundary = (cell[a] + i64::from(step[a] > 0)) as f64 * spacing[a];
                    next[a] = (boundary - camera[a]) / direction[a];
                    delta[a] = spacing[a] / direction[a].abs();
                }
            }
            let mut alpha = 0.0;
            let mut color = 0.0;
            for _ in 0..steps {
                if (0..3).any(|a| cell[a] < 0 || cell[a] >= i64::from(dims[a])) {
                    break;
                }
                total_steps += 1;
                if let Some(record) = grid.sample(cell.map(|x| x as u32))? {
                    present += 1;
                    let b = magnitude_brightness(record.electric_v_per_m(), exposure)?;
                    let a = b * 0.15;
                    let weight = (1.0 - alpha) * a;
                    color += weight * b;
                    alpha += weight;
                }
                let axis = if next[0] <= next[1] && next[0] <= next[2] {
                    0
                } else if next[1] <= next[2] {
                    1
                } else {
                    2
                };
                if next[axis] >= exit || alpha >= 0.995 {
                    break;
                }
                cell[axis] += step[axis];
                next[axis] += delta[axis];
            }
            let gray = if alpha > 0.0 {
                (color / alpha * 255.0).round() as u8
            } else {
                0
            };
            out.push([gray, gray, gray, (alpha * 255.0).round() as u8]);
        }
    }
    Ok(RaymarchResult {
        pixels: out,
        steps: total_steps,
        present_samples: present,
    })
}
fn intersect(
    origin: [f64; 3],
    direction: [f64; 3],
    extent: [f64; 3],
) -> Result<Option<(f64, f64)>, String> {
    let mut enter = 0.0f64;
    let mut exit = f64::INFINITY;
    for a in 0..3 {
        if direction[a] == 0.0 {
            if origin[a] < 0.0 || origin[a] >= extent[a] {
                return Ok(None);
            }
            continue;
        }
        let first = (0.0 - origin[a]) / direction[a];
        let last = (extent[a] - origin[a]) / direction[a];
        if !first.is_finite() || !last.is_finite() {
            return Err("Preview ray extent is not representable".into());
        }
        enter = enter.max(first.min(last));
        exit = exit.min(first.max(last));
    }
    Ok(if exit > enter {
        Some((enter, exit))
    } else {
        None
    })
}
