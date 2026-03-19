use md2paper_core::{convert, convert_with_config, Config, to_typst};
use md2paper_theme::loader::load_builtin;

#[test]
fn test_simple_convert() {
    let pdf = convert("# Hello\n\nWorld").expect("convert should succeed");
    assert!(!pdf.is_empty(), "PDF should not be empty");
    assert!(pdf.starts_with(b"%PDF"), "Output should be a PDF");
}

#[test]
fn test_convert_with_default_theme() {
    let theme = load_builtin("default").unwrap();
    let config = Config::builder().theme(theme).build();
    let pdf = convert_with_config("# Test\n\nContent.", &config).expect("should succeed");
    assert!(pdf.starts_with(b"%PDF"));
}

#[test]
fn test_to_typst_heading() {
    let config = Config::builder().build();
    let typst = to_typst("# Hello\n\n## World", &config).unwrap();
    assert!(typst.contains("= Hello"));
    assert!(typst.contains("== World"));
}

#[test]
fn test_to_typst_bold_italic() {
    let config = Config::builder().build();
    let typst = to_typst("**bold** and *italic*", &config).unwrap();
    assert!(typst.contains("*bold*"));
    assert!(typst.contains("_italic_"));
}

#[test]
fn test_to_typst_code_block() {
    let config = Config::builder().build();
    let typst = to_typst("```rust\nfn main() {}\n```", &config).unwrap();
    assert!(typst.contains("```rust"));
}

#[test]
fn test_front_matter_extraction() {
    let md = "---\ntitle: My Title\nauthor: Test Author\n---\n\n# Body";
    let config = Config::builder().build();
    let typst = to_typst(md, &config).unwrap();
    assert!(typst.contains("My Title"));
    assert!(typst.contains("Test Author"));
}

#[test]
fn test_toc_generation() {
    let config = Config::builder().toc(true).toc_depth(2).build();
    let typst = to_typst("# Heading 1\n\n## Heading 2", &config).unwrap();
    assert!(typst.contains("#outline"));
}

#[test]
fn test_convert_fixtures_basic() {
    let md = include_str!("../fixtures/basic.md");
    let pdf = convert(md).expect("basic.md should convert");
    assert!(pdf.starts_with(b"%PDF"));
}

#[test]
fn test_convert_fixtures_table() {
    let md = include_str!("../fixtures/gfm_table.md");
    let pdf = convert(md).expect("gfm_table.md should convert");
    assert!(pdf.starts_with(b"%PDF"));
}

#[test]
fn test_all_themes_load() {
    for name in ["default", "academic", "minimal", "newspaper"] {
        let theme = load_builtin(name).expect(&format!("theme '{}' should load", name));
        let config = Config::builder().theme(theme).build();
        let pdf = convert_with_config("# Test\n\nContent.", &config)
            .expect(&format!("theme '{}' should render", name));
        assert!(pdf.starts_with(b"%PDF"), "theme '{}' should produce valid PDF", name);
    }
}
