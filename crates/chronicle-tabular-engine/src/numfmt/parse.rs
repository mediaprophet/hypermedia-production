//! Number format parser: sections, tokens, condition brackets.

pub const MAX_SECTIONS: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DatePart {
    Year(u8),
    Month(u8),
    Minute(u8),
    Day(u8),
    Hour(u8),
    Second(u8),
    SubSec(u8),
    AmPm,
}

impl DatePart {
    pub fn is_calendar(self) -> bool {
        matches!(self, DatePart::Year(_) | DatePart::Month(_) | DatePart::Day(_))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
    Lit(String),
    Digit(char),
    Point,
    Comma,
    Percent,
    Exp { upper: bool, plus: bool },
    Slash,
    FixedDenom(String),
    Fill(char),
    At,
    General,
    Date(DatePart),
}

impl Tok {
    pub fn literal_text(&self) -> String {
        match self {
            Tok::Lit(s) | Tok::FixedDenom(s) => s.clone(),
            Tok::Digit(c) => c.to_string(),
            Tok::Point => ".".into(),
            Tok::Comma => ",".into(),
            Tok::Percent => "%".into(),
            Tok::Exp { upper, plus } => format!("{}{}", if *upper { 'E' } else { 'e' }, if *plus { '+' } else { '-' }),
            Tok::Slash => "/".into(),
            Tok::Fill(_) | Tok::At | Tok::General | Tok::Date(_) => String::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecKind {
    General,
    Number,
    Date,
    Text,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FormatColor {
    Black,
    Blue,
    Cyan,
    Green,
    Magenta,
    Red,
    White,
    Yellow,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Section {
    pub kind: SecKind,
    pub toks: Vec<Tok>,
    pub color: Option<FormatColor>,
    pub percent: usize,
    pub exp: bool,
    pub fraction: bool,
    pub fill: bool,
    pub currency: bool,
    pub has_digits: bool,
    pub decimals: usize,
    pub thousands: bool,
}

pub fn parse_sections(code: &str) -> Vec<Section> {
    let mut sections = Vec::new();
    let parts: Vec<&str> = code.split(';').take(MAX_SECTIONS).collect();

    for part in parts {
        let p = part.trim();
        if p.eq_ignore_ascii_case("general") {
            sections.push(Section {
                kind: SecKind::General,
                toks: vec![Tok::General],
                color: None,
                percent: 0,
                exp: false,
                fraction: false,
                fill: false,
                currency: false,
                has_digits: false,
                decimals: 0,
                thousands: false,
            });
            continue;
        }

        let mut toks = Vec::new();
        let mut color = None;
        let mut percent = 0;
        let mut exp = false;
        let mut fraction = false;
        let mut fill = false;
        let mut currency = false;
        let mut has_digits = false;
        let mut decimals = 0;
        let mut in_decimals = false;
        let mut thousands = false;
        let mut is_date = false;

        let chars: Vec<char> = p.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            match c {
                '[' => {
                    let mut j = i + 1;
                    while j < chars.len() && chars[j] != ']' {
                        j += 1;
                    }
                    let tag: String = chars[i + 1..j].iter().collect();
                    let u = tag.to_ascii_uppercase();
                    if u == "RED" { color = Some(FormatColor::Red); }
                    else if u == "BLUE" { color = Some(FormatColor::Blue); }
                    else if u == "GREEN" { color = Some(FormatColor::Green); }
                    i = j + 1;
                }
                '"' => {
                    let mut j = i + 1;
                    let mut lit = String::new();
                    while j < chars.len() && chars[j] != '"' {
                        lit.push(chars[j]);
                        j += 1;
                    }
                    toks.push(Tok::Lit(lit));
                    i = j + 1;
                }
                '0' | '#' | '?' => {
                    has_digits = true;
                    if in_decimals {
                        decimals += 1;
                    }
                    toks.push(Tok::Digit(c));
                    i += 1;
                }
                '.' => {
                    in_decimals = true;
                    toks.push(Tok::Point);
                    i += 1;
                }
                ',' => {
                    thousands = true;
                    toks.push(Tok::Comma);
                    i += 1;
                }
                '%' => {
                    percent += 1;
                    toks.push(Tok::Percent);
                    i += 1;
                }
                '$' => {
                    currency = true;
                    toks.push(Tok::Lit("$".into()));
                    i += 1;
                }
                '*' => {
                    fill = true;
                    if i + 1 < chars.len() {
                        toks.push(Tok::Fill(chars[i + 1]));
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                '@' => {
                    toks.push(Tok::At);
                    i += 1;
                }
                '/' => {
                    fraction = true;
                    toks.push(Tok::Slash);
                    i += 1;
                }
                'y' | 'Y' => {
                    is_date = true;
                    let mut count = 0;
                    while i < chars.len() && (chars[i] == 'y' || chars[i] == 'Y') {
                        count += 1;
                        i += 1;
                    }
                    toks.push(Tok::Date(DatePart::Year(count)));
                }
                'm' | 'M' => {
                    is_date = true;
                    let mut count = 0;
                    while i < chars.len() && (chars[i] == 'm' || chars[i] == 'M') {
                        count += 1;
                        i += 1;
                    }
                    toks.push(Tok::Date(DatePart::Month(count)));
                }
                'd' | 'D' => {
                    is_date = true;
                    let mut count = 0;
                    while i < chars.len() && (chars[i] == 'd' || chars[i] == 'D') {
                        count += 1;
                        i += 1;
                    }
                    toks.push(Tok::Date(DatePart::Day(count)));
                }
                'h' | 'H' => {
                    is_date = true;
                    let mut count = 0;
                    while i < chars.len() && (chars[i] == 'h' || chars[i] == 'H') {
                        count += 1;
                        i += 1;
                    }
                    toks.push(Tok::Date(DatePart::Hour(count)));
                }
                's' | 'S' => {
                    is_date = true;
                    let mut count = 0;
                    while i < chars.len() && (chars[i] == 's' || chars[i] == 'S') {
                        count += 1;
                        i += 1;
                    }
                    toks.push(Tok::Date(DatePart::Second(count)));
                }
                other => {
                    toks.push(Tok::Lit(other.to_string()));
                    i += 1;
                }
            }
        }

        let kind = if is_date {
            SecKind::Date
        } else if toks.iter().any(|t| matches!(t, Tok::At)) {
            SecKind::Text
        } else {
            SecKind::Number
        };

        sections.push(Section {
            kind,
            toks,
            color,
            percent,
            exp,
            fraction,
            fill,
            currency,
            has_digits,
            decimals,
            thousands,
        });
    }

    if sections.is_empty() {
        sections.push(Section {
            kind: SecKind::General,
            toks: vec![Tok::General],
            color: None,
            percent: 0,
            exp: false,
            fraction: false,
            fill: false,
            currency: false,
            has_digits: false,
            decimals: 0,
            thousands: false,
        });
    }

    sections
}
