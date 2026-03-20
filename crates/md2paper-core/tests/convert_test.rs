use md2paper_core::{convert, convert_with_config, Config, to_typst};
use serde_json;
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
    let md = include_str!("../../../tests/fixtures/basic.md");
    let pdf = convert(md).expect("basic.md should convert");
    assert!(pdf.starts_with(b"%PDF"));
}

#[test]
fn test_convert_fixtures_table() {
    let md = include_str!("../../../tests/fixtures/gfm_table.md");
    let pdf = convert(md).expect("gfm_table.md should convert");
    assert!(pdf.starts_with(b"%PDF"));
}

#[test]
fn test_all_themes_load_and_render() {
    for name in ["default", "academic", "minimal", "newspaper"] {
        let theme = load_builtin(name).expect(&format!("theme '{}' should load", name));
        let config = Config::builder().theme(theme).build();
        let pdf = convert_with_config("# Test\n\nContent.", &config)
            .expect(&format!("theme '{}' should render", name));
        assert!(pdf.starts_with(b"%PDF"), "theme '{}' should produce valid PDF", name);
    }
}

// ── Fix 4: 누락된 픽스처 테스트 ──────────────────────────────────────────────

#[test]
fn test_convert_fixtures_math() {
    let md = include_str!("../../../tests/fixtures/math.md");
    let pdf = convert(md).expect("math.md should convert to PDF");
    assert!(pdf.starts_with(b"%PDF"));
}

#[test]
fn test_convert_fixtures_cjk() {
    let md = include_str!("../../../tests/fixtures/cjk.md");
    let pdf = convert(md).expect("cjk.md should convert to PDF");
    assert!(pdf.starts_with(b"%PDF"));
}

#[test]
fn test_convert_fixtures_full_featured() {
    let md = include_str!("../../../tests/fixtures/full_featured.md");
    let pdf = convert(md).expect("full_featured.md should convert to PDF");
    assert!(pdf.starts_with(b"%PDF"));
}

// math 파싱이 Typst 수식 문법으로 올바르게 변환되는지 확인
#[test]
fn test_inline_math_to_typst() {
    let config = Config::builder().build();
    let typst = to_typst("$E = mc^2$", &config).unwrap();
    assert!(typst.contains('$'), "inline math must be preserved: got {}", typst);
}

#[test]
fn test_display_math_to_typst() {
    let config = Config::builder().build();
    let typst = to_typst("$$\n\\int x dx\n$$", &config).unwrap();
    assert!(typst.contains('$'), "display math must be preserved: got {}", typst);
}

// ── Fix 1: escape_typst 특수문자 이스케이프 ─────────────────────────────────

#[test]
fn test_tilde_escaped_in_text() {
    // Typst에서 ~ 는 non-breaking space — 본문 텍스트의 ~ 는 \~ 로 이스케이프돼야 한다
    let config = Config::builder().build();
    let typst = to_typst("foo~bar", &config).unwrap();
    assert!(typst.contains("foo\\~bar"), "tilde must be escaped: got {}", typst);
}

#[test]
fn test_backtick_in_text_escaped() {
    // CommonMark: \` in source → literal ` text node
    let config = Config::builder().build();
    let typst = to_typst("foo\\`bar", &config).unwrap();
    assert!(typst.contains("foo\\`bar"), "backtick in text must be escaped: got {}", typst);
}

// ── Fix 5: 코드 신택스 하이라이팅 ────────────────────────────────────────────

#[test]
fn test_code_block_preamble_uses_set_raw() {
    // 폰트/사이즈 설정은 #set raw 로 해야 show rule 안에서 fill 덮어쓰기를 피할 수 있다
    let config = Config::builder().build();
    let typst = to_typst("# Hello", &config).unwrap();
    assert!(typst.contains("#set raw("), "preamble must configure raw blocks via #set raw");
}

#[test]
fn test_code_block_show_rule_does_not_override_fill() {
    // block code show rule의 text() 래퍼에 fill이 없어야 구문 강조 색상이 보존된다.
    // preamble에서 "raw.where(block: true)" 섹션을 분리해 확인한다.
    let config = Config::builder().build();
    let typst = to_typst("# Hello", &config).unwrap();
    // block=true show rule 시작 위치 탐색
    let block_rule_start = typst
        .find("raw.where(block: true)")
        .expect("preamble must contain raw.where(block: true) rule");
    let block_rule_section = &typst[block_rule_start..];
    // 규칙의 닫는 }} 까지 잘라냄 (다음 #show 나 #set 이전까지)
    let block_rule_end = block_rule_section
        .find("\n#")
        .unwrap_or(block_rule_section.len());
    let rule = &block_rule_section[..block_rule_end];
    // block show rule 내부의 text() 에 fill: 이 있으면 안 된다
    assert!(
        !rule.contains(", fill: rgb("),
        "block code show rule must not contain fill override: got {}",
        rule
    );
}

