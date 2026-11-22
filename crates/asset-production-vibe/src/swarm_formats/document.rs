//! Tagged Unicode PDF interchange with retained fonts and structured reading order.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

use discourse_academic_tex::{
    PdfDoc, PdfPage, Ttf,
    texpdf::{DocumentTags, TextBlock},
};
use parley::{
    Alignment, AlignmentOptions, FontContext, FontFamily, Layout, LayoutContext,
    PositionedLayoutItem, StyleProperty,
};
use serde_json::json;

use super::{encode_json, receipt, result, sha256};
use crate::{
    Args, AssetHost, Value, fields, list, number,
    qualia_containers_governance_poet_ops::encoded_resource, record, required, text,
};

const MAX_DOCUMENT_BYTES: usize = 16 * 1024 * 1024;
const MAX_FONT_BYTES: usize = 8 * 1024 * 1024;
const MAX_FONT_DICTIONARY_BYTES: usize = 1024 * 1024;
const MAX_TEXT_BYTES: usize = 48 * 1024;
const MAX_BLOCKS: usize = 512;
const MAX_GLYPHS: usize = 30_000;
const MAX_PAGES: usize = 256;

struct Grid {
    width: f64,
    height: f64,
    margin: f64,
    size: f64,
    spacing: f64,
}

impl AssetHost<'_> {
    pub(crate) fn paginate_foliation_and_pdf_ua(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["document", "font", "page"])?;
        let document = record(required(args, "document")?)?;
        fields(document, &["title", "language", "blocks"])?;
        let title = text(required(document, "title")?)?;
        let language = text(required(document, "language")?)?;
        if title.trim().is_empty()
            || title.len() > 512
            || title.chars().any(char::is_control)
            || !valid_language(language)
        {
            return Err("Expected a title and a bounded BCP-47-style language tag".into());
        }
        let blocks = list(required(document, "blocks")?)?;
        if blocks.is_empty() || blocks.len() > MAX_BLOCKS {
            return Err("PDF document has an invalid block count".into());
        }
        let mut tags = Vec::with_capacity(blocks.len());
        let mut text_bytes = 0;
        for input in blocks {
            let block = record(input)?;
            fields(block, &["role", "text"])?;
            let role = text(required(block, "role")?)?;
            let actual_text = text(required(block, "text")?)?;
            text_bytes += actual_text.len();
            if !valid_role(role)
                || actual_text.trim().is_empty()
                || text_bytes > MAX_TEXT_BYTES
                || actual_text.chars().any(|c| c.is_control() && c != '\n')
            {
                return Err("Invalid tag or text in bounded tagged-PDF block".into());
            }
            tags.push(TextBlock { role, actual_text });
        }
        let grid = read_grid(args.get("page"))?;
        let font = required(args, "font")?;
        let resource = encoded_resource(self, font)?;
        let font_len = resource.bytes.len();
        if !matches!(resource.kind.as_str(), "ttf" | "otf")
            || font_len > self.limits.resource_bytes.min(MAX_FONT_BYTES)
        {
            return Err("Supply a bounded, retained TrueType outline font".into());
        }
        self.charge(
            font_len.saturating_mul(12) + text_bytes * 1024,
            font_len * 5 + text_bytes * 128,
        )?;
        let font = &encoded_resource(self, font)?.bytes;
        let font_sha256 = sha256(font);
        let (bytes, pages, glyphs) = write_document(title, language, &tags, font, &grid)?;
        if bytes.len() > self.limits.resource_bytes.min(MAX_DOCUMENT_BYTES) {
            return Err("Tagged PDF output exceeds admitted size".into());
        }
        let metadata = json!({
            "schema":"qualia-tagged-pdf/1", "profile":"pdf15_tagged_unicode",
            "page_count":pages, "block_count":blocks.len(), "glyph_count":glyphs,
            "width_pt":grid.width, "height_pt":grid.height, "margin_pt":grid.margin,
            "font_sha256":font_sha256, "sha256":sha256(&bytes)
        });
        let document = self.encode("pdf", bytes)?;
        Ok(result(&[
            ("pdf", document),
            ("report", encode_json(self, &metadata)?),
        ]))
    }

    pub(crate) fn decompile_tagged_pdf_ua(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["source", "font"])?;
        let source = required(args, "source")?;
        let resource = encoded_resource(self, source)?;
        let len = resource.bytes.len();
        if resource.kind != "pdf" || len > self.limits.resource_bytes.min(MAX_DOCUMENT_BYTES) {
            return Err("Expected a bounded tagged PDF resource".into());
        }
        self.charge(len.saturating_mul(12), len.saturating_mul(3))?;
        let bytes = &encoded_resource(self, source)?.bytes;
        let source_hash = sha256(bytes);
        let DecodedDocument {
            document,
            metadata,
            page,
            font_sha256,
        } = read_document(bytes)?;
        let reflow_args = if let Some(font) = args.get("font") {
            let resource = encoded_resource(self, font)?;
            if !matches!(resource.kind.as_str(), "ttf" | "otf")
                || resource.bytes.len() > MAX_FONT_BYTES
                || sha256(&resource.bytes) != font_sha256
            {
                return Err(
                    "Reflow needs retained font bytes matching the embedded TrueType font".into(),
                );
            }
            let page = page
                .as_ref()
                .ok_or("PDF has no retained page authoring recipe")?;
            Some(result(&[
                ("document", document.clone()),
                ("font", font.clone()),
                ("page", page.to_value()),
            ]))
        } else {
            None
        };
        let output_hash =
            sha256(&serde_json::to_vec(&metadata).map_err(|error| error.to_string())?);
        let report = encode_json(self, &metadata)?;
        let receipt = receipt(
            self,
            "urn:qualia:format:tagged-pdf-reading-order",
            &source_hash,
            &output_hash,
        )?;
        let mut output = Args::from([
            ("document".into(), document),
            ("font_sha256".into(), Value::String(font_sha256)),
            ("report".into(), report),
            ("receipt".into(), receipt),
        ]);
        if let Some(page) = page {
            output.insert("page".into(), page.to_value());
        }
        if let Some(reflow_args) = reflow_args {
            output.insert("reflow_args".into(), reflow_args);
        }
        Ok(Value::Record(output))
    }
}

