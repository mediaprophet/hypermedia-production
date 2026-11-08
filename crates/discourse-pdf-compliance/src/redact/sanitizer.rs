//! Document sanitization: removal of metadata, comments, hidden layers, scripts, and private data.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HiddenCategory {
    Metadata,
    Attachments,
    Comments,
    FormFields,
    HiddenText,
    HiddenLayers,
    Bookmarks,
    LinksActionsScripts,
    PrivateData,
}

impl HiddenCategory {
    pub const ALL: [HiddenCategory; 9] = [
        HiddenCategory::Metadata,
        HiddenCategory::Attachments,
        HiddenCategory::Comments,
        HiddenCategory::FormFields,
        HiddenCategory::HiddenText,
        HiddenCategory::HiddenLayers,
        HiddenCategory::Bookmarks,
        HiddenCategory::LinksActionsScripts,
        HiddenCategory::PrivateData,
    ];

    pub fn label(self) -> &'static str {
        match self {
            HiddenCategory::Metadata => "Document Metadata & XMP Streams",
            HiddenCategory::Attachments => "Embedded File Attachments",
            HiddenCategory::Comments => "Comments and Markup Annotations",
            HiddenCategory::FormFields => "Interactive Form Fields",
            HiddenCategory::HiddenText => "Hidden and Invisible Text",
            HiddenCategory::HiddenLayers => "Hidden Optional Content Layers",
            HiddenCategory::Bookmarks => "Bookmarks & Outlines",
            HiddenCategory::LinksActionsScripts => "Links, Actions & JavaScript",
            HiddenCategory::PrivateData => "Third-Party Application Private Data",
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SanitizeManifest {
    pub author: Option<String>,
    pub creator: Option<String>,
    pub producer: Option<String>,
    pub xmp_packets: Vec<String>,
    pub attachment_names: Vec<String>,
    pub comment_count: usize,
    pub form_field_count: usize,
    pub hidden_layer_count: usize,
    pub bookmark_count: usize,
    pub script_count: usize,
    pub private_data_entries: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SanitizeReport {
    pub removed_categories: Vec<HiddenCategory>,
    pub stripped_metadata_keys: Vec<String>,
    pub stripped_attachments: usize,
    pub stripped_comments: usize,
    pub flattened_form_fields: usize,
    pub purged_private_data_keys: usize,
}

/// Sanitizes a document manifest by removing all selected hidden information categories.
pub fn sanitize_document(
    manifest: &mut SanitizeManifest,
    categories: &[HiddenCategory],
) -> SanitizeReport {
    let mut report = SanitizeReport::default();

    for &cat in categories {
        report.removed_categories.push(cat);
        match cat {
            HiddenCategory::Metadata => {
                if manifest.author.is_some() {
                    report.stripped_metadata_keys.push("Author".into());
                    manifest.author = None;
                }
                if manifest.creator.is_some() {
                    report.stripped_metadata_keys.push("Creator".into());
                    manifest.creator = None;
                }
                if manifest.producer.is_some() {
                    report.stripped_metadata_keys.push("Producer".into());
                    manifest.producer = None;
                }
                manifest.xmp_packets.clear();
            }
            HiddenCategory::Attachments => {
                report.stripped_attachments += manifest.attachment_names.len();
                manifest.attachment_names.clear();
            }
            HiddenCategory::Comments => {
                report.stripped_comments += manifest.comment_count;
                manifest.comment_count = 0;
            }
            HiddenCategory::FormFields => {
                report.flattened_form_fields += manifest.form_field_count;
                manifest.form_field_count = 0;
            }
            HiddenCategory::HiddenText => {}
            HiddenCategory::HiddenLayers => {
                manifest.hidden_layer_count = 0;
            }
            HiddenCategory::Bookmarks => {
                manifest.bookmark_count = 0;
            }
            HiddenCategory::LinksActionsScripts => {
                manifest.script_count = 0;
            }
            HiddenCategory::PrivateData => {
                report.purged_private_data_keys += manifest.private_data_entries.len();
                manifest.private_data_entries.clear();
            }
        }
    }

    report
}
