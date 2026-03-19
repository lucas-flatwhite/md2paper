use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Theme {
    pub meta: MetaSection,
    pub page: PageSection,
    pub font: FontSection,
    pub spacing: SpacingSection,
    pub color: ColorSection,
    pub code: CodeSection,
    pub header_footer: HeaderFooterSection,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MetaSection {
    pub name: String,
    pub description: String,
    pub version: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PageSection {
    pub paper: String,
    pub margin_top: String,
    pub margin_bottom: String,
    pub margin_left: String,
    pub margin_right: String,
    pub columns: u8,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FontSection {
    pub body_family: String,
    pub body_size: String,
    pub body_weight: String,
    pub heading_family: String,
    pub heading_weight: String,
    pub code_family: String,
    pub code_size: String,
    pub fallback: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpacingSection {
    pub line_height: f64,
    pub paragraph_spacing: String,
    pub letter_spacing: String,
    pub heading_above: String,
    pub heading_below: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ColorSection {
    pub body_text: String,
    pub heading: String,
    pub link: String,
    pub code_text: String,
    pub code_background: String,
    pub blockquote_border: String,
    pub blockquote_text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodeSection {
    pub theme: String,
    pub line_numbers: bool,
    pub border_radius: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HeaderFooterSection {
    pub header_left: String,
    pub header_center: String,
    pub header_right: String,
    pub footer_left: String,
    pub footer_center: String,
    pub footer_right: String,
}

/// Per-document overrides parsed from front matter
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThemeOverride {
    pub font: Option<FontOverride>,
    pub spacing: Option<SpacingOverride>,
    pub page: Option<PageOverride>,
    pub color: Option<ColorOverride>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FontOverride {
    pub body_family: Option<String>,
    pub body_size: Option<String>,
    pub code_family: Option<String>,
    pub code_size: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SpacingOverride {
    pub line_height: Option<f64>,
    pub paragraph_spacing: Option<String>,
    pub letter_spacing: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PageOverride {
    pub paper: Option<String>,
    pub margin_top: Option<String>,
    pub margin_bottom: Option<String>,
    pub margin_left: Option<String>,
    pub margin_right: Option<String>,
    pub columns: Option<u8>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColorOverride {
    pub body_text: Option<String>,
    pub heading: Option<String>,
    pub link: Option<String>,
}
