# `crates/discourse-pdf-compliance`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_PDF/UA and WCAG accessibility compliance module._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `check_accessibility` | 1 | `crates/discourse-pdf-compliance/src/a11y/checker.rs:88` | - |  |
| `evaluate_rule` | 2 | `crates/discourse-pdf-compliance/src/a11y/checker.rs:99` | - |  |
| `summarize` | 1 | `crates/discourse-pdf-compliance/src/a11y/report.rs:18` | - |  |
| `generate_html_report` | 2 | `crates/discourse-pdf-compliance/src/a11y/report.rs:36` | - |  |
| `impl Category::label` | 1 | `crates/discourse-pdf-compliance/src/a11y/rules.rs:29` | - |  |
| `impl Rule::category` | 1 | `crates/discourse-pdf-compliance/src/a11y/rules.rs:115` | - |  |
| `impl Rule::title` | 1 | `crates/discourse-pdf-compliance/src/a11y/rules.rs:156` | - |  |
| `flatten_form` | 1 | `crates/discourse-pdf-compliance/src/forms/flatten.rs:24` | - | Flattens an AcroForm: renders the appearance of every field into static elements |
| `impl AcroFormField::new_text` | 3 | `crates/discourse-pdf-compliance/src/forms/model.rs:47` | 2d |  |
| `impl AcroFormField::new_checkbox` | 4 | `crates/discourse-pdf-compliance/src/forms/model.rs:61` | 2d |  |
| `impl AcroFormField::is_read_only` | 1 | `crates/discourse-pdf-compliance/src/forms/model.rs:75` | - |  |
| `impl AcroFormField::is_required` | 1 | `crates/discourse-pdf-compliance/src/forms/model.rs:79` | - |  |
| `impl AcroForm::field` | 2 | `crates/discourse-pdf-compliance/src/forms/model.rs:92` | - |  |
| `impl AcroForm::field_mut` | 2 | `crates/discourse-pdf-compliance/src/forms/model.rs:96` | - |  |
| `format_field` | 2 | `crates/discourse-pdf-compliance/src/forms/validation.rs:42` | - |  |
| `validate_field` | 2 | `crates/discourse-pdf-compliance/src/forms/validation.rs:86` | - |  |
| `execute_calculation` | 2 | `crates/discourse-pdf-compliance/src/forms/validation.rs:106` | - |  |
| `export_xfdf` | 1 | `crates/discourse-pdf-compliance/src/forms/xfdf.rs:11` | - | Export AcroForm field values as an XML XFDF document. |
| `import_xfdf` | 2 | `crates/discourse-pdf-compliance/src/forms/xfdf.rs:32` | - | Import field values from an XML XFDF document into an AcroForm. |
| `parse_xfdf_fields` | 1 | `crates/discourse-pdf-compliance/src/forms/xfdf.rs:48` | - |  |
| `extract_attr` | 2 | `crates/discourse-pdf-compliance/src/forms/xfdf.rs:84` | - |  |
| `xml_escape` | 1 | `crates/discourse-pdf-compliance/src/forms/xfdf.rs:91` | - |  |
| `xml_unescape` | 1 | `crates/discourse-pdf-compliance/src/forms/xfdf.rs:99` | - |  |
| `impl StandardProfile::label` | 1 | `crates/discourse-pdf-compliance/src/preflight/evaluator.rs:18` | - |  |
| `evaluate_preflight` | 2 | `crates/discourse-pdf-compliance/src/preflight/evaluator.rs:64` | - |  |
| `evaluate_pdfa` | 3 | `crates/discourse-pdf-compliance/src/preflight/evaluator.rs:84` | - |  |
| `evaluate_pdfx` | 3 | `crates/discourse-pdf-compliance/src/preflight/evaluator.rs:170` | - |  |
| `impl HiddenCategory::label` | 1 | `crates/discourse-pdf-compliance/src/redact/sanitizer.rs:33` | - |  |
| `sanitize_document` | 2 | `crates/discourse-pdf-compliance/src/redact/sanitizer.rs:74` | - | Sanitizes a document manifest by removing all selected hidden information categories. |
| `luhn_check` | 1 | `crates/discourse-pdf-compliance/src/redact/scanner.rs:25` | - | Luhn algorithm for credit card checksum verification. |
| `scan_credit_cards` | 1 | `crates/discourse-pdf-compliance/src/redact/scanner.rs:46` | - | Scan text for credit card numbers (Visa, MasterCard, Amex, Discover, etc.) with Luhn verification. |
| `scan_ssns` | 1 | `crates/discourse-pdf-compliance/src/redact/scanner.rs:91` | - | Scan text for US Social Security Numbers (format `AAA-GG-SSSS` or 9 continuous digits with valid area). |
| `scan_tax_ids` | 1 | `crates/discourse-pdf-compliance/src/redact/scanner.rs:133` | - | Scan text for Employer Identification Numbers / Tax IDs (`XX-XXXXXXX`). |
| `scan_emails` | 1 | `crates/discourse-pdf-compliance/src/redact/scanner.rs:160` | - | Scan text for email addresses. |
| `is_email_local_char` | 1 | `crates/discourse-pdf-compliance/src/redact/scanner.rs:197` | - |  |
| `is_email_domain_char` | 1 | `crates/discourse-pdf-compliance/src/redact/scanner.rs:201` | - |  |
| `scan_phones` | 1 | `crates/discourse-pdf-compliance/src/redact/scanner.rs:206` | - | Scan text for phone numbers (E.164, North American NANP, and international trunk). |
| `scan_all_pii` | 1 | `crates/discourse-pdf-compliance/src/redact/scanner.rs:256` | - | Scans for all PII patterns concurrently. |
| `apply_semantic_redaction` | 3 | `crates/discourse-pdf-compliance/src/redact/verifier.rs:32` | semantics | Applies irreversible, fail-closed redaction to an in-memory text buffer. |
| `verify_zero_residue` | 2 | `crates/discourse-pdf-compliance/src/redact/verifier.rs:50` | - | Zero-residue verification pass: |
| `sha256_slices` | 1 | `crates/discourse-pdf-compliance/src/sign/digest.rs:6` | - | Computes SHA-256 hash over an array of byte slices (e.g. non-contiguous ByteRange segments). |
| `sha256_compress` | 2 | `crates/discourse-pdf-compliance/src/sign/digest.rs:65` | - |  |
| `hex_encode` | 1 | `crates/discourse-pdf-compliance/src/sign/digest.rs:118` | - |  |
| `impl ByteRange::new` | 4 | `crates/discourse-pdf-compliance/src/sign/pades.rs:19` | - |  |
| `impl ByteRange::validate` | 2 | `crates/discourse-pdf-compliance/src/sign/pades.rs:25` | - | Verifies that the ByteRange covers the file cleanly without overlaps, |
| `impl ByteRange::to_pdf_string` | 1 | `crates/discourse-pdf-compliance/src/sign/pades.rs:50` | - |  |
| `verify_pades_byte_range` | 3 | `crates/discourse-pdf-compliance/src/sign/pades.rs:77` | - | Verifies that a signed document's byte range digest matches the signed content. |
| `verify_rfc3161_timestamp` | 2 | `crates/discourse-pdf-compliance/src/sign/timestamp.rs:25` | - |  |
| `test_a11y_32_rules_and_report` | 0 | `crates/discourse-pdf-compliance/src/tests.rs:26` | - |  |
| `test_redaction_pii_and_zero_residue` | 0 | `crates/discourse-pdf-compliance/src/tests.rs:87` | - |  |
| `test_pades_signatures_and_timestamps` | 0 | `crates/discourse-pdf-compliance/src/tests.rs:132` | - |  |
| `test_forms_validation_flattening_and_xfdf` | 0 | `crates/discourse-pdf-compliance/src/tests.rs:157` | - |  |
| `test_preflight_pdfa_and_pdfx` | 0 | `crates/discourse-pdf-compliance/src/tests.rs:204` | - |  |
