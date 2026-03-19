use anyhow::{Result, anyhow};
use std::path::Path;
use typst::layout::PagedDocument;
use typst_pdf::{PdfOptions, pdf};

use crate::world::MemWorld;

/// Compile Typst markup to PDF bytes.
///
/// `base_dir` is used to resolve relative file/image paths in the Typst source.
pub fn render(typst_src: &str, base_dir: &Path) -> Result<Vec<u8>> {
    let world = MemWorld::new(typst_src, base_dir);

    let result = typst::compile::<PagedDocument>(&world);

    // Collect warnings (non-fatal)
    for warning in &result.warnings {
        eprintln!("typst warning: {}", warning.message);
    }

    let document = result
        .output
        .map_err(|errors| {
            let msgs: Vec<String> = errors
                .iter()
                .map(|e| format!("{}", e.message))
                .collect();
            anyhow!("Typst compilation failed:\n{}", msgs.join("\n"))
        })?;

    let options = PdfOptions::default();
    pdf(&document, &options)
        .map_err(|errors| {
            let msgs: Vec<String> = errors
                .iter()
                .map(|e| format!("{}", e.message))
                .collect();
            anyhow!("PDF export failed:\n{}", msgs.join("\n"))
        })
}