#[test]
fn test_line_numbers_enabled_in_preamble() {
    let mut theme = md2paper_theme::loader::load_builtin("default").unwrap();
    theme.code.line_numbers = true;
    let config = Config::builder().theme(theme).build();
    let typst = to_typst("# Hello", &config).unwrap();
    // line_numbers=true이면 raw.lines 를 순회하는 show rule이 있어야 한다
    assert!(
        typst.contains("it.lines"),
        "line_numbers=true must produce a raw.lines show rule: got {}",
        &typst[..typst.len().min(1000)]
    );
}

// ── Fix 2: 각주(Footnote) 렌더링 ─────────────────────────────────────────────

#[test]
fn test_footnote_renders_actual_content() {
    // FootnoteReference가 정의된 내용을 #footnote[...] 안에 포함해야 한다
    let md = "See this note[^1] for details.\n\n[^1]: This is the footnote content.";
    let config = Config::builder().build();
    let typst = to_typst(md, &config).unwrap();
    assert!(typst.contains("#footnote["), "must emit #footnote[: got {}", typst);
    assert!(
        typst.contains("This is the footnote content"),
        "footnote body must appear inside #footnote[]: got {}",
        typst
    );
}

#[test]
fn test_footnote_converts_to_pdf() {
    let md = "Text with note[^a].\n\n[^a]: Footnote body.";
    let pdf = convert(md).expect("footnote doc should convert to PDF");
    assert!(pdf.starts_with(b"%PDF"));
}

// ── Fix 6: HTTP 이미지 다운로드 ───────────────────────────────────────────────

#[test]
fn test_http_image_url_in_typst_markup() {
    // HTTP URL 이미지가 Typst 마크업으로 변환될 때 URL이 그대로 유지돼야 한다
    let config = Config::builder().build();
    let typst = to_typst("![alt](https://example.com/image.png)", &config).unwrap();
    assert!(
        typst.contains("https://example.com/image.png"),
        "HTTP image URL must be preserved in Typst output: got {}",
        typst
    );
}

// 실제 네트워크가 필요한 테스트 — `cargo test -- --include-ignored` 로 실행
#[test]
#[ignore = "requires network access"]
fn test_http_image_downloads_and_converts_to_pdf() {
    // SVG는 바이트가 작고 안정적인 공개 URL을 사용
    let md = "# Test\n\n![Rust logo](https://www.rust-lang.org/static/images/rust-logo-blk.svg)\n";
    let pdf = convert(md).expect("HTTP image should download and render to PDF");
    assert!(pdf.starts_with(b"%PDF"));
}

// ── Fix 3a: --dump-ast ────────────────────────────────────────────────────────

#[test]
fn test_ast_dump_contains_node_types() {
    let dump = md2paper_core::ast_as_debug_string("# Hello\n\nWorld");
    assert!(!dump.is_empty(), "dump must not be empty");
    assert!(dump.contains("Heading") || dump.contains("heading"), "must mention heading node");
}

#[test]
fn test_ast_dump_is_valid_json() {
    let dump = md2paper_core::ast_as_debug_string("# Hello\n\nWorld");
    let parsed: serde_json::Value =
        serde_json::from_str(&dump).expect("ast dump must be valid JSON");
    assert_eq!(parsed["type"], "Document", "root must be Document");
    let children = parsed["children"].as_array().expect("Document must have children");
    assert!(!children.is_empty(), "must have at least one child");
    let first = &children[0];
    assert_eq!(first["type"], "Heading", "first child of '# Hello' must be Heading");
}

// ── Fix 3b: --cover 표지 페이지 ───────────────────────────────────────────────

#[test]
fn test_cover_adds_pagebreak() {
    let config = Config::builder().cover(true).title("My Paper Title").build();
    let typst = to_typst("# Content", &config).unwrap();
    assert!(typst.contains("#pagebreak()"), "cover must include pagebreak: got {}", typst);
    assert!(typst.contains("My Paper Title"), "cover must include title: got {}", typst);
}

#[test]
fn test_no_cover_no_pagebreak() {
    let config = Config::builder().cover(false).title("My Paper Title").build();
    let typst = to_typst("# Content", &config).unwrap();
    assert!(!typst.contains("#pagebreak()"), "without cover there should be no pagebreak: got {}", typst);
}

#[test]
fn test_cover_page_converts_to_pdf() {
    let config = Config::builder()
        .cover(true)
        .title("Test Cover")
        .author("Test Author")
        .date("2026-03-20")
        .build();
    let pdf = convert_with_config("# Content\n\nBody.", &config).expect("cover doc should render");
    assert!(pdf.starts_with(b"%PDF"));
}
