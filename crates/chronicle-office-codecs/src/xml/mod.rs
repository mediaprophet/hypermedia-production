//! XML processing module.

pub mod dom;

pub use dom::{escape_xml, parse_xml, unescape_xml, XEl, XNode, XmlParseError};
