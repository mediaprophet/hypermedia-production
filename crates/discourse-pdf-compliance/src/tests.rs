//! Comprehensive unit tests for discourse-pdf-compliance.

#![forbid(unsafe_code)]

use super::a11y::{
    AccessibleDocSnapshot, AccessibleFigure, AccessibleFormField, AccessibleHeading,
    AccessibleList, AccessibleTable, Rule, Status, check_accessibility, generate_html_report,
};
use super::forms::{
    AcroForm, AcroFormField, CalcOp, CalculateRule, FieldKind, FormatRule, ValidateRule,
    execute_calculation, export_xfdf, flatten_form, format_field, import_xfdf, validate_field,
};
use super::preflight::{
    PreflightSnapshot, StandardProfile, evaluate_preflight,
};
use super::redact::{
    HiddenCategory, SanitizeManifest, apply_semantic_redaction, luhn_check, sanitize_document,
    scan_all_pii, scan_credit_cards, scan_emails, scan_phones, scan_ssns, scan_tax_ids,
    verify_zero_residue,
};
use super::sign::{
    ByteRange, TimestampToken, verify_pades_byte_range, verify_rfc3161_timestamp,
};

#[test]
fn test_a11y_32_rules_and_report() {
    let doc = AccessibleDocSnapshot {
        title: Some("Human Rights Treaty Master".into()),
        language: Some("en-US".into()),
        is_tagged: true,
        has_permission_flag: true,
        is_image_only: false,
        page_count: 5,
        has_bookmarks: true,
        tab_order_consistent: true,
        character_encoding_valid: true,
        untagged_content_count: 0,
        untagged_annotations_count: 0,
        multimedia_tagged: true,
        screen_flicker_detected: false,
        scripts_accessible: true,
        timed_responses_extendable: true,
        navigation_links_valid: true,
        form_fields: vec![AccessibleFormField {
            name: "Signature".into(),
            is_tagged: true,
            tooltip: Some("Digital signature field".into()),
        }],
        figures: vec![AccessibleFigure {
            id: 1,
            alt_text: Some("Organizational diagram".into()),
            is_decorative: false,
            nested_alt: false,
            hides_annotation: false,
        }],
        tables: vec![AccessibleTable {
            id: 10,
            has_tr_rows: true,
            has_valid_cells: true,
            has_headers: true,
            is_regular: true,
            summary: Some("Summary of metrics".into()),
        }],
        lists: vec![AccessibleList {
            id: 20,
            items_are_li: true,
            items_have_lbl_lbody: true,
        }],
        headings: vec![
            AccessibleHeading { level: 1, text: "Chapter 1".into() },
            AccessibleHeading { level: 2, text: "Section 1.1".into() },
        ],
    };

    let verdicts = check_accessibility(&doc);
    assert_eq!(verdicts.len(), 32);

    let tagged_verdict = verdicts.iter().find(|v| v.rule == Rule::TaggedPdf).unwrap();
    assert_eq!(tagged_verdict.status, Status::Passed);

    let html = generate_html_report("Human Rights Treaty Master", &verdicts);
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Human Rights Treaty Master"));
}

#[test]
fn test_redaction_pii_and_zero_residue() {
    let sample = "Contact: john.doe@example.org, SSN: 123-45-6789, Tax: 12-3456789, Phone: +1 555 123 4567";
    let matches = scan_all_pii(sample);
    assert!(matches.iter().any(|m| m.text == "john.doe@example.org"));
    assert!(matches.iter().any(|m| m.text == "123-45-6789"));
    assert!(matches.iter().any(|m| m.text == "12-3456789"));

    // Credit card Luhn validation
    let valid_visa = [4, 5, 3, 9, 1, 4, 8, 8, 0, 3, 4, 3, 6, 4, 6, 7];
    assert!(luhn_check(&valid_visa));
    let invalid_visa = [4, 5, 3, 9, 1, 4, 8, 8, 0, 3, 4, 3, 6, 4, 6, 8];
    assert!(!luhn_check(&invalid_visa));

    // Semantic fail-closed redaction
    let ssn_range = sample.find("123-45-6789").unwrap()..sample.find("123-45-6789").unwrap() + 11;
    let redacted = apply_semantic_redaction(sample, &[ssn_range], '█');
    assert!(!redacted.contains("123-45-6789"));

    // Zero-residue verification
    let verif = verify_zero_residue(&redacted, &["123-45-6789"]);
    assert!(verif.verified_clean);
    assert_eq!(verif.residual_patterns_found, 0);

    // Sanitization of manifest
    let mut manifest = SanitizeManifest {
        author: Some("Private Investigator".into()),
        creator: Some("Authoring Tool".into()),
        producer: Some("PDF Engine".into()),
        xmp_packets: vec!["<x:xmpmeta>...</x:xmpmeta>".into()],
        attachment_names: vec!["secret.xlsx".into()],
        comment_count: 5,
        form_field_count: 2,
        hidden_layer_count: 1,
        bookmark_count: 3,
        script_count: 1,
        private_data_entries: vec!["IllustratorPieceInfo".into()],
    };
    let report = sanitize_document(&mut manifest, &HiddenCategory::ALL);
    assert_eq!(report.stripped_metadata_keys.len(), 3);
    assert_eq!(report.stripped_attachments, 1);
    assert_eq!(manifest.author, None);
    assert_eq!(manifest.attachment_names.len(), 0);
}

