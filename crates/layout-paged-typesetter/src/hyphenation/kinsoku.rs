//! Kinsoku Shori (UAX #14) East Asian typography line-breaking rules.
//!
//! Enforces:
//! - No line may start with closing East Asian punctuation (、。，）」…);
//! - No line may end with opening punctuation (（「【…);
//! - Line-breaking opportunities between adjacent East Asian ideographs.

/// East Asian characters a line may NOT start with (closing brackets, commas, full stops).
pub const NO_START: &str = "、。，．・：；？！゛゜ヽヾゝゞ々〻〉》」』】〕〗〙〛〞〟）］｝｠｡｣､･ﾞﾟ％￠";

/// East Asian characters a line may NOT end with (opening brackets, quotes, currency signs).
pub const NO_END: &str = "〈《「『【〔〖〘〚〝（［｛｟｢＄￡￥";

/// Determines if character `c` is East Asian text (CJK ideographs, kana, bopomofo, CJK punctuation).
pub fn is_east_asian(c: char) -> bool {
    matches!(c as u32,
        0x2E80..=0x2FFF          // radicals, ideographic description
        | 0x3001..=0x303F        // CJK symbols and punctuation
        | 0x3040..=0x3098        // hiragana
        | 0x309B..=0x30FF        // sound marks, katakana
        | 0x3100..=0x312F        // bopomofo
        | 0x3190..=0x31FF        // kanbun, CJK strokes
        | 0x3400..=0x4DBF        // CJK extension A
        | 0x4E00..=0x9FFF        // CJK unified ideographs
        | 0xF900..=0xFAFF        // compatibility ideographs
        | 0xFE30..=0xFE4F        // CJK compatibility forms
        | 0xFF01..=0xFFEF        // fullwidth and halfwidth forms
        | 0x20000..=0x3FFFF      // supplementary ideographs
    )
}

/// Computes valid line-break opportunities (byte offsets) in `text` respecting Kinsoku rules.
pub fn line_break_opportunities(text: &str, kinsoku_on: bool) -> Vec<usize> {
    let mut opps = Vec::new();
    let char_indices: Vec<(usize, char)> = text.char_indices().collect();

    for i in 1..char_indices.len() {
        let (byte_pos, curr_char) = char_indices[i];
        let (_, prev_char) = char_indices[i - 1];

        if kinsoku_on {
            // Check forbidden start and forbidden end
            if NO_START.contains(curr_char) {
                continue;
            }
            if NO_END.contains(prev_char) {
                continue;
            }
        }

        // Standard space breaks
        if prev_char.is_whitespace() && !curr_char.is_whitespace() {
            opps.push(byte_pos);
            continue;
        }

        // East Asian text can break between ideographs
        if is_east_asian(prev_char) && is_east_asian(curr_char) {
            opps.push(byte_pos);
            continue;
        }

        // Boundary between Latin and East Asian
        if (is_east_asian(prev_char) && !curr_char.is_whitespace())
            || (!prev_char.is_whitespace() && is_east_asian(curr_char))
        {
            opps.push(byte_pos);
            continue;
        }
    }

    opps.sort_unstable();
    opps.dedup();
    opps
}
