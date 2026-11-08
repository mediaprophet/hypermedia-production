//! CEA-708 (DTVCC) closed caption packet decoding and window management.
//!
//! Grounded in CTA-708-E and SMPTE ST 334-1.

/// A CEA-708 DTVCC service packet header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DtvccPacketHeader {
    pub sequence_number: u8,
    pub packet_size: usize,
}

pub fn parse_packet_header(b: u8) -> DtvccPacketHeader {
    let seq = (b >> 6) & 0x03;
    let size = (b & 0x3f) as usize;
    DtvccPacketHeader {
        sequence_number: seq,
        packet_size: if size == 0 { 128 } else { size * 2 },
    }
}

/// A CEA-708 command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DtvccCommand {
    SetCurrentWindow(u8),
    ClearWindows(u8),
    DisplayWindows(u8),
    HideWindows(u8),
    ToggleWindows(u8),
    DeleteWindows(u8),
    Reset,
    DefineWindow {
        window_id: u8,
        priority: u8,
        anchor_vertical: u8,
        anchor_horizontal: u8,
        row_count: u8,
        col_count: u8,
        visible: bool,
    },
    Text(String),
}

/// A CEA-708 window state.
#[derive(Clone, Debug, PartialEq)]
pub struct DtvccWindow {
    pub id: u8,
    pub visible: bool,
    pub row_count: usize,
    pub col_count: usize,
    pub text: String,
}

impl DtvccWindow {
    pub fn new(id: u8, rows: usize, cols: usize, visible: bool) -> Self {
        Self {
            id,
            visible,
            row_count: rows,
            col_count: cols,
            text: String::new(),
        }
    }

    pub fn clear(&mut self) {
        self.text.clear();
    }

    pub fn append(&mut self, s: &str) {
        self.text.push_str(s);
    }
}
