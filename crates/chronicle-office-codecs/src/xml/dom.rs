//! Lightweight, pure-Rust, bounded XML parser and serializer for OpenXML formats.

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum XNode {
    Element(XEl),
    Text(String),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct XEl {
    pub name: String,
    pub attrs: Vec<(String, String)>,
    pub children: Vec<XNode>,
}

fn local_name(name: &str) -> &str {
    name.rsplit_once(':').map(|(_, l)| l).unwrap_or(name)
}

impl XEl {
    pub fn new(name: impl Into<String>) -> Self {
        XEl {
            name: name.into(),
            attrs: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn with_attr(mut self, key: impl Into<String>, val: impl Into<String>) -> Self {
        self.attrs.push((key.into(), val.into()));
        self
    }

    pub fn with_child(mut self, child: XEl) -> Self {
        self.children.push(XNode::Element(child));
        self
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.children.push(XNode::Text(text.into()));
        self
    }

    pub fn local(&self) -> &str {
        local_name(&self.name)
    }

    pub fn attr(&self, key: &str) -> Option<&str> {
        let want_local = local_name(key);
        // Try exact match first
        for (k, v) in &self.attrs {
            if k == key {
                return Some(v.as_str());
            }
        }
        // Then try local name match
        for (k, v) in &self.attrs {
            if local_name(k) == want_local {
                return Some(v.as_str());
            }
        }
        None
    }

    pub fn find_child(&self, local: &str) -> Option<&XEl> {
        for node in &self.children {
            if let XNode::Element(el) = node {
                if el.local() == local {
                    return Some(el);
                }
            }
        }
        None
    }

    pub fn find_children<'a>(&'a self, local: &'a str) -> impl Iterator<Item = &'a XEl> {
        self.children.iter().filter_map(move |n| match n {
            XNode::Element(el) if el.local() == local => Some(el),
            _ => None,
        })
    }

    pub fn text_content(&self) -> String {
        let mut out = String::new();
        self.collect_text(&mut out);
        out
    }

    fn collect_text(&self, out: &mut String) {
        for node in &self.children {
            match node {
                XNode::Text(t) => out.push_str(t),
                XNode::Element(el) => el.collect_text(out),
            }
        }
    }

    pub fn write_xml(&self, out: &mut String) {
        out.push('<');
        out.push_str(&self.name);
        for (k, v) in &self.attrs {
            out.push(' ');
            out.push_str(k);
            out.push_str("=\"");
            escape_xml(v, out);
            out.push('"');
        }

        if self.children.is_empty() {
            out.push_str("/>");
        } else {
            out.push('>');
            for child in &self.children {
                match child {
                    XNode::Element(el) => el.write_xml(out),
                    XNode::Text(t) => escape_xml(t, out),
                }
            }
            out.push_str("</");
            out.push_str(&self.name);
            out.push('>');
        }
    }

    pub fn to_xml_string(&self) -> String {
        let mut s = String::new();
        self.write_xml(&mut s);
        s
    }
}

pub fn escape_xml(s: &str, out: &mut String) {
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
}

pub fn unescape_xml(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '&' {
            let mut ent = String::new();
            while let Some(&next_c) = chars.peek() {
                if next_c == ';' {
                    chars.next();
                    break;
                }
                ent.push(chars.next().unwrap());
                if ent.len() > 10 {
                    break;
                }
            }
            match ent.as_str() {
                "amp" => out.push('&'),
                "lt" => out.push('<'),
                "gt" => out.push('>'),
                "quot" => out.push('"'),
                "apos" => out.push('\''),
                _ => {
                    out.push('&');
                    out.push_str(&ent);
                    out.push(';');
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

#[derive(Debug)]
pub enum XmlParseError {
    UnexpectedEof,
    MalformedTag,
    MismatchedTag { expected: String, found: String },
}

impl fmt::Display for XmlParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XmlParseError::UnexpectedEof => write!(f, "unexpected end of XML document"),
            XmlParseError::MalformedTag => write!(f, "malformed XML tag"),
            XmlParseError::MismatchedTag { expected, found } => {
                write!(f, "mismatched XML closing tag: expected {}, found {}", expected, found)
            }
        }
    }
}

impl std::error::Error for XmlParseError {}

/// Parse XML string into an element tree.
pub fn parse_xml(input: &str) -> Result<XEl, XmlParseError> {
    let mut chars = input.char_indices().peekable();

    // Skip XML declaration <?xml ... ?> and doctype <!DOCTYPE ... >
    loop {
        skip_whitespace(&mut chars, input);
        if let Some(&(_, '<')) = chars.peek() {
            let mut probe = chars.clone();
            probe.next();
            if let Some(&(_, '?')) = probe.peek() {
                // Skip processing instruction <? ... ?>
                chars.next(); // '<'
                chars.next(); // '?'
                while let Some((_, c)) = chars.next() {
                    if c == '?' {
                        if let Some(&(_, '>')) = chars.peek() {
                            chars.next();
                            break;
                        }
                    }
                }
                continue;
            } else if let Some(&(_, '!')) = probe.peek() {
                // Skip comment <!-- ... --> or doctype <!DOCTYPE ... >
                chars.next(); // '<'
                chars.next(); // '!'
                let mut depth = 1;
                while let Some((_, c)) = chars.next() {
                    if c == '<' {
                        depth += 1;
                    } else if c == '>' {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                }
                continue;
            }
        }
        break;
    }

    parse_element(&mut chars, input)
}

fn skip_whitespace(chars: &mut std::iter::Peekable<std::str::CharIndices<'_>>, _input: &str) {
    while let Some(&(_, c)) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }
}

fn parse_element(
    chars: &mut std::iter::Peekable<std::str::CharIndices<'_>>,
    input: &str,
) -> Result<XEl, XmlParseError> {
    skip_whitespace(chars, input);

    if chars.next().map(|(_, c)| c) != Some('<') {
        return Err(XmlParseError::MalformedTag);
    }

    // Read tag name
    let mut tag_name = String::new();
    while let Some(&(_, c)) = chars.peek() {
        if c.is_whitespace() || c == '/' || c == '>' {
            break;
        }
        tag_name.push(chars.next().unwrap().1);
    }

    if tag_name.is_empty() {
        return Err(XmlParseError::MalformedTag);
    }

    // Read attributes
    let mut attrs = Vec::new();
    loop {
        skip_whitespace(chars, input);
        match chars.peek() {
            Some(&(_, '/')) => {
                chars.next();
                if chars.next().map(|(_, c)| c) == Some('>') {
                    // Self-closing tag
                    return Ok(XEl {
                        name: tag_name,
                        attrs,
                        children: Vec::new(),
                    });
                } else {
                    return Err(XmlParseError::MalformedTag);
                }
            }
            Some(&(_, '>')) => {
                chars.next();
                break;
            }
            Some(&(_, _)) => {
                // Parse attribute key="val"
                let mut key = String::new();
                while let Some(&(_, c)) = chars.peek() {
                    if c == '=' || c.is_whitespace() || c == '/' || c == '>' {
                        break;
                    }
                    key.push(chars.next().unwrap().1);
                }
                skip_whitespace(chars, input);
                if chars.peek().map(|&(_, c)| c) == Some('=') {
                    chars.next();
                    skip_whitespace(chars, input);
                    let quote = chars.next().map(|(_, c)| c).ok_or(XmlParseError::UnexpectedEof)?;
                    if quote == '"' || quote == '\'' {
                        let mut val = String::new();
                        while let Some((_, c)) = chars.next() {
                            if c == quote {
                                break;
                            }
                            val.push(c);
                        }
                        attrs.push((key, unescape_xml(&val)));
                    } else {
                        return Err(XmlParseError::MalformedTag);
                    }
                } else if !key.is_empty() {
                    attrs.push((key, String::new()));
                }
            }
            None => return Err(XmlParseError::UnexpectedEof),
        }
    }

    // Parse children & text until </tag_name>
    let mut children = Vec::new();
    let mut text_buf = String::new();

    loop {
        if chars.peek().is_none() {
            return Err(XmlParseError::UnexpectedEof);
        }

        if let Some(&(_, '<')) = chars.peek() {
            let mut probe = chars.clone();
            probe.next();
            if let Some(&(_, '/')) = probe.peek() {
                // Closing tag
                chars.next(); // '<'
                chars.next(); // '/'
                let mut close_tag = String::new();
                while let Some((_, c)) = chars.next() {
                    if c == '>' {
                        break;
                    }
                    if !c.is_whitespace() {
                        close_tag.push(c);
                    }
                }
                if !text_buf.is_empty() {
                    children.push(XNode::Text(unescape_xml(&text_buf)));
                }
                if close_tag == tag_name {
                    return Ok(XEl {
                        name: tag_name,
                        attrs,
                        children,
                    });
                } else {
                    return Err(XmlParseError::MismatchedTag {
                        expected: tag_name,
                        found: close_tag,
                    });
                }
            } else if let Some(&(_, '?')) = probe.peek() {
                // Skip processing instruction inside body
                chars.next();
                chars.next();
                while let Some((_, c)) = chars.next() {
                    if c == '?' && chars.peek().map(|&(_, next_c)| next_c) == Some('>') {
                        chars.next();
                        break;
                    }
                }
                continue;
            } else if let Some(&(_, '!')) = probe.peek() {
                // Skip comments/cdata
                chars.next();
                chars.next();
                while let Some((_, c)) = chars.next() {
                    if c == '>' {
                        break;
                    }
                }
                continue;
            } else {
                // Child element
                if !text_buf.is_empty() {
                    children.push(XNode::Text(unescape_xml(&text_buf)));
                    text_buf.clear();
                }
                let child = parse_element(chars, input)?;
                children.push(XNode::Element(child));
            }
        } else {
            text_buf.push(chars.next().unwrap().1);
        }
    }
}
