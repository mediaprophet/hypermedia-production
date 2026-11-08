//! CEA-608 (Line 21) closed caption coding, character sets, and screen buffer.
//!
//! Grounded in ANSI/CTA-608-E and FCC 47 CFR §15.119.

/// Add odd parity bit to a 7-bit value.
pub fn with_parity(b: u8) -> u8 {
    let b = b & 0x7f;
    if b.count_ones().is_multiple_of(2) {
        b | 0x80
    } else {
        b
    }
}

/// Verify and strip odd parity bit from a byte.
pub fn strip_parity(b: u8) -> Option<u8> {
    if b.count_ones().is_multiple_of(2) {
        None // Even number of ones is a parity error in odd parity
    } else {
        Some(b & 0x7f)
    }
}

/// Decode basic character set (0x20..=0x7F).
pub fn basic_char(b: u8) -> Option<char> {
    Some(match b {
        0x2a => 'á',
        0x5c => 'é',
        0x5e => 'í',
        0x5f => 'ó',
        0x60 => 'ú',
        0x7b => 'ç',
        0x7c => '÷',
        0x7d => 'Ñ',
        0x7e => 'ñ',
        0x7f => '█',
        0x20..=0x7e => b as char,
        _ => return None,
    })
}

pub const SPECIAL_CHARS: [char; 16] = [
    '®', '°', '½', '¿', '™', '¢', '£', '♪', 'à', ' ', 'è', 'â', 'ê', 'î', 'ô', 'û',
];

/// Decode a CEA-608 two-byte pair.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cea608Command {
    ResumeCaptionLoading,
    EndOfCaption,
    EraseDisplayedMemory,
    EraseNonDisplayedMemory,
    RollUp(u8),
    CarriageReturn,
    PreambleAddress { row: u8, indent: u8 },
    Text(String),
    Null,
}

pub fn decode_pair(b1_raw: u8, b2_raw: u8) -> Cea608Command {
    let b1 = match strip_parity(b1_raw) {
        Some(b) => b,
        None => return Cea608Command::Null,
    };
    let b2 = match strip_parity(b2_raw) {
        Some(b) => b,
        None => return Cea608Command::Null,
    };

    if b1 == 0 && b2 == 0 {
        return Cea608Command::Null;
    }

    // Control codes have b1 in 0x10..=0x1F
    if (0x10..=0x1f).contains(&b1) {
        match (b1 & 0x17, b2) {
            (0x14, 0x20) => Cea608Command::ResumeCaptionLoading,
            (0x14, 0x2f) => Cea608Command::EndOfCaption,
            (0x14, 0x2c) => Cea608Command::EraseDisplayedMemory,
            (0x14, 0x2e) => Cea608Command::EraseNonDisplayedMemory,
            (0x14, 0x25) => Cea608Command::RollUp(2),
            (0x14, 0x26) => Cea608Command::RollUp(3),
            (0x14, 0x27) => Cea608Command::RollUp(4),
            (0x14, 0x2d) => Cea608Command::CarriageReturn,
            (0x11, 0x30..=0x3f) => {
                let idx = (b2 - 0x30) as usize;
                Cea608Command::Text(SPECIAL_CHARS[idx].to_string())
            }
            _ => {
                // Preamble Address Code or Mid-Row
                if (0x10..=0x17).contains(&b1) && (0x40..=0x7f).contains(&b2) {
                    let row = match (b1 & 0x07, b2 & 0x20 != 0) {
                        (1, false) => 1,
                        (1, true) => 2,
                        (2, false) => 3,
                        (2, true) => 4,
                        (3, false) => 5,
                        (3, true) => 6,
                        (4, false) => 7,
                        (4, true) => 8,
                        (5, false) => 9,
                        (5, true) => 10,
                        (0, false) => 11,
                        (0, true) => 12,
                        (6, false) => 13,
                        (6, true) => 14,
                        (7, _) => 15,
                        _ => 15,
                    };
                    let indent = ((b2 & 0x0e) >> 1) * 4;
                    Cea608Command::PreambleAddress { row, indent }
                } else {
                    Cea608Command::Null
                }
            }
        }
    } else {
        // Basic characters
        let mut text = String::new();
        if let Some(c) = basic_char(b1) {
            text.push(c);
        }
        if let Some(c) = basic_char(b2) {
            text.push(c);
        }
        if text.is_empty() {
            Cea608Command::Null
        } else {
            Cea608Command::Text(text)
        }
    }
}

/// A 15-row x 32-column CEA-608 screen memory buffer.
#[derive(Clone, Debug, PartialEq)]
pub struct ScreenBuffer {
    pub rows: [[char; 32]; 15],
    pub cursor_row: usize,
    pub cursor_col: usize,
}

impl Default for ScreenBuffer {
    fn default() -> Self {
        Self {
            rows: [[' '; 32]; 15],
            cursor_row: 14,
            cursor_col: 0,
        }
    }
}

impl ScreenBuffer {
    pub fn clear(&mut self) {
        self.rows = [[' '; 32]; 15];
        self.cursor_row = 14;
        self.cursor_col = 0;
    }

    pub fn write_char(&mut self, c: char) {
        if self.cursor_row < 15 && self.cursor_col < 32 {
            self.rows[self.cursor_row][self.cursor_col] = c;
            self.cursor_col += 1;
        }
    }

    pub fn carriage_return(&mut self) {
        // Roll up rows 1..15 -> 0..14
        for r in 0..14 {
            self.rows[r] = self.rows[r + 1];
        }
        self.rows[14] = [' '; 32];
        self.cursor_col = 0;
    }

    pub fn render_text(&self) -> String {
        let mut lines = Vec::new();
        for r in 0..15 {
            let row_str: String = self.rows[r].iter().collect();
            let trimmed = row_str.trim();
            if !trimmed.is_empty() {
                lines.push(trimmed.to_string());
            }
        }
        lines.join("\n")
    }
}