fn valid_role(role: &str) -> bool {
    matches!(role, "H1" | "H2" | "H3" | "H4" | "H5" | "H6" | "P")
}

fn valid_language(value: &str) -> bool {
    value.len() <= 35
        && value.split('-').enumerate().all(|(index, item)| {
            (1..=8).contains(&item.len())
                && item
                    .bytes()
                    .all(|b| b.is_ascii_alphabetic() || (index > 0 && b.is_ascii_digit()))
        })
}

fn read_grid(value: Option<&Value>) -> Result<Grid, String> {
    let empty = Args::new();
    let args = value.map(record).transpose()?.unwrap_or(&empty);
    fields(
        args,
        &[
            "width_pt",
            "height_pt",
            "margin_pt",
            "font_size_pt",
            "paragraph_spacing_pt",
        ],
    )?;
    let grid = Grid {
        width: number(args, "width_pt", 595.28)?,
        height: number(args, "height_pt", 841.89)?,
        margin: number(args, "margin_pt", 54.0)?,
        size: number(args, "font_size_pt", 11.0)?,
        spacing: number(args, "paragraph_spacing_pt", 8.0)?,
    };
    grid.validate()?;
    Ok(grid)
}

impl Grid {
    fn validate(&self) -> Result<(), String> {
        if !(144.0..=1800.0).contains(&self.width)
            || !(144.0..=1800.0).contains(&self.height)
            || !(6.0..=144.0).contains(&self.margin)
            || !(6.0..=48.0).contains(&self.size)
            || !(0.0..=144.0).contains(&self.spacing)
            || self.width <= 2.0 * self.margin + 3.0 * self.size
            || self.height <= 2.0 * self.margin + 3.0 * self.size
        {
            return Err("Page dimensions do not admit the specified margins and typography".into());
        }
        Ok(())
    }

    fn to_value(&self) -> Value {
        result(&[
            ("width_pt", Value::F64(self.width)),
            ("height_pt", Value::F64(self.height)),
            ("margin_pt", Value::F64(self.margin)),
            ("font_size_pt", Value::F64(self.size)),
            ("paragraph_spacing_pt", Value::F64(self.spacing)),
        ])
    }
}

