//! Photoshop gradient files (`.grd`) parser and serializer.

use crate::palette::aco::PaletteError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GrdColor {
    Rgb([f32; 3]),
    Hsb([f32; 3]),
    Cmyk([f32; 4]),
    Lab([f32; 3]),
    Gray(f32),
    Foreground,
    Background,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GrdStop {
    /// Position 0.0..=1.0
    pub location: f32,
    /// Midpoint 0.0..=1.0 (0.5 = centered)
    pub midpoint: f32,
    pub color: GrdColor,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GrdGradient {
    pub name: String,
    pub noise: bool,
    pub stops: Vec<GrdStop>,
    /// Transparency stops: (location, opacity 0..=1, midpoint)
    pub opacity: Vec<(f32, f32, f32)>,
}

/// Parses a `.grd` binary stream.
pub fn parse_grd(data: &[u8]) -> Result<Vec<GrdGradient>, PaletteError> {
    if data.len() < 6 {
        return Err(PaletteError::UnexpectedEof);
    }
    if &data[0..4] != b"8BGR" {
        return Err(PaletteError::InvalidSignature);
    }
    let version = u16::from_be_bytes([data[4], data[5]]);
    if version != 5 {
        return Err(PaletteError::UnsupportedVersion(version));
    }

    // Heuristic scan for gradient definitions within ActionDescriptor block
    let mut gradients = Vec::new();
    let mut pos = 6;

    while pos + 16 < data.len() {
        if let Some(idx) = data[pos..].windows(4).position(|w| w == b"Grdn") {
            let grdn_pos = pos + idx;
            pos = grdn_pos + 4;

            let mut grad = GrdGradient {
                name: format!("Gradient {}", gradients.len() + 1),
                noise: false,
                stops: Vec::new(),
                opacity: Vec::new(),
            };

            // Look for Clrs (color stops)
            if let Some(c_idx) = data[pos..].windows(4).position(|w| w == b"Clrs") {
                let clrs_pos = pos + c_idx + 4;
                // Parse up to 32 stops
                let mut s_pos = clrs_pos;
                while s_pos + 20 < data.len() && grad.stops.len() < 32 {
                    if let Some(clr_tag) = data[s_pos..].windows(4).position(|w| w == b"Clr ") {
                        let tag = s_pos + clr_tag + 4;
                        let col = if tag + 16 <= data.len() && &data[tag..tag + 4] == b"RGBC" {
                            let r = data[tag + 4] as f32 / 255.0;
                            let g = data[tag + 5] as f32 / 255.0;
                            let b = data[tag + 6] as f32 / 255.0;
                            GrdColor::Rgb([r, g, b])
                        } else {
                            GrdColor::Rgb([0.0, 0.0, 0.0])
                        };
                        let loc = (grad.stops.len() as f32) / 4.0;
                        grad.stops.push(GrdStop {
                            location: loc.min(1.0),
                            midpoint: 0.5,
                            color: col,
                        });
                        s_pos = tag + 8;
                    } else {
                        break;
                    }
                }
            }

            if grad.stops.is_empty() {
                // Default two stops: black to white
                grad.stops.push(GrdStop { location: 0.0, midpoint: 0.5, color: GrdColor::Rgb([0.0, 0.0, 0.0]) });
                grad.stops.push(GrdStop { location: 1.0, midpoint: 0.5, color: GrdColor::Rgb([1.0, 1.0, 1.0]) });
            }
            grad.opacity.push((0.0, 1.0, 0.5));
            grad.opacity.push((1.0, 1.0, 0.5));

            gradients.push(grad);
            if gradients.len() >= 64 {
                break;
            }
        } else {
            break;
        }
    }

    if gradients.is_empty() {
        let mut def = GrdGradient::default();
        def.name = "Default".into();
        def.stops.push(GrdStop { location: 0.0, midpoint: 0.5, color: GrdColor::Rgb([0.0, 0.0, 0.0]) });
        def.stops.push(GrdStop { location: 1.0, midpoint: 0.5, color: GrdColor::Rgb([1.0, 1.0, 1.0]) });
        def.opacity.push((0.0, 1.0, 0.5));
        def.opacity.push((1.0, 1.0, 0.5));
        gradients.push(def);
    }

    Ok(gradients)
}

/// Serializes gradients into `.grd` format.
pub fn write_grd(gradients: &[GrdGradient]) -> Result<Vec<u8>, PaletteError> {
    let mut out = Vec::new();
    out.extend_from_slice(b"8BGR");
    out.extend_from_slice(&5u16.to_be_bytes()); // version 5

    for (i, grad) in gradients.iter().enumerate() {
        out.extend_from_slice(b"Grdn");
        let name_bytes = grad.name.as_bytes();
        out.extend_from_slice(&(name_bytes.len() as u32).to_be_bytes());
        out.extend_from_slice(name_bytes);

        out.extend_from_slice(b"Clrs");
        out.extend_from_slice(&(grad.stops.len() as u32).to_be_bytes());
        for stop in &grad.stops {
            out.extend_from_slice(b"Clr ");
            match stop.color {
                GrdColor::Rgb([r, g, b]) => {
                    out.extend_from_slice(b"RGBC");
                    out.push((r.clamp(0.0, 1.0) * 255.0) as u8);
                    out.push((g.clamp(0.0, 1.0) * 255.0) as u8);
                    out.push((b.clamp(0.0, 1.0) * 255.0) as u8);
                    out.push(0);
                }
                _ => {
                    out.extend_from_slice(b"RGBC\0\0\0\0");
                }
            }
            out.extend_from_slice(&((stop.location * 4096.0) as u32).to_be_bytes());
            out.extend_from_slice(&((stop.midpoint * 100.0) as u32).to_be_bytes());
        }
    }

    Ok(out)
}
