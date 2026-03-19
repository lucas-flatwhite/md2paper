use comrak::{Arena, ComrakOptions, parse_document, nodes::AstNode};
use serde::Deserialize;

/// Parsed front matter extracted from a Markdown document.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct FrontMatter {
    pub title: Option<String>,
    pub author: Option<String>,
    pub date: Option<String>,
    pub theme: Option<String>,
    pub font: Option<md2paper_theme::model::FontOverride>,
    pub spacing: Option<md2paper_theme::model::SpacingOverride>,
    pub page: Option<md2paper_theme::model::PageOverride>,
}

/// Parse front matter from Markdown source and return (stripped_source, front_matter).
pub fn extract_front_matter(source: &str) -> (String, FrontMatter) {
    if let Some(rest) = source.strip_prefix("---\n") {
        if let Some(end) = rest.find("\n---\n") {
            let yaml_src = &rest[..end];
            let after = &rest[end + 5..]; // skip "\n---\n"
            let fm: FrontMatter = serde_yaml::from_str(yaml_src).unwrap_or_default();
            return (after.to_string(), fm);
        }
        // Also handle "---\r\n" endings
        if let Some(end) = rest.find("\n---") {
            let next = end + 4;
            if next >= rest.len() || rest.as_bytes()[next] == b'\n' || rest.as_bytes()[next] == b'\r' {
                let yaml_src = &rest[..end];
                let after = if next < rest.len() { &rest[next + 1..] } else { "" };
                let fm: FrontMatter = serde_yaml::from_str(yaml_src).unwrap_or_default();
                return (after.to_string(), fm);
            }
        }
    }
    (source.to_string(), FrontMatter::default())
}

/// Build comrak options for GFM + extensions.
pub fn make_options() -> ComrakOptions<'static> {
    let mut opts = ComrakOptions::default();
    opts.extension.strikethrough = true;
    opts.extension.table = true;
    opts.extension.autolink = true;
    opts.extension.tasklist = true;
    opts.extension.footnotes = true;
    opts.extension.math_dollars = true;
    opts.extension.front_matter_delimiter = None; // handled manually
    opts.parse.smart = false;
    opts
}

pub fn parse_markdown<'a>(arena: &'a Arena<AstNode<'a>>, source: &str) -> &'a AstNode<'a> {
    let opts = make_options();
    parse_document(arena, source, &opts)
}
