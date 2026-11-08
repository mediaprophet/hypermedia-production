//! GIMP Palette (`.gpl`) parser and serializer.

use crate::palette::aco::PaletteError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GplColor {
    pub rgb: [u8; 3],
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GplPalette {
    pub name: String,
    pub columns: u32,
    pub comments: Vec<String>,
    pub colors: Vec<GplColor>,
}

pub fn parse_gpl(data: &[u8]) -> Result<GplPalette, PaletteError> {
    let text = std::str::from_utf8(data).map_err(|e| PaletteError::Invalid(e.to_string()))?;
    let mut lines = text.lines();

    let first = lines.next().ok_or(PaletteError::UnexpectedEof)?;
    if !first.trim().eq_ignore_ascii_case("GIMP Palette") {
        return Err(PaletteError::InvalidSignature);
    }

    let mut palette = GplPalette::default();

    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix('#') {
            palette.comments.push(rest.trim().to_string());
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("Name:") {
            palette.name = rest.trim().to_string();
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("Columns:") {
            if let Ok(c) = rest.trim().parse::<u32>() {
                palette.columns = c;
            }
            continue;
        }

        // Color line: R G B [Name]
        let mut parts = trimmed.split_whitespace();
        let r = match parts.next().and_then(|s| s.parse::<u8>().ok()) {
            Some(v) => v,
            None => continue,
        };
        let g = match parts.next().and_then(|s| s.parse::<u8>().ok()) {
            Some(v) => v,
            None => continue,
        };
        let b = match parts.next().and_then(|s| s.parse::<u8>().ok()) {
            Some(v) => v,
            None => continue,
        };
        let name = parts.collect::<Vec<_>>().join(" ");

        palette.colors.push(GplColor {
            rgb: [r, g, b],
            name,
        });
    }

    Ok(palette)
}

pub fn write_gpl(palette: &GplPalette) -> Result<Vec<u8>, PaletteError> {
    let mut out = String::new();
    out.push_str("GIMP Palette\n");
    if !palette.name.is_empty() {
        out.push_str(&format!("Name: {}\n", palette.name));
    }
    if palette.columns > 0 {
        out.push_str(&format!("Columns: {}\n", palette.columns));
    }
    for c in &palette.comments {
        out.push_str(&format!("# {}\n", c));
    }
    out.push_str("#\n");
    for color in &palette.colors {
        if color.name.is_empty() {
            out.push_str(&format!("{:>3} {:>3} {:>3}\n", color.rgb[0], color.rgb[1], color.rgb[2]));
        } else {
            out.push_str(&format!(
                "{:>3} {:>3} {:>3}\t{}\n",
                color.rgb[0], color.rgb[1], color.rgb[2], color.name
            ));
        }
    }
    Ok(out.into_bytes())
}
