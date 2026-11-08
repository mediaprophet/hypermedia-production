//! Extended stroke table for lowercase Latin, digits, punctuation, Greek, and technical symbols.

use super::{SubpathBuilder, Y_BASE, Y_CAP, Y_DESC, Y_MID, Y_TOP, Y_X, Y_XMID};

pub(super) fn char_to_svg_subpath_ext(
    ch: char,
    b: &mut SubpathBuilder<'_>,
    xl: f64,
    xm: f64,
    xr: f64,
    w: f64,
) {
    match ch {
        'a' => {
            b.stroke(&[(xl, Y_X + 0.08), (xl + w * 0.2, Y_X), (xr, Y_X), (xr, Y_BASE)]);
            b.stroke(&[
                (xr, Y_XMID - 0.02),
                (xl + w * 0.15, Y_XMID - 0.02),
                (xl, Y_XMID + 0.08),
                (xl, Y_BASE - 0.08),
                (xl + w * 0.2, Y_BASE),
                (xr, Y_BASE),
            ]);
        }
        'b' => {
            b.stroke(&[(xl, Y_TOP), (xl, Y_BASE)]);
            b.stroke(&[
                (xl, Y_X),
                (xr - w * 0.15, Y_X),
                (xr, Y_X + 0.15),
                (xr, Y_BASE - 0.15),
                (xr - w * 0.15, Y_BASE),
                (xl, Y_BASE),
            ]);
        }
        'c' => {
            b.stroke(&[
                (xr, Y_X + 0.1),
                (xr - w * 0.2, Y_X),
                (xl + w * 0.2, Y_X),
                (xl, Y_X + 0.12),
                (xl, Y_BASE - 0.12),
                (xl + w * 0.2, Y_BASE),
                (xr - w * 0.2, Y_BASE),
                (xr, Y_BASE - 0.1),
            ]);
        }
        'd' => {
            b.stroke(&[(xr, Y_TOP), (xr, Y_BASE)]);
            b.stroke(&[
                (xr, Y_X),
                (xl + w * 0.15, Y_X),
                (xl, Y_X + 0.15),
                (xl, Y_BASE - 0.15),
                (xl + w * 0.15, Y_BASE),
                (xr, Y_BASE),
            ]);
        }
        'e' => {
            b.stroke(&[(xl, Y_XMID), (xr, Y_XMID)]);
            b.stroke(&[
                (xr, Y_XMID),
                (xr, Y_X + 0.1),
                (xm, Y_X),
                (xl, Y_X + 0.1),
                (xl, Y_BASE - 0.1),
                (xm, Y_BASE),
                (xr, Y_BASE - 0.08),
            ]);
        }
        'f' => {
            b.stroke(&[
                (xr, Y_TOP + 0.08),
                (xr - w * 0.3, Y_TOP),
                (xl + w * 0.3, Y_TOP + 0.05),
                (xl + w * 0.3, Y_BASE),
            ]);
            b.stroke(&[(xl, Y_X), (xr, Y_X)]);
        }
        'g' => {
            b.closed(&[
                (xr, Y_X),
                (xl + w * 0.15, Y_X),
                (xl, Y_X + 0.12),
                (xl, Y_BASE - 0.12),
                (xl + w * 0.15, Y_BASE),
                (xr, Y_BASE),
            ]);
            b.stroke(&[(xr, Y_BASE), (xr, Y_DESC - 0.08), (xm, Y_DESC), (xl, Y_DESC - 0.06)]);
        }
        'h' => {
            b.stroke(&[(xl, Y_TOP), (xl, Y_BASE)]);
            b.stroke(&[(xl, Y_X + 0.1), (xl + w * 0.2, Y_X), (xr - w * 0.2, Y_X), (xr, Y_X + 0.12), (xr, Y_BASE)]);
        }
        'i' => {
            b.stroke(&[(xm, Y_X), (xm, Y_BASE)]);
            b.stroke(&[(xm, Y_TOP + 0.04), (xm, Y_TOP + 0.08)]);
        }
        'j' => {
            b.stroke(&[(xr - w * 0.2, Y_X), (xr - w * 0.2, Y_DESC - 0.08), (xm, Y_DESC), (xl, Y_DESC - 0.06)]);
            b.stroke(&[(xr - w * 0.2, Y_TOP + 0.04), (xr - w * 0.2, Y_TOP + 0.08)]);
        }
        'k' => {
            b.stroke(&[(xl, Y_TOP), (xl, Y_BASE)]);
            b.stroke(&[(xr, Y_X), (xl, Y_XMID)]);
            b.stroke(&[(xl + w * 0.25, Y_XMID - 0.04), (xr, Y_BASE)]);
        }
        'l' => b.stroke(&[(xm, Y_TOP), (xm, Y_BASE - 0.06), (xr, Y_BASE)]),
        'm' => {
            b.stroke(&[(xl, Y_X), (xl, Y_BASE)]);
            b.stroke(&[(xl, Y_X + 0.1), (xl + w * 0.25, Y_X), (xm, Y_X + 0.1), (xm, Y_BASE)]);
            b.stroke(&[(xm, Y_X + 0.1), (xm + w * 0.25, Y_X), (xr, Y_X + 0.1), (xr, Y_BASE)]);
        }
        'n' => {
            b.stroke(&[(xl, Y_X), (xl, Y_BASE)]);
            b.stroke(&[(xl, Y_X + 0.1), (xl + w * 0.2, Y_X), (xr - w * 0.2, Y_X), (xr, Y_X + 0.12), (xr, Y_BASE)]);
        }
        'o' => b.closed(&[(xm, Y_X), (xl, Y_X + 0.15), (xl, Y_BASE - 0.15), (xm, Y_BASE), (xr, Y_BASE - 0.15), (xr, Y_X + 0.15)]),
        'p' => {
            b.stroke(&[(xl, Y_X), (xl, Y_DESC)]);
            b.stroke(&[(xl, Y_X), (xr - w * 0.15, Y_X), (xr, Y_X + 0.15), (xr, Y_BASE - 0.15), (xr - w * 0.15, Y_BASE), (xl, Y_BASE)]);
        }
        'q' => {
            b.stroke(&[(xr, Y_X), (xr, Y_DESC)]);
            b.stroke(&[(xr, Y_X), (xl + w * 0.15, Y_X), (xl, Y_X + 0.15), (xl, Y_BASE - 0.15), (xl + w * 0.15, Y_BASE), (xr, Y_BASE)]);
        }
        'r' => {
            b.stroke(&[(xl, Y_X), (xl, Y_BASE)]);
            b.stroke(&[(xl, Y_X + 0.1), (xl + w * 0.3, Y_X), (xr, Y_X + 0.08)]);
        }
        's' => b.stroke(&[(xr, Y_X + 0.08), (xr - w * 0.2, Y_X), (xl + w * 0.2, Y_X), (xl, Y_X + 0.1), (xr, Y_BASE - 0.1), (xr - w * 0.2, Y_BASE), (xl + w * 0.2, Y_BASE), (xl, Y_BASE - 0.08)]),
        't' => {
            b.stroke(&[(xm, Y_TOP + 0.08), (xm, Y_BASE - 0.08), (xr, Y_BASE)]);
            b.stroke(&[(xl, Y_X), (xr, Y_X)]);
        }
        'u' => {
            b.stroke(&[(xl, Y_X), (xl, Y_BASE - 0.12), (xl + w * 0.2, Y_BASE), (xr - w * 0.2, Y_BASE), (xr, Y_BASE - 0.12), (xr, Y_X)]);
            b.stroke(&[(xr, Y_X), (xr, Y_BASE)]);
        }
        'v' => b.stroke(&[(xl, Y_X), (xm, Y_BASE), (xr, Y_X)]),
        'w' => b.stroke(&[(xl, Y_X), (xl + w * 0.25, Y_BASE), (xm, Y_X + 0.14), (xr - w * 0.25, Y_BASE), (xr, Y_X)]),
        'x' => {
            b.stroke(&[(xl, Y_X), (xr, Y_BASE)]);
            b.stroke(&[(xr, Y_X), (xl, Y_BASE)]);
        }
        'y' => {
            b.stroke(&[(xl, Y_X), (xm, Y_BASE)]);
            b.stroke(&[(xr, Y_X), (xl, Y_DESC)]);
        }
        'z' => b.stroke(&[(xl, Y_X), (xr, Y_X), (xl, Y_BASE), (xr, Y_BASE)]),
        '0' => {
            b.closed(&[(xm, Y_CAP), (xl, Y_CAP + 0.15), (xl, Y_BASE - 0.15), (xm, Y_BASE), (xr, Y_BASE - 0.15), (xr, Y_CAP + 0.15)]);
            b.stroke(&[(xr - w * 0.15, Y_CAP + 0.12), (xl + w * 0.15, Y_BASE - 0.12)]);
        }
        '1' => {
            b.stroke(&[(xl + w * 0.15, Y_CAP + 0.12), (xm, Y_CAP), (xm, Y_BASE)]);
            b.stroke(&[(xl, Y_BASE), (xr, Y_BASE)]);
        }
        '2' => b.stroke(&[(xl, Y_CAP + 0.1), (xl + w * 0.2, Y_CAP), (xr - w * 0.15, Y_CAP), (xr, Y_CAP + 0.12), (xl, Y_BASE), (xr, Y_BASE)]),
        '3' => b.stroke(&[(xl, Y_CAP), (xr, Y_CAP), (xm, Y_MID), (xr, Y_MID + 0.12), (xm, Y_BASE), (xl, Y_BASE - 0.08)]),
        '4' => {
            b.stroke(&[(xr - w * 0.2, Y_BASE), (xr - w * 0.2, Y_CAP)]);
            b.stroke(&[(xr - w * 0.2, Y_CAP), (xl, Y_MID + 0.08), (xr, Y_MID + 0.08)]);
        }
        '5' => b.stroke(&[(xr, Y_CAP), (xl, Y_CAP), (xl, Y_MID - 0.02), (xr - w * 0.15, Y_MID - 0.02), (xr, Y_MID + 0.12), (xr, Y_BASE - 0.12), (xl + w * 0.15, Y_BASE), (xl, Y_BASE - 0.06)]),
        '6' => b.stroke(&[(xr - w * 0.15, Y_CAP + 0.08), (xl + w * 0.2, Y_CAP), (xl, Y_CAP + 0.2), (xl, Y_BASE - 0.15), (xm, Y_BASE), (xr, Y_BASE - 0.15), (xr, Y_MID + 0.06), (xl, Y_MID + 0.06)]),
        '7' => b.stroke(&[(xl, Y_CAP), (xr, Y_CAP), (xl + w * 0.15, Y_BASE)]),
        '8' => {
            b.closed(&[(xm, Y_CAP), (xl + w * 0.1, Y_CAP + 0.08), (xm, Y_MID), (xr - w * 0.1, Y_CAP + 0.08)]);
            b.closed(&[(xm, Y_MID), (xl, Y_BASE - 0.12), (xm, Y_BASE), (xr, Y_BASE - 0.12)]);
        }
        '9' => b.stroke(&[(xl, Y_MID), (xr, Y_MID), (xr, Y_CAP + 0.15), (xm, Y_CAP), (xl, Y_CAP + 0.15), (xl, Y_MID), (xr, Y_MID), (xr, Y_BASE - 0.15), (xl + w * 0.15, Y_BASE)]),
        '!' => {
            b.stroke(&[(xm, Y_CAP), (xm, Y_BASE - 0.16)]);
            b.stroke(&[(xm, Y_BASE - 0.04), (xm, Y_BASE)]);
        }
        '"' => {
            b.stroke(&[(xl + w * 0.25, Y_CAP), (xl + w * 0.25, Y_CAP + 0.18)]);
            b.stroke(&[(xr - w * 0.25, Y_CAP), (xr - w * 0.25, Y_CAP + 0.18)]);
        }
        '#' => {
            b.stroke(&[(xl + w * 0.25, Y_CAP), (xl + w * 0.25, Y_BASE)]);
            b.stroke(&[(xr - w * 0.25, Y_CAP), (xr - w * 0.25, Y_BASE)]);
            b.stroke(&[(xl, Y_MID - 0.1), (xr, Y_MID - 0.1)]);
            b.stroke(&[(xl, Y_MID + 0.1), (xr, Y_MID + 0.1)]);
        }
        '$' => {
            b.stroke(&[(xr, Y_CAP + 0.1), (xr - w * 0.2, Y_CAP), (xl + w * 0.2, Y_CAP), (xl, Y_CAP + 0.12), (xm, Y_MID), (xr, Y_MID + 0.12), (xr, Y_BASE - 0.12), (xr - w * 0.2, Y_BASE), (xl + w * 0.2, Y_BASE), (xl, Y_BASE - 0.1)]);
            b.stroke(&[(xm, Y_TOP), (xm, Y_DESC)]);
        }
        '%' => {
            b.stroke(&[(xr, Y_CAP), (xl, Y_BASE)]);
            b.closed(&[(xl + w * 0.25, Y_CAP), (xl + w * 0.05, Y_CAP + 0.08), (xl + w * 0.25, Y_CAP + 0.16), (xl + w * 0.45, Y_CAP + 0.08)]);
            b.closed(&[(xr - w * 0.25, Y_BASE - 0.16), (xr - w * 0.45, Y_BASE - 0.08), (xr - w * 0.25, Y_BASE), (xr - w * 0.05, Y_BASE - 0.08)]);
        }
        '&' => b.stroke(&[(xr, Y_BASE), (xm, Y_BASE), (xl, Y_BASE - 0.15), (xl, Y_MID + 0.08), (xr - w * 0.2, Y_CAP + 0.12), (xm, Y_CAP), (xl + w * 0.2, Y_CAP + 0.12), (xr, Y_BASE - 0.1), (xr, Y_BASE)]),
        '\'' => b.stroke(&[(xm, Y_CAP), (xm, Y_CAP + 0.18)]),
        '(' => b.stroke(&[(xr, Y_CAP), (xl, Y_MID), (xr, Y_BASE)]),
        ')' => b.stroke(&[(xl, Y_CAP), (xr, Y_MID), (xl, Y_BASE)]),
        '*' => {
            b.stroke(&[(xm, Y_MID - 0.18), (xm, Y_MID + 0.18)]);
            b.stroke(&[(xl, Y_MID - 0.09), (xr, Y_MID + 0.09)]);
            b.stroke(&[(xl, Y_MID + 0.09), (xr, Y_MID - 0.09)]);
        }
        '+' => {
            b.stroke(&[(xl, Y_MID), (xr, Y_MID)]);
            b.stroke(&[(xm, Y_MID - 0.22), (xm, Y_MID + 0.22)]);
        }
        ',' => b.stroke(&[(xm, Y_BASE - 0.04), (xm, Y_BASE + 0.06), (xm - w * 0.3, Y_BASE + 0.14)]),
        '-' => b.stroke(&[(xl, Y_MID), (xr, Y_MID)]),
        '.' => b.stroke(&[(xm, Y_BASE - 0.04), (xm, Y_BASE)]),
        '/' => b.stroke(&[(xl, Y_BASE), (xr, Y_CAP)]),
        ':' => {
            b.stroke(&[(xm, Y_XMID - 0.04), (xm, Y_XMID)]);
            b.stroke(&[(xm, Y_BASE - 0.04), (xm, Y_BASE)]);
        }
        ';' => {
            b.stroke(&[(xm, Y_XMID - 0.04), (xm, Y_XMID)]);
            b.stroke(&[(xm, Y_BASE - 0.04), (xm, Y_BASE + 0.06), (xm - w * 0.3, Y_BASE + 0.14)]);
        }
        '<' => b.stroke(&[(xr, Y_CAP + 0.08), (xl, Y_MID), (xr, Y_BASE - 0.08)]),
        '=' => {
            b.stroke(&[(xl, Y_MID - 0.08), (xr, Y_MID - 0.08)]);
            b.stroke(&[(xl, Y_MID + 0.08), (xr, Y_MID + 0.08)]);
        }
        '>' => b.stroke(&[(xl, Y_CAP + 0.08), (xr, Y_MID), (xl, Y_BASE - 0.08)]),
        '?' => {
            b.stroke(&[(xl, Y_CAP + 0.1), (xl + w * 0.2, Y_CAP), (xr - w * 0.2, Y_CAP), (xr, Y_CAP + 0.12), (xm, Y_MID), (xm, Y_BASE - 0.16)]);
            b.stroke(&[(xm, Y_BASE - 0.04), (xm, Y_BASE)]);
        }
        '@' => b.stroke(&[(xr, Y_BASE - 0.1), (xm, Y_BASE), (xl, Y_BASE - 0.2), (xl, Y_CAP + 0.2), (xm, Y_CAP), (xr, Y_CAP + 0.2), (xr, Y_MID), (xm, Y_MID), (xl + w * 0.3, Y_MID + 0.1), (xm, Y_MID + 0.15), (xr - w * 0.2, Y_MID + 0.1), (xr - w * 0.2, Y_BASE)]),
        '[' => b.stroke(&[(xr, Y_TOP), (xl, Y_TOP), (xl, Y_BASE + 0.06), (xr, Y_BASE + 0.06)]),
        '\\' => b.stroke(&[(xl, Y_CAP), (xr, Y_BASE)]),
        ']' => b.stroke(&[(xl, Y_TOP), (xr, Y_TOP), (xr, Y_BASE + 0.06), (xl, Y_BASE + 0.06)]),
        '^' => b.stroke(&[(xl, Y_CAP + 0.18), (xm, Y_CAP), (xr, Y_CAP + 0.18)]),
        '_' => b.stroke(&[(xl, Y_BASE + 0.08), (xr, Y_BASE + 0.08)]),
        '`' => b.stroke(&[(xm - w * 0.2, Y_CAP), (xm + w * 0.2, Y_CAP + 0.14)]),
        '{' => b.stroke(&[(xr, Y_TOP), (xm, Y_TOP), (xm, Y_MID - 0.06), (xl, Y_MID), (xm, Y_MID + 0.06), (xm, Y_BASE + 0.06), (xr, Y_BASE + 0.06)]),
        '|' => b.stroke(&[(xm, Y_TOP), (xm, Y_DESC)]),
        '}' => b.stroke(&[(xl, Y_TOP), (xm, Y_TOP), (xm, Y_MID - 0.06), (xr, Y_MID), (xm, Y_MID + 0.06), (xm, Y_BASE + 0.06), (xl, Y_BASE + 0.06)]),
        '~' => b.stroke(&[(xl, Y_MID + 0.04), (xl + w * 0.3, Y_MID - 0.06), (xr - w * 0.3, Y_MID + 0.06), (xr, Y_MID - 0.04)]),
        'α' => b.stroke(&[(xr, Y_X + 0.06), (xl + w * 0.1, Y_BASE), (xl, Y_X + 0.1), (xr, Y_BASE), (xr, Y_BASE - 0.12)]),
        'μ' => {
            b.stroke(&[(xl, Y_X), (xl, Y_DESC)]);
            b.stroke(&[(xl, Y_BASE - 0.12), (xm, Y_BASE), (xr, Y_BASE - 0.12), (xr, Y_X)]);
            b.stroke(&[(xr, Y_X), (xr, Y_BASE)]);
        }
        'σ' => {
            b.stroke(&[(xm, Y_X), (xr + w * 0.15, Y_X)]);
            b.closed(&[(xm, Y_X), (xl, Y_X + 0.12), (xl, Y_BASE - 0.12), (xm, Y_BASE), (xr, Y_BASE - 0.12), (xr, Y_X + 0.12)]);
        }
        'Δ' => b.closed(&[(xl, Y_BASE), (xm, Y_CAP), (xr, Y_BASE)]),
        'Ω' => {
            b.stroke(&[(xl, Y_BASE), (xl + w * 0.25, Y_BASE)]);
            b.stroke(&[(xl + w * 0.25, Y_BASE), (xl, Y_BASE - 0.18), (xl, Y_CAP + 0.2), (xm, Y_CAP), (xr, Y_CAP + 0.2), (xr, Y_BASE - 0.18), (xr - w * 0.25, Y_BASE)]);
            b.stroke(&[(xr - w * 0.25, Y_BASE), (xr, Y_BASE)]);
        }
        'π' => {
            b.stroke(&[(xl, Y_X), (xr, Y_X)]);
            b.stroke(&[(xl + w * 0.25, Y_X), (xl + w * 0.25, Y_BASE)]);
            b.stroke(&[(xr - w * 0.25, Y_X), (xr - w * 0.25, Y_BASE - 0.06), (xr, Y_BASE)]);
        }
        '±' => {
            b.stroke(&[(xl + w * 0.1, Y_MID - 0.12), (xr - w * 0.1, Y_MID - 0.12)]);
            b.stroke(&[(xm, Y_MID - 0.26), (xm, Y_MID + 0.02)]);
            b.stroke(&[(xl + w * 0.1, Y_BASE), (xr - w * 0.1, Y_BASE)]);
        }
        '°' => b.closed(&[(xm, Y_CAP), (xl + w * 0.15, Y_CAP + 0.07), (xm, Y_CAP + 0.14), (xr - w * 0.15, Y_CAP + 0.07)]),
        '→' => {
            b.stroke(&[(xl, Y_MID), (xr, Y_MID)]);
            b.stroke(&[(xr - w * 0.35, Y_MID - 0.18), (xr, Y_MID), (xr - w * 0.35, Y_MID + 0.18)]);
        }
        '↔' => {
            b.stroke(&[(xl, Y_MID), (xr, Y_MID)]);
            b.stroke(&[(xl + w * 0.3, Y_MID - 0.16), (xl, Y_MID), (xl + w * 0.3, Y_MID + 0.16)]);
            b.stroke(&[(xr - w * 0.3, Y_MID - 0.16), (xr, Y_MID), (xr - w * 0.3, Y_MID + 0.16)]);
        }
        '≤' => {
            b.stroke(&[(xr, Y_CAP + 0.08), (xl, Y_MID - 0.04), (xr, Y_BASE - 0.16)]);
            b.stroke(&[(xl, Y_BASE), (xr, Y_BASE)]);
        }
        '≥' => {
            b.stroke(&[(xl, Y_CAP + 0.08), (xr, Y_MID - 0.04), (xl, Y_BASE - 0.16)]);
            b.stroke(&[(xl, Y_BASE), (xr, Y_BASE)]);
        }
        '×' => {
            b.stroke(&[(xl + w * 0.15, Y_MID - 0.18), (xr - w * 0.15, Y_MID + 0.18)]);
            b.stroke(&[(xr - w * 0.15, Y_MID - 0.18), (xl + w * 0.15, Y_MID + 0.18)]);
        }
        '·' => b.stroke(&[(xm, Y_MID - 0.04), (xm, Y_MID + 0.04)]),
        '—' => b.stroke(&[(xl - 0.06, Y_MID), (xr + 0.06, Y_MID)]),
        _ => {
            b.closed(&[(xl, Y_CAP), (xr, Y_CAP), (xr, Y_BASE), (xl, Y_BASE)]);
            b.stroke(&[(xl, Y_CAP), (xr, Y_BASE)]);
            b.stroke(&[(xr, Y_CAP), (xl, Y_BASE)]);
        }
    }
}
