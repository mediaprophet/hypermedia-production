//! ISO 10303-21 STEP physical file parser.
//!
//! Parses IFC and STEP physical files containing schema headers and numbered entity instances:
//! `#12=IFCWALL('2O2Fr$t4X7Zf8NOew3FLOH',#5,'Wall',$,…);`.

use std::collections::HashMap;

/// A STEP parameter value.
#[derive(Clone, Debug, PartialEq)]
pub enum V {
    /// `$` (unset) or `*` (derived).
    Null,
    Int(i64),
    Real(f64),
    Str(String),
    /// `.T.`, `.ELEMENT.`: enum symbol without dots.
    Enum(String),
    /// `#12` reference to entity instance.
    Ref(u64),
    List(Vec<V>),
    /// `IFCLABEL('Wall')`: typed value.
    Typed(String, Box<V>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Entity {
    pub name: String,
    pub args: Vec<V>,
}

pub struct File {
    pub schema: String,
    pub entities: HashMap<u64, Entity>,
}

/// Lists nest no deeper than this.
const DEPTH_LIMIT: usize = 32;

pub fn parse(text: &[u8]) -> Result<File, String> {
    let header = find(text, b"HEADER;").ok_or("not a STEP file: it has no HEADER section")?;
    let mut p = Parser {
        s: text,
        i: header + 7,
    };
    let mut schema = String::new();
    loop {
        p.skip();
        if p.word(b"ENDSEC") {
            p.skip();
            p.expect(b';')?;
            break;
        }
        let name = p.keyword()?;
        let args = p.list(0)?;
        p.skip();
        p.expect(b';')?;
        if name == "FILE_SCHEMA" {
            schema = first_string(&args)
                .unwrap_or_default()
                .trim()
                .to_ascii_uppercase();
        }
    }
    p.skip();
    if !p.word(b"DATA") {
        return Err(p.error("expected DATA section"));
    }
    p.skip();
    if p.peek() == Some(b'(') {
        p.list(0)?;
        p.skip();
    }
    p.expect(b';')?;
    let mut entities = HashMap::new();
    loop {
        p.skip();
        if p.word(b"ENDSEC") {
            break;
        }
        p.expect(b'#')?;
        let id = p.uint()?;
        p.skip();
        p.expect(b'=')?;
        p.skip();
        let entity = if p.peek() == Some(b'(') {
            p.complex()?;
            None
        } else {
            let name = p.keyword()?;
            Some(Entity {
                name,
                args: p.list(0)?,
            })
        };
        p.skip();
        p.expect(b';')?;
        if let Some(e) = entity {
            entities.insert(id, e);
        }
    }
    Ok(File { schema, entities })
}

fn find(hay: &[u8], needle: &[u8]) -> Option<usize> {
    hay.windows(needle.len()).position(|w| w == needle)
}

fn first_string(args: &[V]) -> Option<String> {
    args.iter().find_map(|v| match v {
        V::Str(s) => Some(s.clone()),
        V::List(l) => first_string(l),
        _ => None,
    })
}

struct Parser<'a> {
    s: &'a [u8],
    i: usize,
}

impl Parser<'_> {
    fn peek(&self) -> Option<u8> {
        self.s.get(self.i).copied()
    }

    fn rest(&self) -> &[u8] {
        self.s.get(self.i..).unwrap_or_default()
    }

    fn skip(&mut self) {
        loop {
            match self.peek() {
                Some(b' ' | b'\t' | b'\r' | b'\n') => self.i += 1,
                Some(b'/') if self.s.get(self.i + 1) == Some(&b'*') => {
                    self.i = find(self.rest(), b"*/").map_or(self.s.len(), |k| self.i + k + 2);
                }
                _ => return,
            }
        }
    }

    fn error(&self, what: &str) -> String {
        let line = self
            .s
            .get(..self.i)
            .unwrap_or(self.s)
            .iter()
            .filter(|&&b| b == b'\n')
            .count()
            + 1;
        format!("line {line}: {what}")
    }

    fn expect(&mut self, b: u8) -> Result<(), String> {
        if self.peek() == Some(b) {
            self.i += 1;
            Ok(())
        } else {
            Err(self.error(&format!("expected `{}`", char::from(b))))
        }
    }

