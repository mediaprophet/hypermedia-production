//! RFC 4180 CSV and TSV reader and writer with delimiter sniffing and quotation handling.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Delimiter {
    Comma,
    Tab,
    Semicolon,
    Pipe,
}

impl Delimiter {
    pub fn as_char(&self) -> char {
        match self {
            Delimiter::Comma => ',',
            Delimiter::Tab => '\t',
            Delimiter::Semicolon => ';',
            Delimiter::Pipe => '|',
        }
    }
}

/// Sniff the most likely delimiter based on counts in the first line.
pub fn sniff_delimiter(input: &str) -> Delimiter {
    let first_line = input.lines().next().unwrap_or("");
    let comma = first_line.chars().filter(|&c| c == ',').count();
    let tab = first_line.chars().filter(|&c| c == '\t').count();
    let semi = first_line.chars().filter(|&c| c == ';').count();
    let pipe = first_line.chars().filter(|&c| c == '|').count();

    if tab > comma && tab > semi && tab > pipe {
        Delimiter::Tab
    } else if semi > comma && semi > tab && semi > pipe {
        Delimiter::Semicolon
    } else if pipe > comma && pipe > tab && pipe > semi {
        Delimiter::Pipe
    } else {
        Delimiter::Comma
    }
}

/// Parse CSV/TSV input into a 2D matrix of strings (`Vec<Vec<String>>`).
pub fn parse_csv(input: &str, delimiter: Option<Delimiter>) -> Vec<Vec<String>> {
    let clean_input = input.strip_prefix("\u{feff}").unwrap_or(input); // Strip UTF-8 BOM
    let delim_char = delimiter.unwrap_or_else(|| sniff_delimiter(clean_input)).as_char();

    let mut rows = Vec::new();
    let mut cur_row = Vec::new();
    let mut cur_cell = String::new();
    let mut in_quotes = false;
    let chars: Vec<char> = clean_input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        if in_quotes {
            if c == '"' {
                if i + 1 < chars.len() && chars[i + 1] == '"' {
                    cur_cell.push('"');
                    i += 2;
                    continue;
                } else {
                    in_quotes = false;
                }
            } else {
                cur_cell.push(c);
            }
        } else {
            if c == '"' {
                in_quotes = true;
            } else if c == delim_char {
                cur_row.push(std::mem::take(&mut cur_cell));
            } else if c == '\n' {
                cur_row.push(std::mem::take(&mut cur_cell));
                rows.push(std::mem::take(&mut cur_row));
            } else if c == '\r' {
                // Handled on \n, or standalone \r
                if i + 1 < chars.len() && chars[i + 1] == '\n' {
                    i += 1;
                }
                cur_row.push(std::mem::take(&mut cur_cell));
                rows.push(std::mem::take(&mut cur_row));
            } else {
                cur_cell.push(c);
            }
        }
        i += 1;
    }

    if !cur_cell.is_empty() || !cur_row.is_empty() {
        cur_row.push(cur_cell);
        rows.push(cur_row);
    }

    rows
}

/// Serialize a 2D matrix of string cells into RFC 4180 CSV / TSV format.
pub fn to_csv(rows: &[Vec<String>], delimiter: Delimiter) -> String {
    let delim = delimiter.as_char();
    let mut out = String::new();

    for row in rows {
        for (c_idx, cell) in row.iter().enumerate() {
            if c_idx > 0 {
                out.push(delim);
            }
            let needs_quotes = cell.contains(delim)
                || cell.contains('"')
                || cell.contains('\n')
                || cell.contains('\r');
            if needs_quotes {
                out.push('"');
                for c in cell.chars() {
                    if c == '"' {
                        out.push_str("\"\"");
                    } else {
                        out.push(c);
                    }
                }
                out.push('"');
            } else {
                out.push_str(cell);
            }
        }
        out.push_str("\r\n");
    }

    out
}