fn write_document(
    title: &str,
    language: &str,
    blocks: &[TextBlock<'_>],
    bytes: &[u8],
    grid: &Grid,
) -> Result<(Vec<u8>, usize, usize), String> {
    let face =
        ttf_parser::Face::parse(bytes, 0).map_err(|err| format!("TrueType font: {err:?}"))?;
    if !bytes.starts_with(&[0, 1, 0, 0])
        || face.is_variable()
        || face.tables().glyf.is_none()
        || !face.is_outline_embedding_allowed()
        || !matches!(
            face.permissions(),
            Some(
                ttf_parser::Permissions::Installable
                    | ttf_parser::Permissions::Editable
                    | ttf_parser::Permissions::PreviewAndPrint
            )
        )
    {
        return Err(
            "Expected a static TrueType outline font with document embedding permission".into(),
        );
    }
    let ttf = Ttf::parse(bytes).map_err(|err| err.to_string())?;
    let mut fonts = FontContext {
        collection: parley::fontique::Collection::new(parley::fontique::CollectionOptions {
            shared: false,
            system_fonts: false,
        }),
        source_cache: Default::default(),
    };
    let families = fonts
        .collection
        .register_fonts(peniko::Blob::new(Arc::new(bytes.to_vec())), None);
    let family = families
        .first()
        .and_then(|family| fonts.collection.family_name(family.0))
        .ok_or("Actual font has no usable family")?
        .replace('"', "");
    let family = format!("\"{family}\"");
    let mut ctx = LayoutContext::<()>::new();
    let mut output = PdfDoc::new();
    let mut page = PdfPage::new(grid.width, grid.height);
    let mut page_tags = Vec::new();
    let mut run_tags = Vec::new();
    let mut cmap = BTreeMap::new();
    let mut glyph_count = 0;
    let mut top = grid.margin;
    for (block_index, block) in blocks.iter().enumerate() {
        let value = block.actual_text;
        let font_size = if block.role == "P" {
            grid.size
        } else {
            grid.size * 1.25
        };
        let mut builder = ctx.ranged_builder(&mut fonts, value, 1.0, true);
        builder.push_default(StyleProperty::FontSize(font_size as f32));
        builder.push_default(StyleProperty::FontFamily(FontFamily::Source(
            Cow::Borrowed(&family),
        )));
        builder.push_default(StyleProperty::Locale(
            parley::fontique::Language::parse(language).ok(),
        ));
        let mut layout: Layout<()> = builder.build(value);
        layout.break_all_lines(Some((grid.width - 2.0 * grid.margin) as f32));
        layout.align(Alignment::Start, AlignmentOptions::default());
        for line in layout.lines() {
            let metrics = line.metrics();
            let line_origin = f64::from(metrics.block_min_coord);
            let line_height = f64::from(metrics.line_height).max(font_size);
            if !line_origin.is_finite()
                || !line_height.is_finite()
                || line_height > grid.height - 2.0 * grid.margin
                || f64::from(metrics.advance - metrics.hanging_advance)
                    > grid.width - 2.0 * grid.margin + 0.05
            {
                return Err("Shaped line cannot fit the page grid".into());
            }
            if top + line_height > grid.height - grid.margin {
                if !page.runs.is_empty() {
                    if page_tags.len() + 1 >= MAX_PAGES {
                        return Err("Tagged PDF exceeds the page budget".into());
                    }
                    output.add_page(page);
                    page_tags.push(run_tags);
                    page = PdfPage::new(grid.width, grid.height);
                    run_tags = Vec::new();
                }
                top = grid.margin;
            }
            for item in line.items() {
                let PositionedLayoutItem::GlyphRun(run) = item else {
                    continue;
                };
                let source = run.run().font();
                if source.index != 0 || source.data.as_ref() != bytes {
                    return Err("Shaper selected a face outside the retained font".into());
                }
                // Map shaped glyphs, including single-glyph ligatures, to their cluster.
                // ActualText carries the complete block when a glyph has more than one source.
                for cluster in run.run().clusters() {
                    let cluster_text = value
                        .get(cluster.text_range())
                        .ok_or("Invalid Unicode cluster range")?;
                    let glyphs = cluster.glyphs();
                    let exact = glyphs.clone().count() == cluster_text.chars().count();
                    let mut scalars = cluster_text.chars();
                    for (index, glyph) in glyphs.enumerate() {
                        let scalar = if exact { scalars.next() } else { None };
                        cmap.entry(glyph.id as u16).or_insert_with(|| {
                            if let Some(scalar) = scalar {
                                scalar.to_string()
                            } else if index == 0 && !cluster_text.is_empty() {
                                cluster_text.to_owned()
                            } else {
                                "\u{feff}".into()
                            }
                        });
                    }
                }
                for glyph in run.positioned_glyphs() {
                    glyph_count += 1;
                    if glyph_count > MAX_GLYPHS
                        || glyph.id == 0
                        || glyph.id >= u32::from(face.number_of_glyphs())
                        || !glyph.x.is_finite()
                        || !glyph.y.is_finite()
                    {
                        return Err("Missing shaped glyph or exceeded bounded glyph counts".into());
                    }
                    let x = grid.margin + f64::from(glyph.x);
                    let y = grid.height - (top + f64::from(glyph.y) - line_origin);
                    page.text("F1", font_size, x, y, vec![glyph.id as u16]);
                    run_tags.push(block_index);
                }
            }
            top += line_height;
        }
        top += grid.spacing;
    }
    if !page.runs.is_empty() {
        output.add_page(page);
        page_tags.push(run_tags);
    }
    output.font("F1", ttf, BTreeMap::new());
    let bbox = face.global_bounding_box();
    let tags = DocumentTags {
        title,
        language,
        blocks,
        page_run_blocks: &page_tags,
        glyph_unicode: &cmap,
        font_bounds: [bbox.x_min, bbox.y_min, bbox.x_max, bbox.y_max],
        italic_angle: face.italic_angle(),
    };
    let bytes = output
        .save_tagged_with_page_layout(
            &tags,
            [
                grid.width,
                grid.height,
                grid.margin,
                grid.size,
                grid.spacing,
            ],
        )
        .map_err(|err| err.to_string())?;
    Ok((bytes, page_tags.len(), glyph_count))
}

struct DecodedDocument {
    document: Value,
    metadata: serde_json::Value,
    page: Option<Grid>,
    font_sha256: String,
}

fn read_document(bytes: &[u8]) -> Result<DecodedDocument, String> {
    let pdf = ClassicPdf::open(bytes)?;
    let catalog = pdf.dictionary(pdf.root)?;
    catalog.expect("/Type", "/Catalog")?;
    catalog.expect("/Marked", "true")?;
    let language = catalog
        .after("/Lang")?
        .trim_start_matches('(')
        .trim_end_matches(')');
    if !valid_language(language) {
        return Err("Invalid tagged PDF document language".into());
    }
    let title = decode_text(pdf.dictionary(pdf.info)?.after("/Title")?)?;
    if title.trim().is_empty() || title.len() > 512 || title.chars().any(char::is_control) {
        return Err("Invalid tagged PDF document title".into());
    }
    let layout = catalog
        .optional_rest("/QualiaPageLayout")
        .map(read_page_recipe)
        .transpose()?;
    let pages_id = catalog.reference("/Pages")?;
    let pages = pdf.dictionary(pages_id)?;
    pages.expect("/Type", "/Pages")?;
    let page_ids = pages.references("/Kids")?;
    if page_ids.is_empty()
        || page_ids.len() > MAX_PAGES
        || pages.after("/Count")?.parse::<usize>().ok() != Some(page_ids.len())
    {
        return Err("Invalid tagged PDF page tree".into());
    }
    let mut page_content = BTreeMap::new();
    let mut glyph_count = 0;
    let mut font_id = None;
    for (index, &id) in page_ids.iter().enumerate() {
        let page = pdf.dictionary(id)?;
        page.expect("/Type", "/Page")?;
        page.expect("/Tabs", "/S")?;
        let face = page.reference("/F1")?;
        if font_id.replace(face).is_some_and(|prior| prior != face) {
            return Err("Tagged PDF pages have conflicting font resources".into());
        }
        if let Some(layout) = &layout {
            let mut bounds = TokenReader::new(page.rest("/MediaBox")?);
            bounds.token("[")?;
            let values = [
                bounds.real()?,
                bounds.real()?,
                bounds.real()?,
                bounds.real()?,
            ];
            bounds.token("]")?;
            if values[0] != 0.0
                || values[1] != 0.0
                || (values[2] - layout.width).abs() > 0.005_000_01
                || (values[3] - layout.height).abs() > 0.005_000_01
            {
                return Err("Tagged PDF page extent conflicts with the authoring recipe".into());
            }
        }
        let marks = read_marks(pdf.stream(page.reference("/Contents")?)?, &mut glyph_count)?;
        if page.reference("/Parent")? != pages_id
            || page.after("/StructParents")?.parse::<usize>().ok() != Some(index)
            || page_content.insert(id, (index, marks)).is_some()
        {
            return Err("Inconsistent tagged PDF page reference".into());
        }
    }
    let font = pdf.embedded_font(font_id.ok_or("Tagged PDF has no font resource")?)?;
    let face = ttf_parser::Face::parse(font, 0)
        .map_err(|_| "Tagged PDF font stream is not a supported TrueType face")?;
    if !font.starts_with(&[0, 1, 0, 0])
        || face.is_variable()
        || face.tables().glyf.is_none()
        || page_content
            .values()
            .flat_map(|(_, marks)| marks)
            .any(|mark| {
                mark.glyphs
                    .iter()
                    .any(|(gid, _, _, _)| *gid >= face.number_of_glyphs())
            })
    {
        return Err(
            "Tagged PDF glyph indices or embedded font are outside the authoring profile".into(),
        );
    }
    let font_sha256 = sha256(font);
    let root_id = catalog.reference("/StructTreeRoot")?;
    let root = pdf.dictionary(root_id)?;
    root.expect("/Type", "/StructTreeRoot")?;
    let document_ids = root.references("/K")?;
    let [doc_id] = document_ids.as_slice() else {
        return Err("Expected one tagged PDF document element".into());
    };
    let doc = pdf.dictionary(*doc_id)?;
    doc.expect("/Type", "/StructElem")?;
    doc.expect("/S", "/Document")?;
    if doc.reference("/P")? != root_id {
        return Err("Invalid tagged PDF document parent".into());
    }
    let ids = doc.references("/K")?;
    if ids.is_empty() || ids.len() > MAX_BLOCKS {
        return Err("Invalid tagged PDF block count".into());
    }
    let parent = pdf.dictionary(root.reference("/ParentTree")?)?;
    let mut parent_numbers = TokenReader::new(parent.rest("/Nums")?);
    parent_numbers.token("[")?;
    let mut parent_map = Vec::new();
    for n in 0..page_ids.len() {
        if parent_numbers.integer()? != n {
            return Err("Unordered tagged PDF parent tree".into());
        }
        parent_numbers.token("[")?;
        let mut parents = Vec::new();
        while !parent_numbers.eat("]") {
            parents.push(parent_numbers.reference()?);
        }
        parent_map.push(parents);
    }
    parent_numbers.token("]")?;
    let mut seen = BTreeSet::new();
    let mut entries = Vec::new();
    let mut metadata = Vec::new();
    let mut text_bytes = 0;
    for id in ids {
        let block = pdf.dictionary(id)?;
        block.expect("/Type", "/StructElem")?;
        if block.reference("/P")? != *doc_id {
            return Err("Invalid tagged PDF block parent".into());
        }
        let role = block
            .after("/S")?
            .strip_prefix('/')
            .ok_or("Malformed tagged PDF role")?;
        if !valid_role(role) {
            return Err("Unsupported tagged PDF block role".into());
        }
        let value = decode_text(block.after("/ActualText")?)?;
        text_bytes += value.len();
        if value.trim().is_empty()
            || text_bytes > MAX_TEXT_BYTES
            || value.chars().any(|c| c.is_control() && c != '\n')
        {
            return Err("Tagged PDF accessible text exceeds the profile bounds".into());
        }
        let mut refs = TokenReader::new(block.rest("/K")?);
        refs.token("[")?;
        let mut sections = Vec::new();
        while !refs.eat("]") {
            refs.token("<<")?;
            refs.token("/Type")?;
            refs.token("/MCR")?;
            refs.token("/Pg")?;
            let page_id = refs.reference()?;
            refs.token("/MCID")?;
            let mcid = refs.integer()?;
            refs.token(">>")?;
            let (page_number, marks) = page_content
                .get(&page_id)
                .ok_or("Marked content selects an unknown page")?;
            if parent_map[*page_number].get(mcid).copied() != Some(id)
                || !seen.insert((*page_number, mcid))
            {
                return Err("Tagged PDF marked content parent does not match reading order".into());
            }
            let mark = marks
                .get(mcid)
                .filter(|mark| mark.role == role)
                .ok_or("Tagged PDF block lacks its marked content stream")?;
            sections.push(json!({ "page":page_number, "glyphs":mark.glyphs }));
        }
        if sections.is_empty() {
            return Err("Tagged PDF block has no marked content reference".into());
        }
        metadata.push(json!({ "role":role, "text":value, "sections":sections }));
        entries.push(result(&[
            ("role", Value::String(role.into())),
            ("text", Value::String(value)),
        ]));
    }
    if seen.len() != parent_map.iter().map(Vec::len).sum::<usize>()
        || seen.len()
            != page_content
                .values()
                .map(|(_, marks)| marks.len())
                .sum::<usize>()
    {
        return Err("Tagged PDF has unreferenced parent-tree entries".into());
    }
    let document = result(&[
        ("title", Value::String(title.clone())),
        ("language", Value::String(language.into())),
        ("blocks", Value::List(entries)),
    ]);
    let metadata = json!({
        "schema":"qualia-tagged-pdf-reading-order/1", "title":title, "language":language,
        "page_count":page_ids.len(), "glyph_count":glyph_count, "blocks":metadata,
        "font_sha256":font_sha256, "has_page_authoring_recipe":layout.is_some(),
        "page":layout.as_ref().map(|page| json!({
            "width_pt":page.width, "height_pt":page.height, "margin_pt":page.margin,
            "font_size_pt":page.size, "paragraph_spacing_pt":page.spacing,
        })),
    });
    Ok(DecodedDocument {
        document,
        metadata,
        page: layout,
        font_sha256,
    })
}

fn read_page_recipe(tokens: &[&str]) -> Result<Grid, String> {
    let mut values = TokenReader::new(tokens);
    values.token("[")?;
    let page = Grid {
        width: values.real()?,
        height: values.real()?,
        margin: values.real()?,
        size: values.real()?,
        spacing: values.real()?,
    };
    values.token("]")?;
    page.validate()?;
    Ok(page)
}

struct Mark<'a> {
    role: &'a str,
    glyphs: Vec<(u16, f64, f64, f64)>,
}

