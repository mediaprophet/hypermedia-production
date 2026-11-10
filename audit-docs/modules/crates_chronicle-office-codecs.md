# `crates/chronicle-office-codecs`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Document interchange codecs (Markdown, HTML, RTF, LaTeX)._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `to_html` | 1 | `crates/chronicle-office-codecs/src/document/html.rs:10` | semantics | Serialize a `FlowDoc` to clean, semantic HTML5. |
| `write_html_inlines` | 2 | `crates/chronicle-office-codecs/src/document/html.rs:89` | - |  |
| `parse_html` | 1 | `crates/chronicle-office-codecs/src/document/html.rs:161` | - | Parse HTML string into a `FlowDoc`. |
| `find_tag_recursive` | 2 | `crates/chronicle-office-codecs/src/document/html.rs:179` | - |  |
| `extract_flow_blocks` | 2 | `crates/chronicle-office-codecs/src/document/html.rs:193` | - |  |
| `extract_inlines_from_html` | 3 | `crates/chronicle-office-codecs/src/document/html.rs:258` | - |  |
| `to_latex` | 1 | `crates/chronicle-office-codecs/src/document/latex.rs:9` | - | Export a `FlowDoc` to standard LaTeX source. |
| `write_latex_inlines` | 2 | `crates/chronicle-office-codecs/src/document/latex.rs:105` | - |  |
| `escape_latex` | 1 | `crates/chronicle-office-codecs/src/document/latex.rs:156` | - |  |
| `parse_latex` | 1 | `crates/chronicle-office-codecs/src/document/latex.rs:177` | - | Simple parser for LaTeX `.tex` documents into `FlowDoc`. |
| `parse_markdown` | 1 | `crates/chronicle-office-codecs/src/document/markdown.rs:10` | - | Parse a Markdown string into a `FlowDoc`. |
| `parse_inlines` | 1 | `crates/chronicle-office-codecs/src/document/markdown.rs:178` | - | Parse inline formatting: bold, italic, code, link, inline math. |
| `to_markdown` | 1 | `crates/chronicle-office-codecs/src/document/markdown.rs:297` | - | Serialize a `FlowDoc` to CommonMark / GFM Markdown. |
| `cell_text` | 1 | `crates/chronicle-office-codecs/src/document/markdown.rs:387` | - |  |
| `write_inlines` | 2 | `crates/chronicle-office-codecs/src/document/markdown.rs:400` | - |  |
| `to_rtf` | 1 | `crates/chronicle-office-codecs/src/document/rtf.rs:8` | - | Serialize a `FlowDoc` to standard RTF 1.5. |
| `write_rtf_inlines` | 2 | `crates/chronicle-office-codecs/src/document/rtf.rs:59` | - |  |
| `escape_rtf` | 2 | `crates/chronicle-office-codecs/src/document/rtf.rs:110` | - |  |
| `parse_rtf` | 1 | `crates/chronicle-office-codecs/src/document/rtf.rs:129` | - | Simple parser for RTF 1.5 files into `FlowDoc`. |
| `impl FlowInline::plain_text` | 1 | `crates/chronicle-office-codecs/src/flow/model.rs:28` | - |  |
| `impl FlowInline::text_content` | 1 | `crates/chronicle-office-codecs/src/flow/model.rs:32` | - |  |
| `impl FlowPara::new` | 1 | `crates/chronicle-office-codecs/src/flow/model.rs:68` | - |  |
| `impl FlowPara::with_text` | 2 | `crates/chronicle-office-codecs/src/flow/model.rs:77` | - |  |
| `impl FlowPara::plain_text` | 1 | `crates/chronicle-office-codecs/src/flow/model.rs:83` | - |  |
| `impl FlowCell::with_text` | 1 | `crates/chronicle-office-codecs/src/flow/model.rs:96` | - |  |
| `impl FlowDoc::new` | 0 | `crates/chronicle-office-codecs/src/flow/model.rs:130` | - |  |
| `impl FlowDoc::push_para` | 2 | `crates/chronicle-office-codecs/src/flow/model.rs:134` | - |  |
| `impl FlowDoc::push_table` | 2 | `crates/chronicle-office-codecs/src/flow/model.rs:138` | - |  |
| `impl NQuin48::new` | 5 | `crates/chronicle-office-codecs/src/ontology.rs:17` | - |  |
| `impl NQuin48::is_valid` | 1 | `crates/chronicle-office-codecs/src/ontology.rs:29` | - |  |
| `to_pptx_package` | 1 | `crates/chronicle-office-codecs/src/presentation/pptx.rs:10` | - | Serialize a `Presentation` into an OpenXML `.pptx` ZIP package. |
| `escape_attr` | 1 | `crates/chronicle-office-codecs/src/presentation/pptx.rs:86` | - |  |
| `emit_txbody_paragraphs` | 2 | `crates/chronicle-office-codecs/src/presentation/pptx.rs:92` | - |  |
| `parse_pptx_package` | 1 | `crates/chronicle-office-codecs/src/presentation/pptx.rs:116` | - | Parse an OpenXML `.pptx` ZIP package into a `Presentation`. |
| `impl Delimiter::as_char` | 1 | `crates/chronicle-office-codecs/src/spreadsheet/csv.rs:12` | - |  |
| `sniff_delimiter` | 1 | `crates/chronicle-office-codecs/src/spreadsheet/csv.rs:23` | - | Sniff the most likely delimiter based on counts in the first line. |
| `parse_csv` | 2 | `crates/chronicle-office-codecs/src/spreadsheet/csv.rs:42` | numerics | Parse CSV/TSV input into a 2D matrix of strings (`Vec<Vec<String>>`). |
| `to_csv` | 2 | `crates/chronicle-office-codecs/src/spreadsheet/csv.rs:98` | numerics | Serialize a 2D matrix of string cells into RFC 4180 CSV / TSV format. |
| `impl XlsxWorkbook::new` | 0 | `crates/chronicle-office-codecs/src/spreadsheet/xlsx.rs:21` | - |  |
| `impl XlsxWorkbook::with_single_sheet` | 2 | `crates/chronicle-office-codecs/src/spreadsheet/xlsx.rs:25` | - |  |
| `to_xlsx_package` | 1 | `crates/chronicle-office-codecs/src/spreadsheet/xlsx.rs:36` | - | Serialize an `XlsxWorkbook` into an OpenXML `.xlsx` ZIP package. |
| `parse_xlsx_package` | 1 | `crates/chronicle-office-codecs/src/spreadsheet/xlsx.rs:132` | - | Parse an OpenXML `.xlsx` ZIP package into an `XlsxWorkbook`. |
| `parse_a1_col` | 1 | `crates/chronicle-office-codecs/src/spreadsheet/xlsx.rs:207` | - |  |
| `to_docx_package` | 1 | `crates/chronicle-office-codecs/src/wordprocessing/docx.rs:10` | - | Serialize a `FlowDoc` into an OpenXML `.docx` ZIP package. |
| `parse_docx_package` | 1 | `crates/chronicle-office-codecs/src/wordprocessing/docx.rs:115` | - | Parse an OpenXML `.docx` ZIP package into a `FlowDoc`. |
| `local_name` | 1 | `crates/chronicle-office-codecs/src/xml/dom.rs:18` | - |  |
| `impl XEl::new` | 1 | `crates/chronicle-office-codecs/src/xml/dom.rs:23` | - |  |
| `impl XEl::with_attr` | 3 | `crates/chronicle-office-codecs/src/xml/dom.rs:31` | - |  |
| `impl XEl::with_child` | 2 | `crates/chronicle-office-codecs/src/xml/dom.rs:36` | - |  |
| `impl XEl::with_text` | 2 | `crates/chronicle-office-codecs/src/xml/dom.rs:41` | - |  |
| `impl XEl::local` | 1 | `crates/chronicle-office-codecs/src/xml/dom.rs:46` | - |  |
| `impl XEl::attr` | 2 | `crates/chronicle-office-codecs/src/xml/dom.rs:50` | - |  |
| `impl XEl::find_child` | 2 | `crates/chronicle-office-codecs/src/xml/dom.rs:67` | - |  |
| `impl XEl::find_children` | 2 | `crates/chronicle-office-codecs/src/xml/dom.rs:78` | - |  |
| `impl XEl::text_content` | 1 | `crates/chronicle-office-codecs/src/xml/dom.rs:85` | - |  |
| `impl XEl::collect_text` | 2 | `crates/chronicle-office-codecs/src/xml/dom.rs:91` | - |  |
| `impl XEl::write_xml` | 2 | `crates/chronicle-office-codecs/src/xml/dom.rs:100` | - |  |
| `impl XEl::to_xml_string` | 1 | `crates/chronicle-office-codecs/src/xml/dom.rs:127` | - |  |
| `escape_xml` | 2 | `crates/chronicle-office-codecs/src/xml/dom.rs:134` | - |  |
| `unescape_xml` | 1 | `crates/chronicle-office-codecs/src/xml/dom.rs:147` | - |  |
| `impl fmt::Display for XmlParseError::fmt` | 2 | `crates/chronicle-office-codecs/src/xml/dom.rs:190` | - |  |
| `parse_xml` | 1 | `crates/chronicle-office-codecs/src/xml/dom.rs:204` | - | Parse XML string into an element tree. |
| `skip_whitespace` | 2 | `crates/chronicle-office-codecs/src/xml/dom.rs:250` | - |  |
| `parse_element` | 2 | `crates/chronicle-office-codecs/src/xml/dom.rs:260` | - |  |
| `impl fmt::Display for ZipError::fmt` | 2 | `crates/chronicle-office-codecs/src/zip/minizip.rs:18` | - |  |
| `crc32` | 1 | `crates/chronicle-office-codecs/src/zip/minizip.rs:32` | - | Standard CRC-32 (IEEE 802.3). |
| `impl ZipPackage::new` | 0 | `crates/chronicle-office-codecs/src/zip/minizip.rs:55` | - |  |
| `impl ZipPackage::insert` | 3 | `crates/chronicle-office-codecs/src/zip/minizip.rs:61` | - |  |
| `impl ZipPackage::get` | 2 | `crates/chronicle-office-codecs/src/zip/minizip.rs:66` | - |  |
| `impl ZipPackage::get_str` | 2 | `crates/chronicle-office-codecs/src/zip/minizip.rs:71` | - |  |
| `impl ZipPackage::write_to_bytes` | 1 | `crates/chronicle-office-codecs/src/zip/minizip.rs:76` | - | Serialize all contained files into a standard ZIP archive byte vector. |
| `impl ZipPackage::parse` | 1 | `crates/chronicle-office-codecs/src/zip/minizip.rs:140` | - | Parse a ZIP byte slice and extract uncompressed (stored) entries. |
| `test_zip_archive_roundtrip` | 0 | `crates/chronicle-office-codecs/tests/codecs_tests.rs:4` | - |  |
| `test_xml_dom_parser_and_writer` | 0 | `crates/chronicle-office-codecs/tests/codecs_tests.rs:18` | - |  |
| `test_markdown_codecs` | 0 | `crates/chronicle-office-codecs/tests/codecs_tests.rs:35` | - |  |
| `test_html_codecs` | 0 | `crates/chronicle-office-codecs/tests/codecs_tests.rs:63` | - |  |
| `test_rtf_codecs` | 0 | `crates/chronicle-office-codecs/tests/codecs_tests.rs:75` | - |  |
| `test_latex_codecs` | 0 | `crates/chronicle-office-codecs/tests/codecs_tests.rs:94` | - |  |
| `test_csv_tsv_codecs` | 0 | `crates/chronicle-office-codecs/tests/codecs_tests.rs:113` | - |  |
| `test_openxml_xlsx_roundtrip` | 0 | `crates/chronicle-office-codecs/tests/codecs_tests.rs:129` | - |  |
| `test_openxml_docx_roundtrip` | 0 | `crates/chronicle-office-codecs/tests/codecs_tests.rs:152` | - |  |
| `test_openxml_pptx_roundtrip` | 0 | `crates/chronicle-office-codecs/tests/codecs_tests.rs:173` | - |  |
| `test_ontology_contract_parity` | 0 | `crates/chronicle-office-codecs/tests/codecs_tests.rs:200` | semantics |  |
