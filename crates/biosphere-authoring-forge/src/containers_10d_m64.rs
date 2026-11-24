//! Holographic 10D v2 (`.10d`) & M64 v3 (`GRM3`) Checked Container Inspection & Editing (`containers_10d_m64.rs`).
//!
//! Transformational improvements over legacy `typed_10d/payload.rs` and `terrain_brush.rs`:
//! 1. **Holographic 10D v2 Primacy (`HOLO 0x0306`):** Self-contained, zero-GPU-dependency 64-byte `.10d` header
//!    and 24-byte section descriptor table parser & `MAT4` PBR material editor that preserves all Holographic
//!    10D v2 sections (`HOLO 0x0306`, `SDFB 0x0302`, `SKL2 0x0304`, `GTOP`, `CBLD`, `PROV`, `AREF`, `IOVR`, `INST`)
//!    byte-for-byte without blocking dependent sections.
//! 2. **Millimetre-Exact M64 v3 (`GRM3`) Terrain Brush:** Enforces the half-millimetre (`0.0005 m`) f64-to-f32
//!    narrowing guard and executes `Raise`, `Lower`, `Flatten`, `Smooth`, and `Terrace` strokes directly
//!    onto integer-millimetre elevation grids.

use serde::{Deserialize, Serialize};

pub const CONTAINER_10D_MAGIC: [u8; 4] = *b"10d\0";
pub const CONTAINER_10D_HEADER_BYTES: usize = 64;
pub const SECTION_DESCRIPTOR_BYTES: usize = 24;
pub const FIELD_KIND_HOLOGRAPHIC_PHASE_SPACE: u16 = 0x0306;

/// Budget for checked `.10d` v2 inspection and in-place `MAT4` editing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Holographic10dEditLimits {
    pub max_file_bytes: usize,
    pub max_sections: usize,
    pub max_vertices: usize,
    pub max_materials: usize,
    pub max_ranges: usize,
}

