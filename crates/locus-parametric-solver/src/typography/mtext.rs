//! MTEXT: formatting parser, control code decoder, stacked fractions, and paragraph layout.

#![forbid(unsafe_code)]

use drafting_geometry::{Bounds2, Vec2};
use super::stroke::{char_advance, layout_stroke_line};

/// Decode `%%` control codes:
/// - `%%d`: Degree symbol `°`
/// - `%%p`: Plus/minus symbol `±`
/// - `%%c`: Circle diameter symbol `⌀`
/// - `%%%`: Literal percent `%`
/// - `%%u`: Underline toggle flag
/// - `%%o`: Overline toggle flag
/// - `%%nnn`: ASCII char code (e.g. `%%065` -> 'A')
pub fn decode_controls(s: &str) -> Vec<(char, bool, bool)> {
    let mut out = Vec::with_capacity(s.len());
    let mut under = false;
    let mut over = false;
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars.get(i).copied().unwrap_or(' ');
        if c == '%' && chars.get(i + 1) == Some(&'%') {
            let code = chars.get(i + 2).copied().unwrap_or(' ');
            match code.to_ascii_lowercase() {
                'd' => out.push(('°', under, over)),
                'p' => out.push(('±', under, over)),
                'c' => out.push(('⌀', under, over)),
                '%' => out.push(('%', under, over)),
                'u' => under = !under,
                'o' => over = !over,
                d if d.is_ascii_digit() => {
                    let digits: String = chars.iter().skip(i + 2).take(3).take_while(|c| c.is_ascii_digit()).collect();
                    let n: u32 = digits.parse().unwrap_or(32);
                    out.push((char::from_u32(n).unwrap_or('?'), under, over));
                    i += 2 + digits.len();
                    continue;
                }
                _ => {
                    out.push(('%', under, over));
                    i += 1;
                    continue;
                }
            }
            i += 3;
            continue;
        }
        out.push((c, under, over));
        i += 1;
    }
    out
}

/// Convert MTEXT formatting codes to plain text with `\n` line breaks.
/// Stacked fractions `\S1/2;`, `\S1^2;`, `\S1#2;` become `1/2`; `\~` is a non-breaking space.
pub fn plain_mtext(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while let Some(&c) = chars.get(i) {
        match c {
            '\\' => {
                let code = chars.get(i + 1).copied().unwrap_or(' ');
                match code {
                    'P' | 'N' => {
                        out.push('\n');
                        i += 2;
                    }
                    '~' => {
                        out.push(' ');
                        i += 2;
                    }
                    '\\' | '{' | '}' => {
                        out.push(code);
                        i += 2;
                    }
                    'S' => {
                        let mut j = i + 2;
                        let mut stack = String::new();
                        while let Some(&ch) = chars.get(j) {
                            if ch == ';' {
                                break;
                            }
                            stack.push(match ch {
                                '^' | '#' => '/',
                                other => other,
                            });
                            j += 1;
                        }
                        if out.chars().last().is_some_and(|ch| ch.is_ascii_digit())
                            && stack.starts_with(|ch: char| ch.is_ascii_digit())
                        {
                            out.push(' ');
                        }
                        out.push_str(stack.trim_end_matches('/'));
                        i = j + 1;
                    }
                    'L' | 'l' | 'O' | 'o' | 'K' | 'k' => i += 2,
                    _ => {
                        let mut j = i + 2;
                        while let Some(&ch) = chars.get(j) {
                            if ch == ';' {
                                break;
                            }
                            j += 1;
                        }
                        i = j + 1;
                    }
                }
            }
            '{' | '}' => i += 1,
            '\r' => i += 1,
            _ => {
                out.push(c);
                i += 1;
            }
        }
    }
    out
}

/// MTEXT layout parameters.
#[derive(Clone, Debug)]
pub struct MTextParams {
    pub insert: Vec2,
    pub height: f64,
    /// Column width for word wrapping; <= 0.0 disables wrapping.
    pub width: f64,
    /// 1..=9: TL(1), TC(2), TR(3), ML(4), MC(5), MR(6), BL(7), BC(8), BR(9).
    pub attach: u8,
    pub rotation: f64,
    pub line_spacing: f64,
    pub width_factor: f64,
}

