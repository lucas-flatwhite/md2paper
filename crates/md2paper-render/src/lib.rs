mod render;
mod world;

use anyhow::Result;
use std::path::Path;

/// Render Typst markup source to PDF bytes.
///
/// `base_dir` is used to resolve relative asset paths (images, etc.).
/// Pass `Path::new(".")` if no file-relative assets are expected.
pub fn render_to_pdf(typst_src: &str) -> Result<Vec<u8>> {
    render::render(typst_src, Path::new("."))
}

/// Render Typst markup source to PDF bytes with a custom base directory.
pub fn render_to_pdf_with_base(typst_src: &str, base_dir: &Path) -> Result<Vec<u8>> {
    render::render(typst_src, base_dir)
}
