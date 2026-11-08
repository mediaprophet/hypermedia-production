//! Office document, spreadsheet, and presentation codecs for open hypermedia interchange.
//! Grounded in UDHR Articles 19 & 27 and Informatics Fiduciary standards.

#![forbid(unsafe_code)]

pub mod document;
pub mod flow;
pub mod ontology;
pub mod presentation;
pub mod spreadsheet;
pub mod wordprocessing;
pub mod xml;
pub mod zip;

pub use document::{
    escape_latex, parse_html, parse_inlines, parse_latex, parse_markdown, parse_rtf, to_html,
    to_latex, to_markdown, to_rtf,
};
pub use flow::{
    FlowBlock, FlowCell, FlowDoc, FlowFmt, FlowInline, FlowKind, FlowListInfo, FlowPara, FlowRow,
    FlowTable,
};
pub use presentation::{parse_pptx_package, to_pptx_package};
pub use spreadsheet::{
    parse_csv, parse_xlsx_package, sniff_delimiter, to_csv, to_xlsx_package, Delimiter,
    XlsxSheet, XlsxWorkbook,
};
pub use wordprocessing::{parse_docx_package, to_docx_package};
pub use xml::{escape_xml, parse_xml, unescape_xml, XEl, XNode, XmlParseError};
pub use zip::{crc32, ZipError, ZipPackage};
