//! Syllable breakpoint engine and hyphenation dictionary.
//!
//! Grounded in typographic heritage and linguistic cultural rights (UDHR Art. 19, 27; ICESCR Art. 15).

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, OnceLock};

pub const MIN_WORD: usize = 6;
pub const MIN_BEFORE: usize = 3;
pub const MIN_AFTER: usize = 3;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Exception {
    Never,
    Only(Vec<usize>),
}

#[derive(Default)]
struct Exceptions {
    source: String,
    map: HashMap<String, Exception>,
}

fn store() -> MutexGuard<'static, Exceptions> {
    static LOCK: OnceLock<Mutex<Exceptions>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(Exceptions::default()))
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn parse_list(source: &str) -> Exceptions {
    let mut map = HashMap::new();
    for entry in source.split(',') {
        let entry = entry.trim();
        if entry.is_empty() {
            continue;
        }
        let (word, ex) = parse_exception(entry);
        if word.is_empty() {
            continue;
        }
        map.insert(word, ex);
    }
    Exceptions {
        source: source.to_string(),
        map,
    }
}

pub fn set_hyphenation_exceptions(source: &str) {
    *store() = parse_list(source);
}

pub fn hyphenation_exceptions() -> String {
    store().source.clone()
}

fn parse_exception(entry: &str) -> (String, Exception) {
    if !entry.contains('-') {
        return (normalize_word(entry), Exception::Never);
    }
    let mut word = String::new();
    let mut points = Vec::new();
    for ch in entry.chars() {
        if ch == '-' {
            let at = word.chars().count();
            if at > 0 {
                points.push(at);
            }
        } else {
            for c in ch.to_lowercase() {
                word.push(c);
            }
        }
    }
    let n = word.chars().count();
    points.retain(|&p| p > 0 && p < n);
    points.sort_unstable();
    points.dedup();
    (word, Exception::Only(points))
}

fn normalize_word(word: &str) -> String {
    word.chars().flat_map(|c| c.to_lowercase()).collect()
}

fn exception_for(word: &str) -> Option<Exception> {
    store().map.get(&normalize_word(word)).cloned()
}

fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u' | 'y') || (c.is_alphabetic() && !c.is_ascii())
}

fn onset_pair(a: char, b: char) -> bool {
    let (a, b) = (a.to_ascii_lowercase(), b.to_ascii_lowercase());
    matches!(
        (a, b),
        ('c' | 's' | 't' | 'p' | 'w' | 'g', 'h')
            | ('b' | 'c' | 'd' | 'f' | 'g' | 'k' | 'p' | 't', 'r')
            | ('b' | 'c' | 'f' | 'g' | 'k' | 'p' | 's', 'l')
            | ('q', 'u')
    )
}

/// Allowed hyphenation points in `word` as char indices (hyphen inserted before char).
pub fn hyphen_points(word: &str) -> Vec<usize> {
    if let Some(ex) = exception_for(word) {
        return match ex {
            Exception::Never => vec![],
            Exception::Only(pts) => pts,
        };
    }
    let c: Vec<char> = word.chars().collect();
    let n = c.len();
    if n < MIN_WORD || !c.iter().all(|c| c.is_alphabetic() || matches!(c, '\'' | '’')) {
        return vec![];
    }
    if c.iter().all(|c| c.is_uppercase()) {
        return vec![];
    }
    let v: Vec<bool> = c.iter().map(|&x| is_vowel(x)).collect();
    let mut out = vec![];
    for i in MIN_BEFORE..=n.saturating_sub(MIN_AFTER) {
        if !c[i].is_alphabetic() || !c[i - 1].is_alphabetic() {
            continue;
        }
        let ok = if v[i - 1] && !v[i] {
            (i + 1 < n && v[i + 1]) || (i + 2 < n && onset_pair(c[i], c[i + 1]) && v[i + 2])
        } else if !v[i - 1] && !v[i] {
            i >= 2 && v[i - 2] && i + 1 < n && v[i + 1] && !onset_pair(c[i - 1], c[i])
        } else {
            false
        };
        if ok {
            out.push(i);
        }
    }
    out
}

pub fn hyphenate_word(word: &str) -> String {
    let pts = hyphen_points(word);
    let mut s = String::with_capacity(word.len() + pts.len());
    for (i, ch) in word.chars().enumerate() {
        if pts.contains(&i) {
            s.push('-');
        }
        s.push(ch);
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn syllable_splitting_rules() {
        let pts = hyphen_points("typography");
        assert_eq!(pts, vec![4, 7]);
        assert_eq!(hyphenate_word("typography"), "typo-gra-phy");
    }

    #[test]
    fn short_words_do_not_hyphenate() {
        assert!(hyphen_points("short").is_empty());
        assert!(hyphen_points("cat").is_empty());
    }
}
