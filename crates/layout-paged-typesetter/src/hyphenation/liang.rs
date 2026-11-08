//! Liang hyphenation pattern engine (TeX algorithm).

use std::collections::HashMap;

pub const DOT: char = '.';
pub const MAX_PATTERN_LEN: usize = 9;

#[derive(Clone, Debug, Default)]
pub struct Patterns {
    pub map: HashMap<String, Vec<u8>>,
    pub max_len: usize,
}

impl Patterns {
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse TeX-style patterns, whitespace-separated (e.g. `hy3ph he2n hena4`).
    pub fn parse(text: &str) -> Patterns {
        let mut map = HashMap::new();
        let mut max_len = 0;

        for word in text.split_whitespace() {
            if word.starts_with('%') || word.starts_with('#') {
                continue;
            }
            let mut letters = String::new();
            let mut vals = Vec::new();
            let mut last_was_digit = false;

            for c in word.chars() {
                if let Some(d) = c.to_digit(10) {
                    vals.push(d as u8);
                    last_was_digit = true;
                } else {
                    if !last_was_digit {
                        vals.push(0);
                    }
                    letters.push(c);
                    last_was_digit = false;
                }
            }
            if !last_was_digit {
                vals.push(0);
            }

            max_len = max_len.max(letters.chars().count());
            map.insert(letters, vals);
        }

        Patterns { map, max_len }
    }

    /// Standard English patterns compiled for common typography.
    pub fn english_standard() -> Patterns {
        // Core subset of Liang's patterns
        let pats = "
        .ach4 .ad4der .ade1 .adn4 .ad4van .ae1r .af1t .al3t .am5at .an5c .ang4 .ani5m .ant4
        .an3te .anti4 .ar5s .ar4t .as1c .as1p .asp4 .at1 .au4d .av4i .ba4g .ba5n .be1e .be1i
        .be1o .be1u .bi3b .bi3n .bla4 .bo4r .bu4t .by1 .ca3l .ca1n .ca4r .cat4 .ch4 .ch1r .co1
        .cor5n .de1 .di3 .di1s .dr4 .ea4 .ed1 .en1 .eq4 .er4 .es1 .eu4 .fa3 .fe4 .fo4r .fu3
        .ge1 .go1 .gre3 .he2n hena4 hen5at 1na n2at 1tio 2io o2n hy3ph 4t1ion 2pe 4ple
        ";
        Patterns::parse(pats)
    }

    /// Break points (character indices where a hyphen may be inserted before that char)
    /// for a lowercase word.
    pub fn points(&self, word: &str) -> Vec<usize> {
        let chars: Vec<char> = word.to_lowercase().chars().collect();
        let n = chars.len();
        if n < 4 {
            return Vec::new();
        }

        let mut padded = Vec::with_capacity(n + 2);
        padded.push(DOT);
        padded.extend_from_slice(&chars);
        padded.push(DOT);

        let mut vals = vec![0u8; padded.len() + 1];

        for start in 0..padded.len() {
            for len in 1..=self.max_len.min(padded.len() - start) {
                let sub: String = padded[start..start + len].iter().collect();
                if let Some(v) = self.map.get(&sub) {
                    for (j, &x) in v.iter().enumerate() {
                        if start + j < vals.len() && x > vals[start + j] {
                            vals[start + j] = x;
                        }
                    }
                }
            }
        }

        let mut out = Vec::new();
        // Skip first 2 and last 2 characters to avoid orphan letters
        for i in 2..n.saturating_sub(1) {
            let val_idx = i + 1;
            if val_idx < vals.len() && vals[val_idx] % 2 == 1 {
                out.push(i);
            }
        }
        out
    }

    /// Hyphenates a word, inserting hyphen strings.
    pub fn hyphenate(&self, word: &str, hyphen: &str) -> String {
        let pts = self.points(word);
        if pts.is_empty() {
            return word.to_string();
        }
        let chars: Vec<char> = word.chars().collect();
        let mut out = String::new();
        for (i, c) in chars.iter().enumerate() {
            if pts.contains(&i) {
                out.push_str(hyphen);
            }
            out.push(*c);
        }
        out
    }
}
