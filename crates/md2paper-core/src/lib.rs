pub mod config;
pub mod parser;
pub mod transform;

pub use config::{Config, ConfigBuilder, apply_front_matter};
pub use md2paper_theme::model::Theme;

/// Return the Markdown AST as a pretty-printed JSON string.
/// Useful for the `--dump-ast` CLI flag.
pub fn ast_as_debug_string(markdown: &str) -> String {
    let (body, _fm) = parser::extract_front_matter(markdown);
    let arena = comrak::Arena::new();
    let root = parser::parse_markdown(&arena, &body);
    let json = ast_node_to_json(root);
    serde_json::to_string_pretty(&json).unwrap_or_else(|_| "{}".to_string())
}

fn ast_node_to_json<'a>(node: &'a comrak::nodes::AstNode<'a>) -> serde_json::Value {
    use comrak::nodes::NodeValue;
    let data = node.data.borrow();
    let type_name = match &data.value {
        NodeValue::Document => "Document",
        NodeValue::Heading(_) => "Heading",
        NodeValue::Paragraph => "Paragraph",
        NodeValue::Text(_) => "Text",
        NodeValue::Strong => "Strong",
        NodeValue::Emph => "Emph",
        NodeValue::Code(_) => "Code",
        NodeValue::CodeBlock(_) => "CodeBlock",
        NodeValue::BlockQuote => "BlockQuote",
        NodeValue::List(_) => "List",
        NodeValue::Item(_) => "Item",
        NodeValue::Link(_) => "Link",
        NodeValue::Image(_) => "Image",
        NodeValue::Table(_) => "Table",
        NodeValue::TableRow(_) => "TableRow",
        NodeValue::TableCell => "TableCell",
        NodeValue::ThematicBreak => "ThematicBreak",
        NodeValue::SoftBreak => "SoftBreak",
        NodeValue::LineBreak => "LineBreak",
        NodeValue::HtmlBlock(_) => "HtmlBlock",
        NodeValue::HtmlInline(_) => "HtmlInline",
        NodeValue::FootnoteDefinition(_) => "FootnoteDefinition",
        NodeValue::FootnoteReference(_) => "FootnoteReference",
        NodeValue::Strikethrough => "Strikethrough",
        NodeValue::Math(_) => "Math",
        NodeValue::TaskItem(_) => "TaskItem",
        _ => "Unknown",
    };
    let extra: Option<serde_json::Value> = match &data.value {
        NodeValue::Heading(h) => Some(serde_json::json!({ "level": h.level })),
        NodeValue::Text(t) => Some(serde_json::json!({ "value": t })),
        NodeValue::Code(c) => Some(serde_json::json!({ "literal": c.literal })),
        NodeValue::CodeBlock(cb) => Some(serde_json::json!({
            "info": cb.info, "literal": cb.literal
        })),
        NodeValue::Link(l) => Some(serde_json::json!({ "url": l.url })),
        NodeValue::Image(i) => Some(serde_json::json!({ "url": i.url })),
        NodeValue::Math(m) => Some(serde_json::json!({
            "literal": m.literal, "display": m.dollar_math
        })),
        _ => None,
    };
    drop(data);
    let children: Vec<serde_json::Value> = node.children()
        .map(|c| ast_node_to_json(c))
        .collect();
    let mut obj = serde_json::json!({ "type": type_name });
    if let Some(e) = extra { obj["data"] = e; }
    if !children.is_empty() { obj["children"] = serde_json::Value::Array(children); }
    obj
}

use anyhow::Result;
use comrak::Arena;

/// Convert Markdown source to PDF bytes using the default theme.
pub fn convert(markdown: &str) -> Result<Vec<u8>> {
    let config = Config::builder().build();
    convert_with_config(markdown, &config)
}

/// Convert Markdown source to PDF bytes with explicit configuration.
pub fn convert_with_config(markdown: &str, config: &Config) -> Result<Vec<u8>> {
    let typst_src = to_typst(markdown, config)?;
    md2paper_render::render_to_pdf(&typst_src)
}

/// Convert Markdown source to Typst markup string.
pub fn to_typst(markdown: &str, config: &Config) -> Result<String> {
    let (body, fm) = parser::extract_front_matter(markdown);

    // Resolve theme from front matter if provided
    let mut effective_config = config.clone();
    apply_front_matter(&mut effective_config, &fm);

    // Parse Markdown
    let arena = Arena::new();
    let root = parser::parse_markdown(&arena, &body);

    if effective_config.dump_ast {
        // Not printing here; caller handles this
    }

    // Transform AST → Typst body
    let typst_body = transform::ast_to_typst(root);

    // Generate preamble from theme
    let title = effective_config.title.as_deref().unwrap_or("");
    let author = effective_config.author.as_deref().unwrap_or("");
    let date = effective_config.date.as_deref().unwrap_or("");
    let preamble = md2paper_theme::inject::generate_preamble(&effective_config.theme, title, author, date, effective_config.cover);

    // Optional TOC
    let toc_block = if effective_config.toc {
        format!("#outline(depth: {})\n\n", effective_config.toc_depth)
    } else {
        String::new()
    };

    Ok(format!("{preamble}\n{toc_block}{typst_body}"))
}