impl Default for MTextParams {
    fn default() -> Self {
        MTextParams {
            insert: Vec2::ZERO,
            height: 1.0,
            width: 0.0,
            attach: 1, // TL
            rotation: 0.0,
            line_spacing: 1.5,
            width_factor: 1.0,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MTextPiece {
    pub text: String,
    pub origin: Vec2,
    pub width: f64,
    pub height: f64,
    pub strokes: Vec<Vec<Vec2>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MTextLayout {
    pub pieces: Vec<MTextPiece>,
    pub bounds: Bounds2,
    pub total_width: f64,
    pub total_height: f64,
}

/// Layout an MTEXT string into positioned lines and strokes.
pub fn layout_mtext(text: &str, params: &MTextParams) -> MTextLayout {
    let plain = plain_mtext(text);
    let mut raw_lines: Vec<String> = Vec::new();

    // Decode control codes per line
    for l in plain.split('\n') {
        let decoded: String = decode_controls(l).into_iter().map(|(c, _, _)| c).collect();
        raw_lines.push(decoded);
    }

    // Word wrap if column width is positive
    let mut wrapped_lines = Vec::new();
    let col_w = if params.width > 0.0 { params.width } else { f64::INFINITY };

    for line in raw_lines {
        if line.is_empty() {
            wrapped_lines.push(String::new());
            continue;
        }
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.is_empty() {
            wrapped_lines.push(String::new());
            continue;
        }

        let mut cur = String::new();
        let mut cur_w = 0.0;

        for word in words {
            let word_w: f64 = word.chars().map(|c| char_advance(c) * params.height * params.width_factor).sum();
            let space_w = char_advance(' ') * params.height * params.width_factor;

            if cur.is_empty() {
                cur.push_str(word);
                cur_w = word_w;
            } else if cur_w + space_w + word_w <= col_w {
                cur.push(' ');
                cur.push_str(word);
                cur_w += space_w + word_w;
            } else {
                wrapped_lines.push(cur);
                cur = word.to_string();
                cur_w = word_w;
            }
        }
        if !cur.is_empty() {
            wrapped_lines.push(cur);
        }
    }

    let line_h = params.height * params.line_spacing;
    let n_lines = wrapped_lines.len().max(1);
    let total_h = (n_lines as f64) * line_h;
    let mut max_w: f64 = 0.0;

    let mut line_layouts = Vec::new();
    for l in &wrapped_lines {
        let sl = layout_stroke_line(l, params.height, params.width_factor, 0.0);
        if sl.width > max_w {
            max_w = sl.width;
        }
        line_layouts.push((l.clone(), sl));
    }

    let box_w = if params.width > 0.0 { params.width } else { max_w };

    // Compute attachment offset relative to params.insert
    let (ox, oy) = match params.attach {
        1 => (0.0, 0.0),                     // TL
        2 => (-box_w * 0.5, 0.0),             // TC
        3 => (-box_w, 0.0),                  // TR
        4 => (0.0, total_h * 0.5),           // ML
        5 => (-box_w * 0.5, total_h * 0.5),   // MC
        6 => (-box_w, total_h * 0.5),        // MR
        7 => (0.0, total_h),                 // BL
        8 => (-box_w * 0.5, total_h),         // BC
        9 => (-box_w, total_h),              // BR
        _ => (0.0, 0.0),
    };

    let mut pieces = Vec::new();
    let mut min_x = f64::INFINITY;
    let mut max_x = -f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = -f64::INFINITY;

    for (idx, (txt, sl)) in line_layouts.into_iter().enumerate() {
        let y_local = oy - (idx as f64 + 1.0) * line_h;
        let x_local = ox;

        let mut transformed_strokes = Vec::new();
        for stroke in sl.strokes {
            let tr: Vec<Vec2> = stroke
                .into_iter()
                .map(|p| {
                    let lx = x_local + p.x;
                    let ly = y_local + p.y;
                    // Apply rotation around params.insert
                    let rot_p = Vec2::new(lx, ly).rotate(params.rotation);
                    let final_p = params.insert + rot_p;
                    if final_p.x < min_x { min_x = final_p.x; }
                    if final_p.x > max_x { max_x = final_p.x; }
                    if final_p.y < min_y { min_y = final_p.y; }
                    if final_p.y > max_y { max_y = final_p.y; }
                    final_p
                })
                .collect();
            transformed_strokes.push(tr);
        }

        let piece_origin = params.insert + Vec2::new(x_local, y_local).rotate(params.rotation);
        pieces.push(MTextPiece {
            text: txt,
            origin: piece_origin,
            width: sl.width,
            height: params.height,
            strokes: transformed_strokes,
        });
    }

    let bounds = if min_x <= max_x {
        Bounds2::new(Vec2::new(min_x, min_y), Vec2::new(max_x, max_y))
    } else {
        Bounds2::new(params.insert, params.insert)
    };

    MTextLayout {
        pieces,
        bounds,
        total_width: max_w,
        total_height: total_h,
    }
}
