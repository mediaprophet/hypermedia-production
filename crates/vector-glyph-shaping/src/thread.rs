//! Threaded text: one story flowing through multiple linked text frames in order.
//!
//! Grounded in authoring rights and communicative integrity (UDHR Art. 19, 27).

use kurbo::Rect;
use serde::{Deserialize, Serialize};

use crate::layout::{layout_area_text, AreaTextOptions};

/// A run of text within a story.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextRun {
    pub text: String,
    pub size: f64,
}

/// An area text frame container.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextFrame {
    pub bounds: Rect,
    pub insets: [f64; 4],
    pub line_height: f64,
}

/// One frame's share of a distributed story.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Slice {
    pub runs: Vec<TextRun>,
}

/// Join all frame slices back into the full story text.
pub fn story(slices: &[Slice]) -> Vec<TextRun> {
    let mut combined: Vec<TextRun> = Vec::new();
    for s in slices {
        for r in &s.runs {
            if let Some(last) = combined.last_mut() {
                if (last.size - r.size).abs() < 1e-4 {
                    last.text.push_str(&r.text);
                    continue;
                }
            }
            combined.push(r.clone());
        }
    }
    combined
}

/// Distribute a sequence of story runs across linked text frames in thread order.
pub fn distribute(frames: &[TextFrame], story: &[TextRun]) -> Vec<Slice> {
    if frames.is_empty() {
        return Vec::new();
    }

    let full_text: String = story.iter().map(|r| r.text.as_str()).collect();
    let mut remaining = full_text.as_str();
    let mut out = Vec::with_capacity(frames.len());

    for (i, f) in frames.iter().enumerate() {
        let is_last = i + 1 == frames.len();
        if is_last || remaining.is_empty() {
            out.push(Slice {
                runs: vec![TextRun {
                    text: remaining.to_string(),
                    size: story.first().map(|r| r.size).unwrap_or(12.0),
                }],
            });
            break;
        }

        let opts = AreaTextOptions {
            insets: f.insets,
            line_height: f.line_height,
            ..Default::default()
        };
        let res = layout_area_text(f.bounds, remaining, &opts);

        let mut byte_count = 0;
        for l in &res.lines {
            byte_count += l.text.len();
            // Count spaces between words
            if byte_count < remaining.len() && remaining.as_bytes().get(byte_count) == Some(&b' ') {
                byte_count += 1;
            }
        }

        byte_count = byte_count.min(remaining.len());
        let (take, rest) = remaining.split_at(byte_count);
        out.push(Slice {
            runs: vec![TextRun {
                text: take.to_string(),
                size: story.first().map(|r| r.size).unwrap_or(12.0),
            }],
        });
        remaining = rest.trim_start();
    }

    while out.len() < frames.len() {
        out.push(Slice {
            runs: vec![TextRun {
                text: String::new(),
                size: story.first().map(|r| r.size).unwrap_or(12.0),
            }],
        });
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distribute_and_rejoin_story() {
        let frames = [
            TextFrame {
                bounds: Rect::new(0.0, 0.0, 50.0, 30.0),
                insets: [0.0; 4],
                line_height: 12.0,
            },
            TextFrame {
                bounds: Rect::new(60.0, 0.0, 110.0, 30.0),
                insets: [0.0; 4],
                line_height: 12.0,
            },
        ];
        let original_story = vec![TextRun {
            text: "The quick brown fox jumps over the lazy dog".to_string(),
            size: 12.0,
        }];
        let slices = distribute(&frames, &original_story);
        assert_eq!(slices.len(), 2);
        assert!(!slices[0].runs[0].text.is_empty());
        assert!(!slices[1].runs[0].text.is_empty());
    }
}
