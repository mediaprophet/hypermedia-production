//! Lightweight pure-Rust XML reader DOM and writer.
//!
//! Provides deterministic XML generation and parsing for FCP7, FCPX, and caption formats
//! without external parser dependencies.

use std::fmt::Write as _;

pub fn escape_xml(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => o.push_str("&amp;"),
            '<' => o.push_str("&lt;"),
            '>' => o.push_str("&gt;"),
            '"' => o.push_str("&quot;"),
            '\'' => o.push_str("&apos;"),
            c => o.push(c),
        }
    }
    o
}

pub fn unescape_xml(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(i) = rest.find('&') {
        out.push_str(&rest[..i]);
        let after = &rest[i + 1..];
        if let Some(semi) = after.find(';') {
            let entity = &after[..semi];
            match entity {
                "amp" => out.push('&'),
                "lt" => out.push('<'),
                "gt" => out.push('>'),
                "quot" => out.push('"'),
                "apos" => out.push('\''),
                _ if entity.starts_with("#x") => {
                    if let Ok(cp) = u32::from_str_radix(&entity[2..], 16) {
                        if let Some(ch) = char::from_u32(cp) {
                            out.push(ch);
                        }
                    }
                }
                _ if entity.starts_with('#') => {
                    if let Ok(cp) = entity[1..].parse::<u32>() {
                        if let Some(ch) = char::from_u32(cp) {
                            out.push(ch);
                        }
                    }
                }
                _ => {
                    out.push('&');
                    out.push_str(entity);
                    out.push(';');
                }
            }
            rest = &after[semi + 1..];
        } else {
            out.push('&');
            rest = after;
        }
    }
    out.push_str(rest);
    out
}

/// XML formatting writer.
pub struct XmlWriter {
    out: String,
    stack: Vec<String>,
}

impl XmlWriter {
    pub fn new(doctype: Option<&str>) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        if let Some(d) = doctype {
            let _ = writeln!(out, "<!DOCTYPE {d}>");
        }
        Self {
            out,
            stack: Vec::new(),
        }
    }

    fn indent(&mut self) {
        for _ in 0..self.stack.len() {
            self.out.push_str("  ");
        }
    }

    pub fn open(&mut self, name: &str, attrs: &[(&str, &str)]) {
        self.indent();
        self.out.push('<');
        self.out.push_str(name);
        for (k, v) in attrs {
            let _ = write!(self.out, " {k}=\"{}\"", escape_xml(v));
        }
        self.out.push_str(">\n");
        self.stack.push(name.to_string());
    }

    pub fn close(&mut self) {
        let Some(name) = self.stack.pop() else { return };
        self.indent();
        let _ = writeln!(self.out, "</{name}>");
    }

    pub fn empty(&mut self, name: &str, attrs: &[(&str, &str)]) {
        self.indent();
        self.out.push('<');
        self.out.push_str(name);
        for (k, v) in attrs {
            let _ = write!(self.out, " {k}=\"{}\"", escape_xml(v));
        }
        self.out.push_str("/>\n");
    }

    pub fn text(&mut self, name: &str, value: &str) {
        self.indent();
        let _ = writeln!(self.out, "<{name}>{}</{name}>", escape_xml(value));
    }

    pub fn finish(mut self) -> String {
        while !self.stack.is_empty() {
            self.close();
        }
        self.out
    }
}

/// A parsed XML node tree.
#[derive(Clone, Debug, PartialEq)]
pub struct XmlNode {
    pub tag: String,
    pub attrs: Vec<(String, String)>,
    pub children: Vec<XmlNode>,
    pub text: String,
}

impl XmlNode {
    pub fn new(tag: impl Into<String>) -> Self {
        Self {
            tag: tag.into(),
            attrs: Vec::new(),
            children: Vec::new(),
            text: String::new(),
        }
    }

    pub fn attr(&self, key: &str) -> Option<&str> {
        self.attrs
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }

    pub fn child(&self, tag: &str) -> Option<&XmlNode> {
        self.children.iter().find(|c| c.tag == tag)
    }

    pub fn children_named<'a>(&'a self, tag: &'a str) -> impl Iterator<Item = &'a XmlNode> {
        self.children.iter().filter(move |c| c.tag == tag)
    }

    pub fn child_text(&self, tag: &str) -> Option<&str> {
        self.child(tag).map(|c| c.text.trim())
    }

    /// Parse an XML text document into an `XmlNode` tree.
    pub fn parse(xml: &str) -> Result<XmlNode, String> {
        let mut tokens = tokenize(xml)?;
        if tokens.is_empty() {
            return Err("Empty XML input".into());
        }

        // Find root element token
        let root_idx = tokens
            .iter()
            .position(|t| matches!(t, Token::StartTag { .. } | Token::EmptyTag { .. }))
            .ok_or("No root element found")?;

        let mut pos = root_idx;
        parse_node(&tokens, &mut pos)
    }
}