    fn word(&mut self, w: &[u8]) -> bool {
        let after = self
            .s
            .get(self.i + w.len())
            .is_some_and(|c| c.is_ascii_alphanumeric() || *c == b'_');
        if self.rest().starts_with(w) && !after {
            self.i += w.len();
            true
        } else {
            false
        }
    }

    fn keyword(&mut self) -> Result<String, String> {
        let start = self.i;
        while self
            .peek()
            .is_some_and(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b'!')
        {
            self.i += 1;
        }
        match self.s.get(start..self.i) {
            Some(k) if !k.is_empty() => Ok(String::from_utf8_lossy(k).to_ascii_uppercase()),
            _ => Err(self.error("expected a name")),
        }
    }

    fn uint(&mut self) -> Result<u64, String> {
        let start = self.i;
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.i += 1;
        }
        let digits =
            std::str::from_utf8(self.s.get(start..self.i).unwrap_or_default()).unwrap_or_default();
        digits
            .parse()
            .map_err(|_| self.error(&format!("`{digits}` is not an entity number")))
    }

    fn number(&mut self) -> Result<V, String> {
        let start = self.i;
        while self
            .peek()
            .is_some_and(|c| c.is_ascii_digit() || matches!(c, b'.' | b'E' | b'e' | b'+' | b'-'))
        {
            self.i += 1;
        }
        let t =
            std::str::from_utf8(self.s.get(start..self.i).unwrap_or_default()).unwrap_or_default();
        let v = if t.contains(['.', 'E', 'e']) {
            t.parse().map(V::Real).ok()
        } else {
            t.parse()
                .map(V::Int)
                .ok()
                .or_else(|| t.parse().map(V::Real).ok())
        };
        v.ok_or_else(|| self.error(&format!("`{t}` is not a number")))
    }

    fn string(&mut self) -> Result<V, String> {
        self.i += 1;
        let mut raw = Vec::new();
        loop {
            match self.peek() {
                None => return Err(self.error("unterminated string")),
                Some(b'\'') if self.s.get(self.i + 1) == Some(&b'\'') => {
                    raw.push(b'\'');
                    self.i += 2;
                }
                Some(b'\'') => {
                    self.i += 1;
                    return Ok(V::Str(decode(&String::from_utf8_lossy(&raw))));
                }
                Some(c) => {
                    raw.push(c);
                    self.i += 1;
                }
            }
        }
    }

    fn until(&mut self, end: u8) -> Result<String, String> {
        let k = self
            .rest()
            .iter()
            .position(|&c| c == end)
            .ok_or_else(|| self.error(&format!("no closing `{}`", char::from(end))))?;
        let t = String::from_utf8_lossy(self.rest().get(..k).unwrap_or_default()).into_owned();
        self.i += k + 1;
        Ok(t)
    }

    fn value(&mut self, depth: usize) -> Result<V, String> {
        self.skip();
        match self.peek() {
            Some(b'$' | b'*') => {
                self.i += 1;
                Ok(V::Null)
            }
            Some(b'\'') => self.string(),
            Some(b'"') => {
                self.i += 1;
                self.until(b'"').map(V::Str)
            }
            Some(b'#') => {
                self.i += 1;
                self.uint().map(V::Ref)
            }
            Some(b'.') => {
                self.i += 1;
                self.until(b'.')
                    .map(|e| V::Enum(e.trim().to_ascii_uppercase()))
            }
            Some(b'(') => self.list(depth + 1).map(V::List),
            Some(c) if c.is_ascii_digit() || c == b'-' || c == b'+' => self.number(),
            Some(c) if c.is_ascii_alphabetic() || c == b'_' || c == b'!' => {
                let name = self.keyword()?;
                let mut args = self.list(depth + 1)?;
                let v = if args.len() == 1 {
                    args.pop().unwrap_or(V::Null)
                } else {
                    V::List(args)
                };
                Ok(V::Typed(name, Box::new(v)))
            }
            _ => Err(self.error("unexpected character")),
        }
    }

    fn complex(&mut self) -> Result<(), String> {
        self.expect(b'(')?;
        loop {
            self.skip();
            if self.peek() == Some(b')') {
                self.i += 1;
                return Ok(());
            }
            self.keyword()?;
            self.list(1)?;
        }
    }

    fn list(&mut self, depth: usize) -> Result<Vec<V>, String> {
        if depth > DEPTH_LIMIT {
            return Err(self.error("lists nested too deep"));
        }
        self.skip();
        self.expect(b'(')?;
        let mut out = Vec::new();
        self.skip();
        if self.peek() == Some(b')') {
            self.i += 1;
            return Ok(out);
        }
        loop {
            out.push(self.value(depth)?);
            self.skip();
            match self.peek() {
                Some(b',') => self.i += 1,
                Some(b')') => {
                    self.i += 1;
                    return Ok(out);
                }
                _ => return Err(self.error("expected `,` or `)`")),
            }
        }
    }
}

