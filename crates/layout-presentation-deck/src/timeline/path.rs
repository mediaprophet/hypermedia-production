//! Motion path parser and arc-length spline evaluator.

const CURVE_STEPS: usize = 16;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MotionPath {
    pub points: Vec<(f64, f64)>,
    pub cum_len: Vec<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tok {
    Cmd(char),
    Num(f64),
}

fn tokenize(s: &str) -> Vec<Tok> {
    let cs: Vec<char> = s.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < cs.len() {
        let c = cs[i];
        if c.is_whitespace() || c == ',' {
            i += 1;
            continue;
        }
        if c.is_ascii_digit() || c == '.' || c == '-' || c == '+' {
            let start = i;
            i += 1;
            let mut seen_dot = c == '.';
            while i < cs.len() {
                let d = cs[i];
                if d.is_ascii_digit() {
                    i += 1;
                } else if d == '.' && !seen_dot {
                    seen_dot = true;
                    i += 1;
                } else {
                    break;
                }
            }
            let s_num: String = cs[start..i].iter().collect();
            if let Ok(v) = s_num.parse::<f64>() {
                if v.is_finite() {
                    out.push(Tok::Num(v));
                }
            }
            continue;
        }
        out.push(Tok::Cmd(c));
        i += 1;
    }
    out
}

impl MotionPath {
    pub fn parse(s: &str) -> MotionPath {
        let toks = tokenize(s);
        let mut pts: Vec<(f64, f64)> = Vec::new();
        let mut cur = (0.0, 0.0);
        let mut start = (0.0, 0.0);
        let mut i = 0;

        while i < toks.len() {
            match toks[i] {
                Tok::Cmd(c) => {
                    i += 1;
                    match c {
                        'M' | 'm' => {
                            let rel = c == 'm';
                            if i + 1 < toks.len() {
                                if let (Tok::Num(x), Tok::Num(y)) = (toks[i], toks[i + 1]) {
                                    cur = if rel { (cur.0 + x, cur.1 + y) } else { (x, y) };
                                    start = cur;
                                    pts.push(cur);
                                    i += 2;
                                }
                            }
                        }
                        'L' | 'l' => {
                            let rel = c == 'l';
                            if i + 1 < toks.len() {
                                if let (Tok::Num(x), Tok::Num(y)) = (toks[i], toks[i + 1]) {
                                    cur = if rel { (cur.0 + x, cur.1 + y) } else { (x, y) };
                                    pts.push(cur);
                                    i += 2;
                                }
                            }
                        }
                        'C' | 'c' => {
                            let rel = c == 'c';
                            if i + 5 < toks.len() {
                                if let (
                                    Tok::Num(x1),
                                    Tok::Num(y1),
                                    Tok::Num(x2),
                                    Tok::Num(y2),
                                    Tok::Num(x),
                                    Tok::Num(y),
                                ) = (toks[i], toks[i + 1], toks[i + 2], toks[i + 3], toks[i + 4], toks[i + 5])
                                {
                                    let (p0x, p0y) = cur;
                                    let (p1x, p1y) = if rel { (cur.0 + x1, cur.1 + y1) } else { (x1, y1) };
                                    let (p2x, p2y) = if rel { (cur.0 + x2, cur.1 + y2) } else { (x2, y2) };
                                    let (p3x, p3y) = if rel { (cur.0 + x, cur.1 + y) } else { (x, y) };

                                    for step in 1..=CURVE_STEPS {
                                        let t = step as f64 / CURVE_STEPS as f64;
                                        let u = 1.0 - t;
                                        let px = u * u * u * p0x + 3.0 * u * u * t * p1x + 3.0 * u * t * t * p2x + t * t * t * p3x;
                                        let py = u * u * u * p0y + 3.0 * u * u * t * p1y + 3.0 * u * t * t * p2y + t * t * t * p3y;
                                        pts.push((px, py));
                                    }
                                    cur = (p3x, p3y);
                                    i += 6;
                                }
                            }
                        }
                        'Z' | 'z' => {
                            cur = start;
                            pts.push(cur);
                        }
                        _ => {}
                    }
                }
                _ => {
                    i += 1;
                }
            }
        }

        // Calculate cumulative length
        let mut cum_len = Vec::with_capacity(pts.len());
        let mut total = 0.0;
        cum_len.push(0.0);
        for w in pts.windows(2) {
            let dx = w[1].0 - w[0].0;
            let dy = w[1].1 - w[0].1;
            total += (dx * dx + dy * dy).sqrt();
            cum_len.push(total);
        }

        MotionPath { points: pts, cum_len }
    }

    pub fn total_length(&self) -> f64 {
        self.cum_len.last().copied().unwrap_or(0.0)
    }

    /// Evaluate position (fractional x, y) at progress t (0.0 .. 1.0).
    pub fn point_at(&self, t: f64) -> (f64, f64) {
        if self.points.is_empty() {
            return (0.0, 0.0);
        }
        if self.points.len() == 1 || t <= 0.0 {
            return self.points[0];
        }
        if t >= 1.0 {
            return *self.points.last().unwrap();
        }

        let tot = self.total_length();
        if tot <= 0.0 {
            return self.points[0];
        }

        let target_d = t * tot;
        let idx = match self.cum_len.binary_search_by(|d| d.partial_cmp(&target_d).unwrap()) {
            Ok(i) => return self.points[i],
            Err(i) => i,
        };

        if idx >= self.points.len() {
            return *self.points.last().unwrap();
        }
        let i0 = idx.saturating_sub(1);
        let i1 = idx;
        let d0 = self.cum_len[i0];
        let d1 = self.cum_len[i1];
        let span = d1 - d0;
        let seg_t = if span > 1e-9 { (target_d - d0) / span } else { 0.0 };

        let p0 = self.points[i0];
        let p1 = self.points[i1];
        (p0.0 + (p1.0 - p0.0) * seg_t, p0.1 + (p1.1 - p0.1) * seg_t)
    }
}
