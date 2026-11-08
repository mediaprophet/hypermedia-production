use layout_paged_typesetter::*;

#[test]
fn test_equation_latex_and_unicode_math() {
    let latex = "\\frac{x+1}{y-2}";
    let nodes = parse_latex(latex);
    assert_eq!(nodes.len(), 1);
    assert!(matches!(nodes[0], MNode::Frac { .. }));

    let back = to_latex(&nodes);
    assert!(back.contains("\\frac{x+1}{y-2}"));

    let linear = "(x+1)/(y-2)";
    let lin_nodes = parse_linear(linear);
    assert_eq!(lin_nodes.len(), 1);
    assert!(matches!(lin_nodes[0], MNode::Frac { .. }));

    let layout = layout_equation(&nodes, 12.0);
    assert!(layout.width > 0.0);
    assert!(layout.ascent > 0.0);
}

#[test]
fn test_liang_hyphenation() {
    let pats = Patterns::english_standard();
    let word = "hyphenation";
    let pts = pats.points(word);
    assert!(!pts.is_empty());
    let hyphed = pats.hyphenate(word, "-");
    assert!(hyphed.contains('-'));
}

#[test]
fn test_kinsoku_line_breaking() {
    let ja = "日本語、「漢字」。テスト";
    let opps = line_break_opportunities(ja, true);
    // Kinsoku: no line starts with closing quote or comma
    for &byte_pos in &opps {
        let ch = ja[byte_pos..].chars().next().unwrap();
        assert!(!NO_START.contains(ch), "Kinsoku violation: line starts with '{ch}'");
    }
}

#[test]
fn test_table_autofit() {
    let table = TableData {
        rows: vec![
            vec![TableCellData::new("Name"), TableCellData::new("Description of item")],
            vec![TableCellData::new("A"), TableCellData::new("Short text")],
        ],
    };
    let widths = autofit_widths(&table, Some(300.0), 11.0);
    assert_eq!(widths.len(), 2);
    let total: f32 = widths.iter().sum();
    assert!((total - 300.0).abs() < 1.0);
    assert!(widths[1] > widths[0]); // Description column should be wider
}

#[test]
fn test_pagination_widow_orphan() {
    let section = SectionProps {
        page_w: 612.0,
        page_h: 300.0, // Short page to force pagination
        margin_top: 36.0,
        margin_bottom: 36.0,
        margin_left: 36.0,
        margin_right: 36.0,
        num_columns: 1,
        column_gap: 0.0,
    };
    let typesetter = Typesetter::new(section);

    let paras = vec![
        ParaDoc::heading("Introduction", 1),
        ParaDoc::new("This is the first line of the test paragraph that will be typeset across the short page height with full widow and orphan control enabled so that single lines are never left alone."),
        ParaDoc::new("Another paragraph with some content to verify multi-page flow and footnote placement."),
    ];

    let pages = typesetter.paginate(&paras, &[]);
    assert!(pages.len() >= 2, "Expected multiple pages");
}