pub fn decode(s: &str) -> String {
    if !s.contains('\\') {
        return s.to_string();
    }
    let hex = |h: &str| u32::from_str_radix(h, 16).ok();
    let mut out = String::new();
    let mut rest = s;
    while let Some(k) = rest.find('\\') {
        out.push_str(rest.get(..k).unwrap_or_default());
        rest = rest.get(k..).unwrap_or_default();
        if let Some(r) = rest.strip_prefix(r"\\") {
            out.push('\\');
            rest = r;
        } else if let Some((r, wide)) = rest
            .strip_prefix(r"\X2\")
            .map(|r| (r, false))
            .or_else(|| rest.strip_prefix(r"\X4\").map(|r| (r, true)))
        {
            let end = r.find(r"\X0\").unwrap_or(r.len());
            let digits: Vec<char> = r.get(..end).unwrap_or_default().chars().collect();
            let width = if wide { 8 } else { 4 };
            let units = digits
                .chunks_exact(width)
                .filter_map(|c| hex(&c.iter().collect::<String>()));
            if wide {
                out.extend(
                    units.map(|u| char::from_u32(u).unwrap_or(char::REPLACEMENT_CHARACTER)),
                );
            } else {
                let units: Vec<u16> = units.filter_map(|u| u16::try_from(u).ok()).collect();
                out.extend(
                    char::decode_utf16(units)
                        .map(|c| c.unwrap_or(char::REPLACEMENT_CHARACTER)),
                );
            }
            rest = r.get(end + 4..).unwrap_or_default();
        } else if let Some(r) = rest.strip_prefix(r"\X\") {
            if let Some(byte) = r.get(..2).and_then(hex) {
                out.extend(char::from_u32(byte));
                rest = r.get(2..).unwrap_or_default();
            } else {
                out.push('\\');
                rest = rest.get(1..).unwrap_or_default();
            }
        } else if let Some(r) = rest.strip_prefix(r"\S\") {
            if let Some(c) = r.chars().next() {
                out.extend(char::from_u32(u32::from(c) + 128));
                rest = r.get(c.len_utf8()..).unwrap_or_default();
            } else {
                out.push('\\');
                rest = rest.get(1..).unwrap_or_default();
            }
        } else if rest.starts_with(r"\P") && rest.get(3..4) == Some(r"\") {
            rest = rest.get(4..).unwrap_or_default();
        } else {
            out.push('\\');
            rest = rest.get(1..).unwrap_or_default();
        }
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_step_file() {
        let f = parse(
            b"ISO-10303-21;\nHEADER;\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\n\
              #1=IFCWALL('2O2Fr$t4X7Zf8NOew3FLOH',$,'Wall',$,$,#2,#3,'101');\n\
              #2=IFCPERSON('USR01','Jane','Doe');\n\
              ENDSEC;\nEND-ISO-10303-21;\n",
        )
        .unwrap();
        assert_eq!(f.schema, "IFC4");
        assert_eq!(f.entities.len(), 2);
        let wall = &f.entities[&1];
        assert_eq!(wall.name, "IFCWALL");
        assert_eq!(wall.args[0], V::Str("2O2Fr$t4X7Zf8NOew3FLOH".into()));
        assert_eq!(wall.args[1], V::Null);
    }

    #[test]
    fn step_decode_escapes() {
        assert_eq!(decode(r"\X2\00D6\X0\ffice"), "Öffice");
        assert_eq!(decode(r"caf\X\E9"), "café");
        assert_eq!(decode(r"a\\b"), r"a\b");
    }
}
