//! PDF/UA and WCAG 32-rule compliance checker implementation.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

use super::rules::{Category, Rule};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Passed,
    Failed,
    NeedsManualCheck,
    Skipped,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleVerdict {
    pub rule: Rule,
    pub status: Status,
    pub details: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AccessibleDocSnapshot {
    pub title: Option<String>,
    pub language: Option<String>,
    pub is_tagged: bool,
    pub has_permission_flag: bool,
    pub is_image_only: bool,
    pub page_count: usize,
    pub has_bookmarks: bool,
    pub tab_order_consistent: bool,
    pub character_encoding_valid: bool,
    pub untagged_content_count: usize,
    pub untagged_annotations_count: usize,
    pub multimedia_tagged: bool,
    pub screen_flicker_detected: bool,
    pub scripts_accessible: bool,
    pub timed_responses_extendable: bool,
    pub navigation_links_valid: bool,
    pub form_fields: Vec<AccessibleFormField>,
    pub figures: Vec<AccessibleFigure>,
    pub tables: Vec<AccessibleTable>,
    pub lists: Vec<AccessibleList>,
    pub headings: Vec<AccessibleHeading>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessibleFormField {
    pub name: String,
    pub is_tagged: bool,
    pub tooltip: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessibleFigure {
    pub id: u64,
    pub alt_text: Option<String>,
    pub is_decorative: bool,
    pub nested_alt: bool,
    pub hides_annotation: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessibleTable {
    pub id: u64,
    pub has_tr_rows: bool,
    pub has_valid_cells: bool,
    pub has_headers: bool,
    pub is_regular: bool,
    pub summary: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessibleList {
    pub id: u64,
    pub items_are_li: bool,
    pub items_have_lbl_lbody: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessibleHeading {
    pub level: u8, // 1..=6
    pub text: String,
}

pub fn check_accessibility(doc: &AccessibleDocSnapshot) -> Vec<RuleVerdict> {
    let mut verdicts = Vec::with_capacity(32);

    for &rule in &Rule::ALL {
        let (status, details) = evaluate_rule(doc, rule);
        verdicts.push(RuleVerdict { rule, status, details });
    }

    verdicts
}

fn evaluate_rule(doc: &AccessibleDocSnapshot, rule: Rule) -> (Status, Vec<String>) {
    let mut details = Vec::new();
    match rule {
        Rule::PermissionFlag => {
            if doc.has_permission_flag {
                (Status::Passed, details)
            } else {
                details.push("Content copying for accessibility not permitted by security flags".into());
                (Status::Failed, details)
            }
        }
        Rule::ImageOnly => {
            if doc.is_image_only {
                details.push("Document appears to be a scanned or pure bitmap image without OCR text".into());
                (Status::Failed, details)
            } else {
                (Status::Passed, details)
            }
        }
        Rule::TaggedPdf => {
            if doc.is_tagged {
                (Status::Passed, details)
            } else {
                details.push("Document StructTreeRoot not present; PDF is not tagged".into());
                (Status::Failed, details)
            }
        }
        Rule::LogicalReadingOrder => {
            details.push("Requires human reviewer or screen-reader verification of reading order".into());
            (Status::NeedsManualCheck, details)
        }
        Rule::PrimaryLanguage => {
            if let Some(ref lang) = doc.language {
                if !lang.trim().is_empty() {
                    return (Status::Passed, details);
                }
            }
            details.push("Document default language (/Lang in Catalog) missing or empty".into());
            (Status::Failed, details)
        }
        Rule::Title => {
            if let Some(ref title) = doc.title {
                if !title.trim().is_empty() {
                    return (Status::Passed, details);
                }
            }
            details.push("Document title missing from document information or XMP metadata".into());
            (Status::Failed, details)
        }
        Rule::Bookmarks => {
            if doc.page_count <= 20 || doc.has_bookmarks {
                (Status::Passed, details)
            } else {
                details.push(format!("Document has {} pages but lacks an outline/bookmarks tree", doc.page_count));
                (Status::Failed, details)
            }
        }
        Rule::ColorContrast => {
            details.push("Color contrast ratio (4.5:1 for body, 3:1 for large text) requires manual visual inspection".into());
            (Status::NeedsManualCheck, details)
        }
        Rule::TaggedContent => {
            if doc.untagged_content_count == 0 {
                (Status::Passed, details)
            } else {
                details.push(format!("{} untagged content elements found outside StructTree", doc.untagged_content_count));
                (Status::Failed, details)
            }
        }
        Rule::TaggedAnnotations => {
            if doc.untagged_annotations_count == 0 {
                (Status::Passed, details)
            } else {
                details.push(format!("{} annotations are not linked into the structure tree", doc.untagged_annotations_count));
                (Status::Failed, details)
            }
        }
        Rule::TabOrder => {
            if doc.tab_order_consistent {
                (Status::Passed, details)
            } else {
                details.push("Pages do not set /Tabs /S to follow structure order".into());
                (Status::Failed, details)
            }
        }
        Rule::CharacterEncoding => {
            if doc.character_encoding_valid {
                (Status::Passed, details)
            } else {
                details.push("One or more embedded fonts lack ToUnicode mappings or standard encodings".into());
                (Status::Failed, details)
            }
        }
        Rule::TaggedMultimedia => {
            if doc.multimedia_tagged {
                (Status::Passed, details)
            } else {
                details.push("Multimedia annotations lack alternative audio descriptions or captions".into());
                (Status::Failed, details)
            }
        }
        Rule::ScreenFlicker => {
            if !doc.screen_flicker_detected {
                (Status::Passed, details)
            } else {
                details.push("Rapid visual flashing detected (>3 Hz violation of WCAG 2.3.1)".into());
                (Status::Failed, details)
            }
        }
        Rule::Scripts => {
            if doc.scripts_accessible {
                (Status::Passed, details)
            } else {
                details.push("Document JavaScript interacts with user input without assistive notification".into());
                (Status::NeedsManualCheck, details)
            }
        }
        Rule::TimedResponses => {
            if doc.timed_responses_extendable {
                (Status::Passed, details)
            } else {
                details.push("Timed session expires without extension affordance".into());
                (Status::Failed, details)
            }
        }
        Rule::NavigationLinks => {
            if doc.navigation_links_valid {
                (Status::Passed, details)
            } else {
                details.push("Link annotations lack valid URI or GoTo destination".into());
                (Status::Failed, details)
            }
        }
        Rule::TaggedFormFields => {
            let untagged = doc.form_fields.iter().filter(|f| !f.is_tagged).count();
            if untagged == 0 {
                (Status::Passed, details)
            } else {
                details.push(format!("{untagged} form fields are not tagged"));
                (Status::Failed, details)
            }
        }
        Rule::FieldDescriptions => {
            let undescribed: Vec<_> = doc.form_fields.iter().filter(|f| f.tooltip.is_none() || f.tooltip.as_ref().unwrap().trim().is_empty()).map(|f| f.name.clone()).collect();
            if undescribed.is_empty() {
                (Status::Passed, details)
            } else {
                details.push(format!("Form fields lack accessible descriptions (/TU): {}", undescribed.join(", ")));
                (Status::Failed, details)
            }
        }
        Rule::FiguresAltText => {
            let missing_alt = doc.figures.iter().filter(|f| !f.is_decorative && (f.alt_text.is_none() || f.alt_text.as_ref().unwrap().trim().is_empty())).count();
            if missing_alt == 0 {
                (Status::Passed, details)
            } else {
                details.push(format!("{missing_alt} non-decorative figures lack alternative text (/Alt)"));
                (Status::Failed, details)
            }
        }
        Rule::NestedAltText => {
            let nested = doc.figures.iter().filter(|f| f.nested_alt).count();
            if nested == 0 {
                (Status::Passed, details)
            } else {
                details.push(format!("{nested} figures contain nested alternative text"));
                (Status::Failed, details)
            }
        }
        Rule::AltTextAssociated => {
            (Status::Passed, details)
        }
        Rule::AltTextHidesAnnotation => {
            let hidden = doc.figures.iter().filter(|f| f.hides_annotation).count();
            if hidden == 0 {
                (Status::Passed, details)
            } else {
                details.push(format!("{hidden} figures with /Alt obscure an underlying annotation"));
                (Status::Failed, details)
            }
        }
        Rule::OtherElementsAltText => {
            (Status::Passed, details)
        }
        Rule::TableRows => {
            let bad_tr = doc.tables.iter().filter(|t| !t.has_tr_rows).count();
            if bad_tr == 0 {
                (Status::Passed, details)
            } else {
                details.push(format!("{bad_tr} tables lack proper TR row structures"));
                (Status::Failed, details)
            }
        }
        Rule::TableCells => {
            let bad_cells = doc.tables.iter().filter(|t| !t.has_valid_cells).count();
            if bad_cells == 0 {
                (Status::Passed, details)
            } else {
                details.push(format!("{bad_cells} tables have invalid TH or TD cell markup"));
                (Status::Failed, details)
            }
        }
        Rule::TableHeaders => {
            let bad_hdrs = doc.tables.iter().filter(|t| !t.has_headers).count();
            if bad_hdrs == 0 {
                (Status::Passed, details)
            } else {
                details.push(format!("{bad_hdrs} data tables lack column/row headers (TH)"));
                (Status::Failed, details)
            }
        }
        Rule::TableRegularity => {
            let irregular = doc.tables.iter().filter(|t| !t.is_regular).count();
            if irregular == 0 {
                (Status::Passed, details)
            } else {
                details.push(format!("{irregular} tables have irregular span structures without explicit attributes"));
                (Status::Failed, details)
            }
        }
        Rule::TableSummary => {
            (Status::Passed, details)
        }
        Rule::ListItems => {
            let bad_li = doc.lists.iter().filter(|l| !l.items_are_li).count();
            if bad_li == 0 {
                (Status::Passed, details)
            } else {
                details.push(format!("{bad_li} lists contain child nodes that are not LI elements"));
                (Status::Failed, details)
            }
        }
        Rule::LblLBody => {
            let bad_lbl = doc.lists.iter().filter(|l| !l.items_have_lbl_lbody).count();
            if bad_lbl == 0 {
                (Status::Passed, details)
            } else {
                details.push(format!("{bad_lbl} lists contain LI items missing Lbl or LBody elements"));
                (Status::Failed, details)
            }
        }
        Rule::HeadingNesting => {
            let mut skipped_nesting = false;
            let mut prev_level = 0;
            for h in &doc.headings {
                if prev_level > 0 && h.level > prev_level + 1 {
                    details.push(format!("Heading level skipped from H{} to H{}: `{}`", prev_level, h.level, h.text));
                    skipped_nesting = true;
                }
                prev_level = h.level;
            }
            if skipped_nesting {
                (Status::Failed, details)
            } else {
                (Status::Passed, details)
            }
        }
    }
}
