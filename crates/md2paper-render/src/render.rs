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
            let mut combined = msgs.join("\n");
            // Detect likely LaTeX-style math errors and append a hint
            if combined.contains("unknown variable")
                || combined.contains("not found")
            {
                combined.push_str(
                    "\n\nHint: md2paper uses Typst math syntax, not LaTeX.\n\
                     LaTeX → Typst: \\int → integral, \\frac{a}{b} → a/b, \
                     \\nabla → nabla, \\infty → infinity, \\sqrt{x} → sqrt(x)"
                );
            }
            anyhow!("Typst compilation failed:\n{}", combined)
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
