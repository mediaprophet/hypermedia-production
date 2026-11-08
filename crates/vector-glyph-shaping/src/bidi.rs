//! UAX #9 Unicode Bidirectional algorithm run ordering.
//!
//! Grounded in universal linguistic participation and cultural expression (UDHR Art. 19, 27; ICESCR Art. 15).

use serde::{Deserialize, Serialize};

/// Basic bidirectional character classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BidiClass {
    L,   // Left-to-Right
    R,   // Right-to-Left
    AL,  // Arabic Letter
    EN,  // European Number
    ES,  // European Number Separator
    ET,  // European Number Terminator
    AN,  // Arabic Number
    CS,  // Common Number Separator
    B,   // Paragraph Separator
    S,   // Segment Separator
    WS,  // Whitespace
    ON,  // Other Neutrals
}

/// Character classification according to Unicode bidirectional rules.
pub fn bidi_class(c: char) -> BidiClass {
    let u = c as u32;
    match u {
        // Arabic, Hebrew, Syriac, Thaana
        0x0590..=0x05FF | 0x0750..=0x077F | 0x08A0..=0x08FF | 0xFB50..=0xFDFF | 0xFE70..=0xFEFF => BidiClass::AL,
        0x0600..=0x074F => BidiClass::R,
        // ASCII digits
        0x0030..=0x0039 => BidiClass::EN,
        // Common separators
        0x002B | 0x002D => BidiClass::ES, // + -
        0x0023 | 0x0024 | 0x0025 | 0x00A2..=0x00A5 => BidiClass::ET, // # $ % etc
        0x002C | 0x002E | 0x003A | 0x002F => BidiClass::CS, // , . : /
        // Whitespace and separators
        0x0009 | 0x0020 => BidiClass::WS,
        0x000A | 0x000D | 0x001C..=0x001E => BidiClass::B,
        // Default to Left-to-Right for Latin, Greek, Cyrillic, CJK, etc.
        0x0041..=0x005A | 0x0061..=0x007A | 0x00C0..=0x02AF | 0x0370..=0x052F | 0x2E80..=0x9FFF | 0xAC00..=0xD7AF => {
            BidiClass::L
        }
        _ => BidiClass::ON,
    }
}

/// Direction of a paragraph or run.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Ltr,
    Rtl,
}

/// A contiguous run of text with constant embedding level.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BidiRun {
    pub start: usize,
    pub end: usize,
    pub level: u8,
}

impl BidiRun {
    pub fn is_rtl(&self) -> bool {
        self.level % 2 != 0
    }
}

/// Resolve paragraph direction and decompose text into directional runs.
pub fn resolve_bidi_runs(text: &str, base_direction: Option<Direction>) -> (Direction, Vec<BidiRun>) {
    if text.is_empty() {
        return (base_direction.unwrap_or(Direction::Ltr), Vec::new());
    }

    let base_dir = base_direction.unwrap_or_else(|| {
        for ch in text.chars() {
            match bidi_class(ch) {
                BidiClass::AL | BidiClass::R => return Direction::Rtl,
                BidiClass::L => return Direction::Ltr,
                _ => {}
            }
        }
        Direction::Ltr
    });

    let base_level: u8 = if base_dir == Direction::Rtl { 1 } else { 0 };

    let mut levels = Vec::with_capacity(text.len());
    let mut current_level = base_level;

    for ch in text.chars() {
        let cl = bidi_class(ch);
        let lvl = match cl {
            BidiClass::AL | BidiClass::R => 1,
            BidiClass::L => 0,
            BidiClass::EN | BidiClass::AN => {
                if base_level == 1 {
                    2
                } else {
                    0
                }
            }
            _ => current_level,
        };
        current_level = lvl;
        for _ in 0..ch.len_utf8() {
            levels.push(lvl);
        }
    }

    // Coalesce into runs
    let mut runs = Vec::new();
    let mut run_start = 0;
    let mut run_level = levels[0];

    for (i, &lvl) in levels.iter().enumerate() {
        if lvl != run_level {
            runs.push(BidiRun {
                start: run_start,
                end: i,
                level: run_level,
            });
            run_start = i;
            run_level = lvl;
        }
    }
    runs.push(BidiRun {
        start: run_start,
        end: levels.len(),
        level: run_level,
    });

    (base_dir, runs)
}

/// Reorder run indices from logical reading order into visual left-to-right display order.
pub fn reorder_visual(levels: &[u8]) -> Vec<usize> {
    if levels.is_empty() {
        return Vec::new();
    }
    let mut order: Vec<usize> = (0..levels.len()).collect();
    let max_level = *levels.iter().max().unwrap_or(&0);
    let min_odd_level = levels
        .iter()
        .filter(|&&l| l % 2 != 0)
        .min()
        .copied()
        .unwrap_or(max_level + 1);

    for lvl in (min_odd_level..=max_level).rev() {
        let mut i = 0;
        while i < levels.len() {
            if levels[order[i]] >= lvl {
                let start = i;
                while i < levels.len() && levels[order[i]] >= lvl {
                    i += 1;
                }
                order[start..i].reverse();
            } else {
                i += 1;
            }
        }
    }
    order
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_english_is_ltr() {
        let text = "Hello world";
        let (dir, runs) = resolve_bidi_runs(text, None);
        assert_eq!(dir, Direction::Ltr);
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].level, 0);
    }

    #[test]
    fn arabic_text_is_rtl() {
        let text = "مرحبا";
        let (dir, runs) = resolve_bidi_runs(text, None);
        assert_eq!(dir, Direction::Rtl);
        assert_eq!(runs[0].level, 1);
    }

    #[test]
    fn mixed_ltr_rtl_reordering() {
        let levels = [0, 0, 1, 1, 0];
        let visual = reorder_visual(&levels);
        assert_eq!(visual, vec![0, 1, 3, 2, 4]);
    }
}