fn read_marks<'a>(content: &'a str, glyph_count: &mut usize) -> Result<Vec<Mark<'a>>, String> {
    let mut cursor = content.lines();
    let mut output = Vec::new();
    while let Some(marker) = cursor.next() {
        let (role, number) = marker
            .strip_prefix('/')
            .and_then(|line| line.split_once(" << /MCID "))
            .and_then(|(role, rest)| rest.strip_suffix(" >> BDC").map(|n| (role, n)))
            .ok_or("Unsupported marked PDF block syntax")?;
        if !valid_role(role)
            || number.parse::<usize>().ok() != Some(output.len())
            || output.len() >= MAX_BLOCKS
        {
            return Err("Invalid PDF marked content role or index".into());
        }
        let mut glyphs = Vec::new();
        loop {
            let line = cursor
                .next()
                .ok_or("Missing PDF marked-content terminator")?;
            if line == "EMC" {
                break;
            }
            *glyph_count += 1;
            if *glyph_count > MAX_GLYPHS || line.len() > 256 {
                return Err("Tagged PDF exceeds bounded glyph layout".into());
            }
            let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
            let [
                "BT",
                "/F1",
                size,
                "Tf",
                "1",
                "0",
                "0",
                "1",
                x,
                y,
                "Tm",
                glyph,
                "Tj",
                "ET",
            ] = tokens.as_slice()
            else {
                return Err("Unsupported positioned PDF glyph syntax".into());
            };
            let glyph = glyph
                .strip_prefix('<')
                .and_then(|s| s.strip_suffix('>'))
                .filter(|s| s.len() == 4)
                .and_then(|s| u16::from_str_radix(s, 16).ok())
                .filter(|&g| g != 0)
                .ok_or("Invalid PDF glyph index")?;
            let finite = |text: &str| {
                text.parse::<f64>()
                    .ok()
                    .filter(|x| x.is_finite())
                    .ok_or("Invalid PDF glyph coordinate")
            };
            let size = finite(size)?;
            if !(6.0..=60.0).contains(&size) {
                return Err("PDF glyph size exceeds profile bounds".into());
            }
            glyphs.push((glyph, finite(x)?, finite(y)?, size));
        }
        if glyphs.is_empty() {
            return Err("PDF marked content contains no glyph runs".into());
        }
        output.push(Mark { role, glyphs });
    }
    Ok(output)
}

