use md2paper_theme::inject::generate_preamble;
use md2paper_theme::loader::{load, load_builtin, load_file};

#[test]
fn test_load_builtin_default() {
    let theme = load_builtin("default").unwrap();
    assert_eq!(theme.meta.name, "default");
    assert_eq!(theme.page.paper, "a4");
    assert_eq!(theme.page.columns, 1);
}

#[test]
fn test_load_builtin_academic() {
    let theme = load_builtin("academic").unwrap();
    assert_eq!(theme.meta.name, "academic");
}

#[test]
fn test_load_builtin_minimal() {
    let theme = load_builtin("minimal").unwrap();
    assert_eq!(theme.meta.name, "minimal");
}

#[test]
fn test_load_builtin_newspaper() {
    let theme = load_builtin("newspaper").unwrap();
    assert_eq!(theme.meta.name, "newspaper");
    assert_eq!(theme.page.columns, 2);
}

#[test]
fn test_load_unknown_builtin_fails() {
    assert!(load_builtin("nonexistent").is_err());
}

#[test]
fn test_load_by_name() {
    let theme = load("default").unwrap();
    assert_eq!(theme.meta.name, "default");
}

#[test]
fn test_load_toml_file() {
    let toml_content = r##"
[meta]
name = "test"
description = "Test theme"
version = "0.0.1"

[page]
paper = "letter"
margin_top = "1in"
margin_bottom = "1in"
margin_left = "1in"
margin_right = "1in"
columns = 1

[font]
body_family = "DejaVu Serif"
body_size = "12pt"
body_weight = "regular"
heading_family = "DejaVu Sans"
heading_weight = "bold"
code_family = "DejaVu Sans Mono"
code_size = "10pt"
fallback = []

[spacing]
line_height = 1.5
paragraph_spacing = "1em"
letter_spacing = "0pt"
heading_above = "1em"
heading_below = "0.5em"

[color]
body_text = "#000000"
heading = "#000000"
link = "#0000ee"
code_text = "#333333"
code_background = "#f0f0f0"
blockquote_border = "#aaaaaa"
blockquote_text = "#666666"

[code]
theme = "default"
line_numbers = false
border_radius = "0pt"

[header_footer]
header_left = ""
header_center = ""
header_right = ""
footer_left = ""
footer_center = ""
footer_right = ""
"##;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.toml");
    std::fs::write(&path, toml_content).unwrap();
    let theme = load_file(&path).unwrap();
    assert_eq!(theme.meta.name, "test");
    assert_eq!(theme.page.paper, "letter");
}

#[test]
fn test_generate_preamble_contains_key_elements() {
    let theme = load_builtin("default").unwrap();
    let preamble = generate_preamble(&theme, "My Title", "Author", "2026", false);
    assert!(preamble.contains("#set page("));
    assert!(preamble.contains("#set text("));
    assert!(preamble.contains("#show heading:"));
    assert!(preamble.contains("My Title"));
    assert!(preamble.contains("Author"));
}