#[test]
fn test_pades_signatures_and_timestamps() {
    let dummy_doc = b"HEADER_CONTENT_BEFORE_SIG_HOLE___________________[SIGNATURE_HOLE_PLACEHOLDER_48_BYTES]___FOOTER_CONTENT_AFTER_SIG_HOLE_EOF";
    let byte_range = ByteRange::new(0, 49, 97, 30);
    assert!(byte_range.validate(dummy_doc.len()).is_ok());

    let res = verify_pades_byte_range(dummy_doc, &byte_range, None);
    assert!(res.byte_range_valid);
    assert!(res.digest_matches);
    assert_ne!(res.computed_sha256, [0u8; 32]);

    // RFC 3161 timestamp verification
    let ts_token = TimestampToken {
        policy_oid: "1.3.6.1.4.1.13762.3".into(),
        hash_algorithm_oid: "2.16.840.1.101.3.4.2.1".into(),
        message_imprint: res.computed_sha256.to_vec(),
        serial_number: vec![0x01, 0x02, 0x03],
        gen_time_utc: "20261108060000Z".into(),
        nonce: Some(999888),
    };
    let ts_verif = verify_rfc3161_timestamp(&ts_token, &res.computed_sha256);
    assert!(ts_verif.valid);
    assert!(ts_verif.imprint_matches);
}

#[test]
fn test_forms_validation_flattening_and_xfdf() {
    let mut form = AcroForm::default();
    let mut f1 = AcroFormField::new_text("Quantity", 0, [10.0, 10.0, 50.0, 30.0]);
    f1.value = "5".into();
    let mut f2 = AcroFormField::new_text("UnitPrice", 0, [60.0, 10.0, 100.0, 30.0]);
    f2.value = "20.50".into();
    let mut f3 = AcroFormField::new_text("Total", 0, [110.0, 10.0, 160.0, 30.0]);

    form.fields.push(f1);
    form.fields.push(f2);
    form.fields.push(f3);

    // Calculation rule: Product of Quantity and UnitPrice
    let calc = CalculateRule::Simple {
        op: CalcOp::Product,
        field_names: vec!["Quantity".into(), "UnitPrice".into()],
    };
    let total_calc = execute_calculation(&form, &calc).expect("calc product");
    assert!((total_calc - 102.50).abs() < 1e-6);

    // Number formatting
    let formatted = format_field("102.5", &FormatRule::Number { decimals: 2, currency_prefix: "$".into() });
    assert_eq!(formatted, "$102.50");

    // Range validation
    assert!(validate_field("50", &ValidateRule::Range { min: Some(10.0), max: Some(100.0) }).is_ok());
    assert!(validate_field("5", &ValidateRule::Range { min: Some(10.0), max: Some(100.0) }).is_err());

    // Export XFDF
    let xfdf = export_xfdf(&form);
    assert!(xfdf.contains("<field name=\"Quantity\">"));
    assert!(xfdf.contains("<value>5</value>"));

    // Import XFDF into fresh form
    let mut fresh_form = AcroForm::default();
    fresh_form.fields.push(AcroFormField::new_text("Quantity", 0, [0.0; 4]));
    let imported_count = import_xfdf(&mut fresh_form, &xfdf).expect("import xfdf");
    assert_eq!(imported_count, 1);
    assert_eq!(fresh_form.field("Quantity").unwrap().value, "5");

    // Flatten form
    let flatten_rep = flatten_form(&mut form);
    assert_eq!(flatten_rep.flattened_count, 3);
    assert_eq!(form.fields.len(), 0); // interactive fields cleared
}

#[test]
fn test_preflight_pdfa_and_pdfx() {
    let good_pdfa_snapshot = PreflightSnapshot {
        pdfa_part: Some(2),
        pdfa_conformance: Some("B".into()),
        pdfx_version: None,
        is_encrypted: false,
        uses_lzw: false,
        unembedded_font_count: 0,
        output_intent_identifier: Some("GTS_PDFA1".into()),
        forbidden_actions_found: Vec::new(),
        forbidden_annotations_found: Vec::new(),
        has_rgb_content: false,
        has_trim_box: true,
        has_bleed_box: true,
        trapped_key_present: true,
        has_unattached_embedded_files: false,
    };

    let rep = evaluate_preflight(&good_pdfa_snapshot, StandardProfile::PdfA2b);
    assert!(rep.compliant);
    assert_eq!(rep.issues.len(), 0);

    let bad_pdfx_snapshot = PreflightSnapshot {
        has_rgb_content: true, // Forbidden in PDF/X-1a
        is_encrypted: true,    // Forbidden in PDF/X
        has_trim_box: false,   // Required in PDF/X
        output_intent_identifier: None,
        ..Default::default()
    };
    let rep_x = evaluate_preflight(&bad_pdfx_snapshot, StandardProfile::PdfX1a);
    assert!(!rep_x.compliant);
    assert!(rep_x.issues.iter().any(|i| i.message.contains("PDF/X-1a restricts color spaces")));
}