struct ClassicPdf<'a> {
    bytes: &'a [u8],
    offsets: Vec<usize>,
    root: usize,
    info: usize,
}

impl<'a> ClassicPdf<'a> {
    fn open(bytes: &'a [u8]) -> Result<Self, String> {
        if !bytes.starts_with(b"%PDF-1.5\n") {
            return Err(
                "The tagged Unicode roundtrip reader expects its classic PDF 1.5 profile".into(),
            );
        }
        let trailer_end = bytes.len().saturating_sub(64);
        let tail =
            std::str::from_utf8(&bytes[trailer_end..]).map_err(|_| "Invalid PDF end marker")?;
        let (_, pointer) = tail
            .rsplit_once("\nstartxref\n")
            .ok_or("Missing PDF startxref")?;
        let (xref, end) = pointer.split_once('\n').ok_or("Invalid PDF startxref")?;
        if end.trim() != "%%EOF" {
            return Err("Invalid PDF end marker".into());
        }
        let xref = xref
            .parse::<usize>()
            .map_err(|_| "Invalid PDF xref offset")?;
        let index = std::str::from_utf8(bytes.get(xref..).ok_or("Out-of-bounds PDF xref")?)
            .map_err(|_| "Invalid PDF classic xref")?;
        let mut lines = index.lines();
        if lines.next() != Some("xref") {
            return Err("Expected classic PDF xref table".into());
        }
        let count = lines
            .next()
            .and_then(|line| line.strip_prefix("0 "))
            .and_then(|n| n.parse::<usize>().ok())
            .filter(|n| (15..=MAX_BLOCKS + MAX_PAGES * 2 + 32).contains(n))
            .ok_or("Invalid PDF xref table count")?;
        if lines.next() != Some("0000000000 65535 f ") {
            return Err("Invalid PDF xref first row".into());
        }
        let mut offsets = vec![0];
        for id in 1..count {
            let line = lines.next().ok_or("Truncated PDF xref table")?;
            let offset = line
                .strip_suffix(" 00000 n ")
                .filter(|n| n.len() == 10)
                .and_then(|n| n.parse::<usize>().ok())
                .filter(|&n| *offsets.last().unwrap_or(&0) < n && n < xref)
                .ok_or("Invalid PDF xref table row")?;
            let prefix = format!("{id} 0 obj\n");
            if !bytes[offset..xref].starts_with(prefix.as_bytes()) {
                return Err("PDF xref points outside the claimed object".into());
            }
            offsets.push(offset);
        }
        offsets.push(xref);
        if lines.next() != Some("trailer") {
            return Err("Missing PDF trailer".into());
        }
        let trailer = Dictionary(tokenize(lines.next().ok_or("Truncated PDF trailer")?));
        if trailer.after("/Size")?.parse::<usize>().ok() != Some(count) {
            return Err("Invalid PDF trailer size".into());
        }
        Ok(Self {
            bytes,
            offsets,
            root: trailer.reference("/Root")?,
            info: trailer.reference("/Info")?,
        })
    }

