//! PDF/UA (ISO 14289) and WCAG 32-rule accessibility compliance definitions.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Category {
    Document,
    PageContent,
    Forms,
    AlternateText,
    Tables,
    Lists,
    Headings,
}

impl Category {
    pub const ALL: [Category; 7] = [
        Category::Document,
        Category::PageContent,
        Category::Forms,
        Category::AlternateText,
        Category::Tables,
        Category::Lists,
        Category::Headings,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Category::Document => "Document",
            Category::PageContent => "Page Content",
            Category::Forms => "Forms",
            Category::AlternateText => "Alternate Text",
            Category::Tables => "Tables",
            Category::Lists => "Lists",
            Category::Headings => "Headings",
        }
    }
}

/// The 32 normative rules of PDF/UA and WCAG full accessibility evaluation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Rule {
    PermissionFlag,
    ImageOnly,
    TaggedPdf,
    LogicalReadingOrder,
    PrimaryLanguage,
    Title,
    Bookmarks,
    ColorContrast,
    TaggedContent,
    TaggedAnnotations,
    TabOrder,
    CharacterEncoding,
    TaggedMultimedia,
    ScreenFlicker,
    Scripts,
    TimedResponses,
    NavigationLinks,
    TaggedFormFields,
    FieldDescriptions,
    FiguresAltText,
    NestedAltText,
    AltTextAssociated,
    AltTextHidesAnnotation,
    OtherElementsAltText,
    TableRows,
    TableCells,
    TableHeaders,
    TableRegularity,
    TableSummary,
    ListItems,
    LblLBody,
    HeadingNesting,
}

impl Rule {
    pub const ALL: [Rule; 32] = [
        Rule::PermissionFlag,
        Rule::ImageOnly,
        Rule::TaggedPdf,
        Rule::LogicalReadingOrder,
        Rule::PrimaryLanguage,
        Rule::Title,
        Rule::Bookmarks,
        Rule::ColorContrast,
        Rule::TaggedContent,
        Rule::TaggedAnnotations,
        Rule::TabOrder,
        Rule::CharacterEncoding,
        Rule::TaggedMultimedia,
        Rule::ScreenFlicker,
        Rule::Scripts,
        Rule::TimedResponses,
        Rule::NavigationLinks,
        Rule::TaggedFormFields,
        Rule::FieldDescriptions,
        Rule::FiguresAltText,
        Rule::NestedAltText,
        Rule::AltTextAssociated,
        Rule::AltTextHidesAnnotation,
        Rule::OtherElementsAltText,
        Rule::TableRows,
        Rule::TableCells,
        Rule::TableHeaders,
        Rule::TableRegularity,
        Rule::TableSummary,
        Rule::ListItems,
        Rule::LblLBody,
        Rule::HeadingNesting,
    ];

    pub fn category(self) -> Category {
        match self {
            Rule::PermissionFlag
            | Rule::ImageOnly
            | Rule::TaggedPdf
            | Rule::LogicalReadingOrder
            | Rule::PrimaryLanguage
            | Rule::Title
            | Rule::Bookmarks
            | Rule::ColorContrast => Category::Document,

            Rule::TaggedContent
            | Rule::TaggedAnnotations
            | Rule::TabOrder
            | Rule::CharacterEncoding
            | Rule::TaggedMultimedia
            | Rule::ScreenFlicker
            | Rule::Scripts
            | Rule::TimedResponses
            | Rule::NavigationLinks => Category::PageContent,

            Rule::TaggedFormFields | Rule::FieldDescriptions => Category::Forms,

            Rule::FiguresAltText
            | Rule::NestedAltText
            | Rule::AltTextAssociated
            | Rule::AltTextHidesAnnotation
            | Rule::OtherElementsAltText => Category::AlternateText,

            Rule::TableRows
            | Rule::TableCells
            | Rule::TableHeaders
            | Rule::TableRegularity
            | Rule::TableSummary => Category::Tables,

            Rule::ListItems | Rule::LblLBody => Category::Lists,

            Rule::HeadingNesting => Category::Headings,
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Rule::PermissionFlag => "Accessibility permission flag must be set",
            Rule::ImageOnly => "Document is not image-only PDF",
            Rule::TaggedPdf => "Document is tagged PDF",
            Rule::LogicalReadingOrder => "Document has logical reading order",
            Rule::PrimaryLanguage => "Document specifies default primary language",
            Rule::Title => "Document has non-empty title in metadata",
            Rule::Bookmarks => "Bookmarks present in long documents",
            Rule::ColorContrast => "Appropriate color contrast",
            Rule::TaggedContent => "All visible content is tagged or marked artifact",
            Rule::TaggedAnnotations => "Annotations are tagged",
            Rule::TabOrder => "Consistent tab order matches structure tree",
            Rule::CharacterEncoding => "Reliable character encoding / ToUnicode maps",
            Rule::TaggedMultimedia => "Tagged multimedia content",
            Rule::ScreenFlicker => "No screen flicker or rapid blinking",
            Rule::Scripts => "Scripts do not interfere with assistive technology",
            Rule::TimedResponses => "Timed responses allow extension",
            Rule::NavigationLinks => "Navigation links are tagged and functional",
            Rule::TaggedFormFields => "All form fields are tagged",
            Rule::FieldDescriptions => "Form fields have accessible descriptions",
            Rule::FiguresAltText => "Figures have alternate text",
            Rule::NestedAltText => "No nested alternate text",
            Rule::AltTextAssociated => "Alternate text is associated with content",
            Rule::AltTextHidesAnnotation => "Alternate text does not hide annotation",
            Rule::OtherElementsAltText => "Other non-text elements have alternate text",
            Rule::TableRows => "Table rows are TR elements",
            Rule::TableCells => "Table cells are TH or TD elements",
            Rule::TableHeaders => "Headers present for data tables",
            Rule::TableRegularity => "Table has uniform row and column spans",
            Rule::TableSummary => "Table summary present where required",
            Rule::ListItems => "List items are LI elements",
            Rule::LblLBody => "List item contains Lbl and LBody elements",
            Rule::HeadingNesting => "Appropriate heading levels nesting (H1..H6)",
        }
    }
}