#[derive(Debug)]
enum Token {
    StartTag {
        tag: String,
        attrs: Vec<(String, String)>,
    },
    EndTag {
        tag: String,
    },
    EmptyTag {
        tag: String,
        attrs: Vec<(String, String)>,
    },
    Text(String),
}

fn tokenize(xml: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut rest = xml;

    while !rest.is_empty() {
        if let Some(i) = rest.find('<') {
            if i > 0 {
                let text = unescape_xml(&rest[..i]);
                if !text.trim().is_empty() {
                    tokens.push(Token::Text(text));
                }
            }
            let after = &rest[i + 1..];
            if after.starts_with("!--") {
                // Comment
                let close = after.find("-->").ok_or("Unclosed comment")?;
                rest = &after[close + 3..];
                continue;
            }
            if after.starts_with("?") || after.starts_with("!DOCTYPE") {
                // Processing instruction / doctype
                let close = after.find('>').ok_or("Unclosed PI or DOCTYPE")?;
                rest = &after[close + 1..];
                continue;
            }

            let end_tag = after.starts_with('/');
            let tag_content_start = if end_tag { 1 } else { 0 };
            let close = after.find('>').ok_or("Unclosed tag")?;
            let tag_body = &after[tag_content_start..close].trim();
            let is_empty = tag_body.ends_with('/');
            let tag_body = if is_empty {
                tag_body[..tag_body.len() - 1].trim()
            } else {
                tag_body
            };

            let mut parts = tag_body.split_whitespace();
            let tag_name = parts.next().unwrap_or("").to_string();

            if end_tag {
                tokens.push(Token::EndTag { tag: tag_name });
            } else {
                let mut attrs = Vec::new();
                let attr_str = &tag_body[tag_name.len()..].trim();
                parse_attrs(attr_str, &mut attrs);

                if is_empty {
                    tokens.push(Token::EmptyTag {
                        tag: tag_name,
                        attrs,
                    });
                } else {
                    tokens.push(Token::StartTag {
                        tag: tag_name,
                        attrs,
                    });
                }
            }
            rest = &after[close + 1..];
        } else {
            let text = unescape_xml(rest);
            if !text.trim().is_empty() {
                tokens.push(Token::Text(text));
            }
            break;
        }
    }
    Ok(tokens)
}

fn parse_attrs(mut s: &str, out: &mut Vec<(String, String)>) {
    while let Some(eq) = s.find('=') {
        let key = s[..eq].trim();
        let after = s[eq + 1..].trim();
        if let Some(quote) = after.chars().next() {
            if quote == '"' || quote == '\'' {
                if let Some(end_q) = after[1..].find(quote) {
                    let val = unescape_xml(&after[1..=end_q]);
                    out.push((key.to_string(), val));
                    s = after[end_q + 2..].trim();
                    continue;
                }
            }
        }
        break;
    }
}

fn parse_node(tokens: &[Token], pos: &mut usize) -> Result<XmlNode, String> {
    if *pos >= tokens.len() {
        return Err("Unexpected end of tokens".into());
    }
    match &tokens[*pos] {
        Token::EmptyTag { tag, attrs } => {
            let node = XmlNode {
                tag: tag.clone(),
                attrs: attrs.clone(),
                children: Vec::new(),
                text: String::new(),
            };
            *pos += 1;
            Ok(node)
        }
        Token::StartTag { tag, attrs } => {
            let cur_tag = tag.clone();
            let cur_attrs = attrs.clone();
            *pos += 1;
            let mut children = Vec::new();
            let mut text = String::new();

            while *pos < tokens.len() {
                match &tokens[*pos] {
                    Token::EndTag { tag } => {
                        if tag == &cur_tag {
                            *pos += 1;
                            return Ok(XmlNode {
                                tag: cur_tag,
                                attrs: cur_attrs,
                                children,
                                text,
                            });
                        } else {
                            *pos += 1; // skip mismatch
                        }
                    }
                    Token::Text(t) => {
                        if !text.is_empty() {
                            text.push(' ');
                        }
                        text.push_str(t);
                        *pos += 1;
                    }
                    Token::StartTag { .. } | Token::EmptyTag { .. } => {
                        let child = parse_node(tokens, pos)?;
                        children.push(child);
                    }
                }
            }
            Ok(XmlNode {
                tag: cur_tag,
                attrs: cur_attrs,
                children,
                text,
            })
        }
        Token::Text(_) | Token::EndTag { .. } => {
            *pos += 1;
            Err("Unexpected token type when expecting start element".into())
        }
    }
}