    fn object(&self, id: usize) -> Result<&'a [u8], String> {
        if id == 0 || id >= self.offsets.len() - 1 {
            return Err("Tagged PDF object reference out of range".into());
        }
        let marker = format!("{id} 0 obj\n");
        self.bytes[self.offsets[id]..self.offsets[id + 1]]
            .strip_prefix(marker.as_bytes())
            .and_then(|body| body.strip_suffix(b"\nendobj\n"))
            .ok_or_else(|| "Invalid PDF object extent".into())
    }

    fn dictionary(&self, id: usize) -> Result<Dictionary<'a>, String> {
        self.dictionary_bounded(id, MAX_TEXT_BYTES * 5)
    }

    fn dictionary_bounded(&self, id: usize, limit: usize) -> Result<Dictionary<'a>, String> {
        let object = self.object(id)?;
        if object.len() > limit {
            return Err("Tagged PDF dictionary exceeds size bound".into());
        }
        let text = std::str::from_utf8(object).map_err(|_| "Invalid tagged PDF dictionary")?;
        if !text.starts_with("<< ") || !text.ends_with(" >>") {
            return Err("Expected an inline PDF dictionary".into());
        }
        Ok(Dictionary(tokenize(text)))
    }

    fn embedded_font(&self, id: usize) -> Result<&'a [u8], String> {
        let font = self.dictionary(id)?;
        font.expect("/Type", "/Font")?;
        font.expect("/Subtype", "/Type0")?;
        font.expect("/Encoding", "/Identity-H")?;
        let descendants = font.references("/DescendantFonts")?;
        let [cid_id] = descendants.as_slice() else {
            return Err("Tagged PDF requires one TrueType descendant font".into());
        };
        let cid = self.dictionary_bounded(*cid_id, MAX_FONT_DICTIONARY_BYTES)?;
        cid.expect("/Type", "/Font")?;
        cid.expect("/Subtype", "/CIDFontType2")?;
        cid.expect("/CIDToGIDMap", "/Identity")?;
        let descriptor = self.dictionary(cid.reference("/FontDescriptor")?)?;
        descriptor.expect("/Type", "/FontDescriptor")?;
        let object = self.object(descriptor.reference("/FontFile2")?)?;
        let line_end = object
            .iter()
            .take(128)
            .position(|&byte| byte == b'\n')
            .ok_or("Missing tagged PDF font stream header")?;
        let line = std::str::from_utf8(&object[..line_end])
            .map_err(|_| "Invalid tagged PDF font stream header")?;
        let (length, original_length) = line
            .strip_prefix("<< /Length ")
            .and_then(|line| line.strip_suffix(" >>"))
            .and_then(|line| line.split_once(" /Length1 "))
            .ok_or("Unsupported tagged PDF font stream header")?;
        let length = length
            .parse::<usize>()
            .ok()
            .filter(|length| (1..=MAX_FONT_BYTES).contains(length))
            .ok_or("Invalid tagged PDF font length")?;
        if original_length.parse::<usize>().ok() != Some(length) {
            return Err("Tagged PDF embedded font length fields disagree".into());
        }
        object[line_end..]
            .strip_prefix(b"\nstream\n")
            .and_then(|body| body.strip_suffix(b"\nendstream"))
            .filter(|body| body.len() == length)
            .ok_or_else(|| "Invalid tagged PDF embedded font extent".into())
    }

    fn stream(&self, id: usize) -> Result<&'a str, String> {
        let object = self.object(id)?;
        let line_end = object
            .iter()
            .position(|&b| b == b'\n')
            .ok_or("Missing PDF stream dictionary")?;
        let line =
            std::str::from_utf8(&object[..line_end]).map_err(|_| "Invalid PDF stream header")?;
        let len = line
            .strip_prefix("<< /Length ")
            .and_then(|s| s.strip_suffix(" >>"))
            .and_then(|n| n.parse::<usize>().ok())
            .ok_or("Unsupported PDF stream dictionary")?;
        let content = object[line_end..]
            .strip_prefix(b"\nstream\n")
            .and_then(|body| body.strip_suffix(b"\nendstream"))
            .filter(|body| body.len() == len)
            .ok_or("Invalid PDF stream length")?;
        std::str::from_utf8(content).map_err(|_| "Invalid PDF tagged text content stream".into())
    }
}