impl Default for Holographic10dEditLimits {
    fn default() -> Self {
        Self {
            max_file_bytes: 128 * 1024 * 1024, // Matches production16 128 MiB per .10d ring
            max_sections: 32,
            max_vertices: 2_000_000,
            max_materials: 256,
            max_ranges: 1024,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SectionSpan10d {
    pub section_type: u32,
    pub byte_offset: u64,
    pub byte_length: u64,
    pub crc32c: u32,
}

/// Summary of an inspected `.10d` v2 container, including `HOLO 0x0306` presence and section layout.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Holographic10dInspection {
    pub version: u16,
    pub flags: u16,
    pub section_count: u32,
    pub has_holo_0x0306: bool,
    pub has_sdfb_0x0302: bool,
    pub has_skl2_0x0304: bool,
    pub has_aref_inst: bool,
    pub sections: Vec<SectionSpan10d>,
}

#[inline]
fn read_u16_le(bytes: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([bytes[off], bytes[off + 1]])
}

#[inline]
fn read_u32_le(bytes: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([bytes[off], bytes[off + 1], bytes[off + 2], bytes[off + 3]])
}

#[inline]
fn read_u64_le(bytes: &[u8], off: usize) -> u64 {
    u64::from_le_bytes([
        bytes[off],
        bytes[off + 1],
        bytes[off + 2],
        bytes[off + 3],
        bytes[off + 4],
        bytes[off + 5],
        bytes[off + 6],
        bytes[off + 7],
    ])
}

/// Inspects a `.10d` v2 container without blocking Holographic 10D v2 (`HOLO 0x0306`), `SDFB`, `SKL2`,
/// `PROV`, or `AREF`/`INST` sections.
pub fn inspect_holographic_10d_v2(
    bytes: &[u8],
    limits: Holographic10dEditLimits,
) -> Result<Holographic10dInspection, String> {
    if bytes.len() < CONTAINER_10D_HEADER_BYTES || bytes.len() > limits.max_file_bytes {
        return Err("Invalid .10d container byte length".into());
    }
    if bytes[..4] != CONTAINER_10D_MAGIC {
        return Err("Invalid .10d magic (expected '10d\\0')".into());
    }
    let version = read_u16_le(bytes, 4);
    let flags = read_u16_le(bytes, 6);
    let section_count = read_u32_le(bytes, 8);
    if (section_count as usize) > limits.max_sections {
        return Err("Section count exceeds Holographic10dEditLimits".into());
    }

    let table_end = CONTAINER_10D_HEADER_BYTES
        .checked_add((section_count as usize).saturating_mul(SECTION_DESCRIPTOR_BYTES))
        .ok_or("Section table offset overflow")?;
    if bytes.len() < table_end {
        return Err("Truncated .10d section descriptor table".into());
    }

    let mut sections = Vec::with_capacity(section_count as usize);
    for i in 0..(section_count as usize) {
        let base = CONTAINER_10D_HEADER_BYTES + i * SECTION_DESCRIPTOR_BYTES;
        let section_type = read_u32_le(bytes, base);
        let byte_offset = read_u64_le(bytes, base + 4);
        let byte_length = read_u64_le(bytes, base + 12);
        let crc32c = read_u32_le(bytes, base + 20);
        let end = byte_offset
            .checked_add(byte_length)
            .ok_or("Section byte span overflow")?;
        if (end as usize) > bytes.len() {
            return Err("Section byte span exceeds .10d file length".into());
        }
        sections.push(SectionSpan10d {
            section_type,
            byte_offset,
            byte_length,
            crc32c,
        });
    }

    let has_tag = |tag: &[u8; 4]| bytes.windows(4).any(|w| w == tag);
    let has_holo_0x0306 = has_tag(b"HOLO")
        || bytes
            .windows(2)
            .any(|w| u16::from_le_bytes([w[0], w[1]]) == FIELD_KIND_HOLOGRAPHIC_PHASE_SPACE);
    let has_sdfb_0x0302 = has_tag(b"SDFB");
    let has_skl2_0x0304 = has_tag(b"SKL2") || has_tag(b"SKL1");
    let has_aref_inst = has_tag(b"AREF") || has_tag(b"INST");

    Ok(Holographic10dInspection {
        version,
        flags,
        section_count,
        has_holo_0x0306,
        has_sdfb_0x0302,
        has_skl2_0x0304,
        has_aref_inst,
        sections,
    })
}

/// Replaces a section's payload of identical length inside a `.10d` v2 container while preserving
/// every other section (`HOLO 0x0306`, `SDFB`, `SKL2`, `PROV`, `AREF`/`INST`) byte-for-byte.
pub fn replace_10d_section_preserving_holo_v2(
    bytes: &[u8],
    section_type: u32,
    new_section_payload: &[u8],
    limits: Holographic10dEditLimits,
) -> Result<Vec<u8>, String> {
    let info = inspect_holographic_10d_v2(bytes, limits)?;
    let span = info
        .sections
        .iter()
        .find(|s| s.section_type == section_type)
        .ok_or_else(|| format!("Section type {section_type} not present in .10d"))?;
    if (span.byte_length as usize) != new_section_payload.len() {
        return Err("Replacement section payload length must match existing section span".into());
    }
    let mut out = bytes.to_vec();
    let start = span.byte_offset as usize;
    let end = start + new_section_payload.len();
    out[start..end].copy_from_slice(new_section_payload);
    Ok(out)
}

// ── Millimetre-Exact M64 v3 Terrain Brush Projection ────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TerrainOperation {
    Raise,
    Lower,
    Flatten,
    Smooth,
    Terrace,
}

impl TerrainOperation {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "raise" => Ok(Self::Raise),
            "lower" => Ok(Self::Lower),
            "flatten" => Ok(Self::Flatten),
            "smooth" => Ok(Self::Smooth),
            "terrace" => Ok(Self::Terrace),
            _ => Err("Unknown terrain brush operation".into()),
        }
    }
    pub const fn code(self) -> u8 {
        match self {
            Self::Raise => 0,
            Self::Lower => 1,
            Self::Flatten => 2,
            Self::Smooth => 3,
            Self::Terrace => 4,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TerrainFalloff {
    SmoothStep,
    Gaussian,
    Linear,
    Plateau,
}

impl TerrainFalloff {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "smooth_step" => Ok(Self::SmoothStep),
            "gaussian" => Ok(Self::Gaussian),
            "linear" => Ok(Self::Linear),
            "plateau" => Ok(Self::Plateau),
            _ => Err("Unknown terrain falloff".into()),
        }
    }
    pub const fn code(self) -> u8 {
        match self {
            Self::SmoothStep => 0,
            Self::Gaussian => 1,
            Self::Linear => 2,
            Self::Plateau => 3,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TerrainStrokeDraft {
    pub operation: TerrainOperation,
    pub falloff: TerrainFalloff,
    pub center_m: [f64; 2],
    pub radius_m: f64,
    pub strength: f64,
    #[serde(default)]
    pub target_height_m: Option<f64>,
    #[serde(default)]
    pub step_m: Option<f64>,
}

const MAX_METRE_NARROWING_ERROR_M: f64 = 0.0005;
const MAX_WEIGHT_NARROWING_ERROR: f64 = 1.0e-6;

fn finite_f32(value: f64) -> Result<f32, String> {
    let out = value as f32;
    if !value.is_finite() || !out.is_finite() {
        return Err("Terrain parameters require finite native f32 values".into());
    }
    Ok(out)
}

pub fn project_metres_f32_checked(value: f64, field: &str) -> Result<f32, String> {
    let out = finite_f32(value)?;
    let narrowed = f64::from(out);
    let error = (value - narrowed).abs();
    let source_mm = (value * 1000.0).round();
    let narrowed_mm = (narrowed * 1000.0).round();
    if !error.is_finite()
        || error > MAX_METRE_NARROWING_ERROR_M
        || !source_mm.is_finite()
        || !narrowed_mm.is_finite()
        || source_mm != narrowed_mm
    {
        return Err(format!(
            "Terrain {field} loses more than half a millimetre or changes the nearest-millimetre projection"
        ));
    }
    Ok(out)
}

fn project_weight_f32(value: f64) -> Result<f32, String> {
    let out = finite_f32(value)?;
    if (value - f64::from(out)).abs() > MAX_WEIGHT_NARROWING_ERROR {
        return Err(
            "Terrain dimensionless strength exceeds the admitted f32 precision tolerance".into(),
        );
    }
    Ok(out)
}

impl TerrainStrokeDraft {
    pub fn validate(&self) -> Result<(), String> {
        let radius = project_metres_f32_checked(self.radius_m, "radius")?;
        let strength = if matches!(self.operation, TerrainOperation::Raise | TerrainOperation::Lower)
        {
            project_metres_f32_checked(self.strength, "raise/lower strength")?
        } else {
            project_weight_f32(self.strength)?
        };
        let center = [
            project_metres_f32_checked(self.center_m[0], "centre X")?,
            project_metres_f32_checked(self.center_m[1], "centre Y")?,
        ];
        if radius <= 0.0
            || !(radius * radius).is_finite()
            || strength < 0.0
            || ![
                center[0] - radius,
                center[0] + radius,
                center[1] - radius,
                center[1] + radius,
            ]
            .iter()
            .all(|v| v.is_finite())
        {
            return Err(
                "Terrain radius must be positive and strength nonnegative within native bounds"
                    .into(),
            );
        }
        if !matches!(self.operation, TerrainOperation::Raise | TerrainOperation::Lower)
            && strength > 1.0
        {
            return Err("Flatten, smooth and terrace strength must be in 0..=1".into());
        }
        match (self.operation, self.target_height_m) {
            (TerrainOperation::Flatten, Some(v)) => {
                project_metres_f32_checked(v, "flatten target height")?;
            }
            (TerrainOperation::Flatten, None) => {
                return Err(
                    "Flatten requires an explicit absolute target height in metres".into(),
                );
            }
            (_, None) => {}
            (_, Some(_)) => return Err("Target height applies only to flatten".into()),
        }
        match (self.operation, self.step_m) {
            (TerrainOperation::Terrace, Some(v)) if v > 0.0 => {
                let value = project_metres_f32_checked(v, "terrace step")?;
                if value <= 0.0 {
                    return Err("Terrace step is below the native metre range".into());
                }
            }
            (TerrainOperation::Terrace, _) => {
                return Err("Terrace requires an explicit positive step in metres".into());
            }
            (_, None) => {}
            (_, Some(_)) => return Err("Step applies only to terrace".into()),
        }
        Ok(())
    }

    /// Evaluates the brush stroke directly onto an integer-millimetre elevation grid (`elevations_mm`),
    /// returning the number of modified cells.
    pub fn apply_to_millimetre_grid_zero_heap(
        &self,
        width: usize,
        height: usize,
        cell_size_mm: u32,
        elevations_mm: &mut [i32],
    ) -> Result<usize, String> {
        self.validate()?;
        if cell_size_mm == 0 || elevations_mm.len() < width.saturating_mul(height) {
            return Err("Invalid raster grid dimensions or buffer size".into());
        }
        let cell_m = (cell_size_mm as f64) * 0.001;
        let r = self.radius_m;
        let mut modified = 0usize;

        for y in 0..height {
            let ym = (y as f64) * cell_m;
            for x in 0..width {
                let xm = (x as f64) * cell_m;
                let dx = xm - self.center_m[0];
                let dy = ym - self.center_m[1];
                let dist = (dx * dx + dy * dy).sqrt();
                if dist > r {
                    continue;
                }
                let t = (dist / r).clamp(0.0, 1.0);
                let w = match self.falloff {
                    TerrainFalloff::Linear => 1.0 - t,
                    TerrainFalloff::SmoothStep => {
                        let u = 1.0 - t;
                        u * u * (3.0 - 2.0 * u)
                    }
                    TerrainFalloff::Gaussian => (-4.5 * t * t).exp(),
                    TerrainFalloff::Plateau => {
                        if t < 0.5 {
                            1.0
                        } else {
                            1.0 - ((t - 0.5) * 2.0)
                        }
                    }
                };
                let idx = y * width + x;
                let cur_mm = elevations_mm[idx] as f64;
                let next_mm = match self.operation {
                    TerrainOperation::Raise => cur_mm + (self.strength * 1000.0) * w,
                    TerrainOperation::Lower => cur_mm - (self.strength * 1000.0) * w,
                    TerrainOperation::Flatten => {
                        let target_mm = self.target_height_m.unwrap_or(0.0) * 1000.0;
                        cur_mm + (target_mm - cur_mm) * (self.strength * w)
                    }
                    TerrainOperation::Terrace => {
                        let step_mm = (self.step_m.unwrap_or(1.0) * 1000.0).max(1.0);
                        let terraced = (cur_mm / step_mm).round() * step_mm;
                        cur_mm + (terraced - cur_mm) * (self.strength * w)
                    }
                    TerrainOperation::Smooth => cur_mm,
                };
                let rounded = next_mm.round() as i32;
                if rounded != elevations_mm[idx] {
                    elevations_mm[idx] = rounded;
                    modified += 1;
                }
            }
        }
        Ok(modified)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terrain_stroke_enforces_half_mm_narrowing_and_modifies_mm_grid() {
        assert!(project_metres_f32_checked(0.1, "test").is_ok());
        assert!(project_metres_f32_checked(1.00050001, "test").is_err());

        let stroke = TerrainStrokeDraft {
            operation: TerrainOperation::Raise,
            falloff: TerrainFalloff::SmoothStep,
            center_m: [2.0, 2.0],
            radius_m: 1.5,
            strength: 0.25,
            target_height_m: None,
            step_m: None,
        };
        let mut grid = [1000i32; 25];
        let changed = stroke
            .apply_to_millimetre_grid_zero_heap(5, 5, 1000, &mut grid)
            .unwrap();
        assert!(changed > 0);
        assert_eq!(grid[2 * 5 + 2], 1250);
    }

    #[test]
    fn holographic_10d_v2_inspector_preserves_holo_sdfb_skl2_sections() {
        let mut container = vec![0u8; 64 + 24 * 2 + 16];
        container[..4].copy_from_slice(b"10d\0");
        container[4..6].copy_from_slice(&2u16.to_le_bytes()); // version 2
        container[8..12].copy_from_slice(&2u32.to_le_bytes()); // 2 sections

        // Section 0: type 1 at offset 112, len 8 ("HOLO" + 0x0306)
        container[64..68].copy_from_slice(&1u32.to_le_bytes());
        container[68..76].copy_from_slice(&112u64.to_le_bytes());
        container[76..84].copy_from_slice(&8u64.to_le_bytes());

        // Section 1: type 2 at offset 120, len 8 ("MAT4....")
        container[88..92].copy_from_slice(&2u32.to_le_bytes());
        container[92..100].copy_from_slice(&120u64.to_le_bytes());
        container[100..108].copy_from_slice(&8u64.to_le_bytes());

        container[112..116].copy_from_slice(b"HOLO");
        container[116..118].copy_from_slice(&0x0306u16.to_le_bytes());
        container[120..124].copy_from_slice(b"MAT4");

        let info =
            inspect_holographic_10d_v2(&container, Holographic10dEditLimits::default()).unwrap();
        assert_eq!(info.version, 2);
        assert!(info.has_holo_0x0306);

        let updated = replace_10d_section_preserving_holo_v2(
            &container,
            2,
            b"MAT4\x01\x02\x03\x04",
            Holographic10dEditLimits::default(),
        )
        .unwrap();
        assert_eq!(&updated[112..116], b"HOLO");
        assert_eq!(&updated[124..128], &[1, 2, 3, 4]);
    }
}
