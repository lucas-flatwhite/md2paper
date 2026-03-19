pub mod config;
pub mod parser;
pub mod transform;

pub use config::{Config, ConfigBuilder, apply_front_matter};
pub use md2paper_theme::model::Theme;

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
    let preamble = md2paper_theme::inject::generate_preamble(&effective_config.theme, title, author, date);

    // Optional TOC
    let toc_block = if effective_config.toc {
        format!("#outline(depth: {})\n\n", effective_config.toc_depth)
    } else {
        String::new()
    };

    Ok(format!("{preamble}\n{toc_block}{typst_body}"))
}