struct Dictionary<'a>(Vec<&'a str>);

impl<'a> Dictionary<'a> {
    fn optional_rest(&self, name: &str) -> Option<&[&'a str]> {
        self.0
            .iter()
            .position(|&t| t == name)
            .map(|position| &self.0[position + 1..])
    }

    fn rest(&self, name: &str) -> Result<&[&'a str], String> {
        self.optional_rest(name)
            .ok_or_else(|| format!("Missing PDF {name}"))
    }

    fn after(&self, name: &str) -> Result<&'a str, String> {
        self.rest(name)?
            .first()
            .copied()
            .ok_or_else(|| format!("Missing PDF {name} value"))
    }

    fn expect(&self, key: &str, value: &str) -> Result<(), String> {
        if self.after(key)? == value {
            Ok(())
        } else {
            Err(format!("Unexpected PDF {key} value"))
        }
    }

    fn reference(&self, name: &str) -> Result<usize, String> {
        TokenReader::new(self.rest(name)?).reference()
    }

    fn references(&self, name: &str) -> Result<Vec<usize>, String> {
        let mut tokens = TokenReader::new(self.rest(name)?);
        tokens.token("[")?;
        let mut refs = Vec::new();
        while !tokens.eat("]") {
            refs.push(tokens.reference()?);
        }
        Ok(refs)
    }
}

