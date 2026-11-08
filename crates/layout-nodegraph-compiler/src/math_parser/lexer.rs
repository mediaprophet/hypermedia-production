//! Lexer for mathematical expressions.

#![forbid(unsafe_code)]

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    LParen,
    RParen,
    Comma,
}

pub fn tokenize(s: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut i = 0;

    while i < n {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }

        match c {
            '+' => { tokens.push(Token::Plus); i += 1; }
            '-' => { tokens.push(Token::Minus); i += 1; }
            '*' => { tokens.push(Token::Star); i += 1; }
            '/' => { tokens.push(Token::Slash); i += 1; }
            '%' => { tokens.push(Token::Percent); i += 1; }
            '^' => { tokens.push(Token::Caret); i += 1; }
            '(' => { tokens.push(Token::LParen); i += 1; }
            ')' => { tokens.push(Token::RParen); i += 1; }
            ',' => { tokens.push(Token::Comma); i += 1; }
            d if d.is_ascii_digit() || (d == '.' && i + 1 < n && chars[i + 1].is_ascii_digit()) => {
                let start = i;
                let mut dot_seen = d == '.';
                i += 1;
                while i < n && (chars[i].is_ascii_digit() || (!dot_seen && chars[i] == '.')) {
                    if chars[i] == '.' {
                        dot_seen = true;
                    }
                    i += 1;
                }
                // Check for scientific notation exponent: 'e' or 'E' followed by optional sign and digits
                if i < n && (chars[i] == 'e' || chars[i] == 'E') {
                    let exp_start = i;
                    let mut j = i + 1;
                    if j < n && (chars[j] == '+' || chars[j] == '-') {
                        j += 1;
                    }
                    if j < n && chars[j].is_ascii_digit() {
                        while j < n && chars[j].is_ascii_digit() {
                            j += 1;
                        }
                        i = j;
                    } else {
                        // Not an exponent; treat 'e' as Euler's constant / variable
                        i = exp_start;
                    }
                }
                let num_str: String = chars[start..i].iter().collect();
                let val: f64 = num_str.parse().map_err(|e| format!("Invalid number `{num_str}`: {e}"))?;
                tokens.push(Token::Number(val));
            }
            alpha if alpha.is_alphabetic() || alpha == '_' || alpha == '°' => {
                let start = i;
                i += 1;
                while i < n && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '°') {
                    i += 1;
                }
                let name: String = chars[start..i].iter().collect();
                tokens.push(Token::Ident(name));
            }
            other => return Err(format!("Unexpected character `{other}` at offset {i}")),
        }
    }

    Ok(tokens)
}