fn tokenize(input: &str) -> Vec<&str> {
    let mut tokens = Vec::new();
    for word in input.split_ascii_whitespace() {
        let stripped = word.trim_matches(['[', ']']);
        let leading = word.len() - word.trim_start_matches(['[', ']']).len();
        tokens.extend(
            word[..leading]
                .as_bytes()
                .iter()
                .map(|&b| if b == b'[' { "[" } else { "]" }),
        );
        if !stripped.is_empty() {
            tokens.push(stripped);
            tokens.extend(
                word[leading + stripped.len()..]
                    .as_bytes()
                    .iter()
                    .map(|&b| if b == b'[' { "[" } else { "]" }),
            );
        }
    }
    tokens
}

struct TokenReader<'a, 'b>(&'a [&'b str]);

impl<'a, 'b> TokenReader<'a, 'b> {
    fn new(tokens: &'a [&'b str]) -> Self {
        Self(tokens)
    }

    fn eat(&mut self, token: &str) -> bool {
        if self.0.first().copied() == Some(token) {
            self.0 = &self.0[1..];
            true
        } else {
            false
        }
    }

    fn token(&mut self, token: &str) -> Result<(), String> {
        if self.eat(token) {
            Ok(())
        } else {
            Err(format!("Expected PDF token {token}"))
        }
    }

    fn integer(&mut self) -> Result<usize, String> {
        let (&value, rest) = self.0.split_first().ok_or("Missing PDF integer")?;
        self.0 = rest;
        value.parse().map_err(|_| "Invalid PDF integer".into())
    }

    fn real(&mut self) -> Result<f64, String> {
        let (&value, rest) = self.0.split_first().ok_or("Missing PDF number")?;
        self.0 = rest;
        value
            .parse::<f64>()
            .ok()
            .filter(|value| value.is_finite())
            .ok_or_else(|| "Invalid finite PDF number".into())
    }

    fn reference(&mut self) -> Result<usize, String> {
        let id = self.integer()?;
        self.token("0")?;
        self.token("R")?;
        Ok(id)
    }
}

fn decode_text(encoded: &str) -> Result<String, String> {
    let hex = encoded
        .strip_prefix("<FEFF")
        .and_then(|text| text.strip_suffix('>'))
        .filter(|text| text.len() % 4 == 0 && text.len() <= MAX_TEXT_BYTES * 4)
        .ok_or("Expected UTF-16BE tagged PDF text")?;
    let mut units = Vec::with_capacity(hex.len() / 4);
    for chunk in hex.as_bytes().chunks_exact(4) {
        let chunk = std::str::from_utf8(chunk).map_err(|_| "Invalid PDF character hex")?;
        units.push(u16::from_str_radix(chunk, 16).map_err(|_| "Invalid PDF character hex")?);
    }
    String::from_utf16(&units).map_err(|_| "Invalid UTF-16BE tagged PDF string".into())
}
