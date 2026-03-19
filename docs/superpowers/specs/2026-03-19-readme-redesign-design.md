---
title: md2paper README Redesign
date: 2026-03-19
status: approved
---

# md2paper README Redesign — Design Spec

## 목표

현재 2줄짜리 README를 사용자(설치·사용)와 기여자(비전·구조) 모두를 만족시키는 풍성한 영문 README로 교체하고, 동일 구조의 한국어 `README.ko.md`를 추가한다.

---

## 언어 전략

- 기본 언어: 영문 (`README.md`)
- 한국어 번역: `README.ko.md` (동일 구조)
- 두 파일 최상단에 언어 선택 배너를 `<div align="right">` 로 배치
- 현재 언어는 링크 없는 텍스트, 다른 언어는 링크로 표시 (아래 각 파일의 배너 HTML 참조)

---

## README.md 섹션 구성

### 1. 언어 선택 배너
`<div align="right">` 로 오른쪽 정렬, 영어/한국어 링크.

### 2. 제목 + 태그라인 + 배지
`<div align="center">` 블록 안에:
- `<h1>md2paper</h1>`
- 태그라인: *Turn your Markdown into beautifully typeset paper.*
- 배지 4개 (한 줄, 모두 shields.io 기반):
  - crates.io 버전: `https://img.shields.io/crates/v/md2paper`
  - License MIT: `https://img.shields.io/crates/l/md2paper`
  - Build Status: `https://img.shields.io/github/actions/workflow/status/lucas-flatwhite/md2paper/ci.yml`
  - Rust Edition: `https://img.shields.io/badge/rust-2021-orange`
- 데모 이미지 placeholder: `> 📸 Screenshot coming soon`
  (나중에 실제 GIF/PNG로 교체 가능한 자리)

### 3. 핵심 가치 한 줄
```
✨ Beautiful by default · ⚡ Lightning fast · 📦 Zero dependency · 🎨 Themeable
```

### 4. Features
아이콘 + 한 줄 설명 불릿 리스트:
- 🖋 Full GFM Markdown — tables, footnotes, task lists, strikethrough
- ➗ Math rendering — `$inline$` and `$$display$$` (KaTeX-compatible syntax via comrak)
- ⚡ Typst-powered — orders of magnitude faster than LaTeX
- 📦 Single binary — no Node.js, Python, or LaTeX required
- 🎨 TOML theme system — fully customizable typography and layout
- 🌐 Native CJK support — Korean, Japanese, Chinese out of the box
- 📚 Use as a Rust library — `md2paper-core` crate

**주의:** 수학식은 DESIGN_SPEC.md Phase 2 예정 기능. Features 섹션에는 comrak의 수학식 파싱 지원을 명시하되, 렌더링은 "coming in Phase 2"로 표기.

### 5. Installation
```bash
# via cargo
cargo install md2paper

# via Homebrew (coming soon)
# brew install md2paper

# Pre-built binaries → GitHub Releases page 링크
```

### 6. Quick Start
stdin 예제는 `-` 인수를 명시적으로 사용:
```bash
# Basic conversion (outputs input.pdf)
md2paper input.md

# Custom output path
md2paper input.md -o output.pdf

# Use a built-in theme
md2paper input.md --theme academic

# Read from stdin
cat README.md | md2paper - -o readme.pdf
```

### 7. Themes
빌트인 테마 4개 소개 테이블 (모두 `themes/*.toml` 파일로 구현 완료 확인됨: `default.toml`, `academic.toml`, `minimal.toml`, `newspaper.toml`):

| Theme | Description | Best for |
|-------|-------------|----------|
| `default` | Clean, readable serif layout | General purpose |
| `academic` | Narrow margins, serif-heavy, citation-ready | Papers, reports |
| `minimal` | Wide margins, minimal decoration | Essays, notes |
| `newspaper` | Two-column layout, serif headings | Newsletters, articles |

### 8. CLI Reference
핵심 옵션 9개만 포함 (일상적으로 쓰이는 것):

| Option | Description |
|--------|-------------|
| `-o, --output <PATH>` | Output PDF path |
| `-t, --theme <THEME>` | Theme name or `.toml` path |
| `--title <TEXT>` | Override document title |
| `--author <TEXT>` | Override author |
| `--font <FAMILY>` | Override body font |
| `--paper <FORMAT>` | Paper size: `a4`, `letter`, `legal` |
| `--toc` | Generate table of contents |
| `--cover` | Generate cover page from front matter |
| `--emit-typst` | Output Typst markup instead of PDF (useful for debugging custom themes) |

하단에 `See md2paper --help for all options` 안내.

### 9. Library API
실제 공개 API 기준 (검증 완료):
- `md2paper_core::convert(markdown: &str) -> Result<Vec<u8>>`
- `md2paper_core::convert_with_config(markdown: &str, config: &Config) -> Result<Vec<u8>>`
- `md2paper_core::Config::builder() -> ConfigBuilder`
- `ConfigBuilder` 체이닝 메서드: `.theme()`, `.font_family()`, `.line_height()`, `.toc()`, `.cover()`, `.title()`, `.author()`, `.date()` 등
- 빌트인 테마 로딩: `md2paper_theme::loader::load_builtin("academic")`

README 예시 코드:
```rust
use md2paper_core::{convert, convert_with_config, Config};
use md2paper_theme::loader::load_builtin;

// Simplest usage
let pdf_bytes = convert("# Hello\n\nWorld")?;
std::fs::write("output.pdf", pdf_bytes)?;

// With config
let theme = load_builtin("academic")?;
let config = Config::builder()
    .theme(theme)
    .font_family("Pretendard")
    .line_height(1.8)
    .toc(true)
    .build();

let pdf_bytes = convert_with_config("# Hello\n\nWorld", &config)?;
```

### 10. Comparison

| | md2paper | Pandoc + LaTeX | md-to-pdf (Node) | WeasyPrint |
|---|---|---|---|---|
| **Runtime** | None (single binary) | LaTeX required | Node.js required | Python required |
| **Speed** | Fast (Typst) | Slow | Medium | Medium |
| **Output Quality** | High | Very High | Medium (CSS) | Medium (CSS) |
| **Theme System** | TOML | LaTeX template | CSS | CSS |
| **CJK Support** | Native | Complex setup | Depends | Depends |
| **Binary Size** | ~20 MB | ~2 GB+ | ~200 MB+ | ~100 MB+ |

### 11. Roadmap
Phase별 간략 체크리스트:
- **Phase 1 — Core MVP** ✅: 기본 변환, 테마 시스템, CLI
- **Phase 2 — Rich Features**: GFM 테이블, 수학식, 각주, TOC, 표지
- **Phase 3 — DX & Ecosystem**: `--watch`, `--preview`, GitHub Action, WASM

### 12. Contributing
- 기여 환영 문구
- `DESIGN_SPEC.md` 링크: 아키텍처 및 파이프라인 이해용
- 빠른 시작:
  ```bash
  cargo build
  cargo test
  ```

### 13. License
MIT

---

## README.ko.md 구성

`README.md`와 동일한 섹션 구조. 모든 설명 텍스트를 한국어로 번역. 코드 블록·배지·옵션 테이블은 그대로 유지.

언어 배너에서 현재 언어(한국어)를 링크 없는 텍스트로 표시하고, 영어는 링크:
```html
<div align="right">
  <a href="README.md">🇺🇸 English</a> | 🇰🇷 한국어
</div>
```

영문 README에서는 반대로 한국어가 링크:
```html
<div align="right">
  🇺🇸 English | <a href="README.ko.md">🇰🇷 한국어</a>
</div>
```

---

## 파일 목록

| 파일 | 설명 |
|------|------|
| `README.md` | 영문 메인 README (교체) |
| `README.ko.md` | 한국어 README (신규) |

---

## 결정 사항 요약

| 항목 | 결정 |
|------|------|
| 주요 독자 | 사용자 + 기여자 둘 다 |
| 헤더 스타일 | 태그라인 + 배지(shields.io) + 핵심 가치 한 줄 |
| 언어 연결 | 최상단 언어 배너 (양방향 링크) |
| 콘텐츠 깊이 | 풍성하게 (비교표, 로드맵, Library API 포함) |
| 접근법 | B — README는 쇼케이스, 깊은 스펙은 DESIGN_SPEC.md로 링크 |
| stdin 예제 | `cat file.md | md2paper - -o out.pdf` (`-` 명시) |
| 수학식 표기 | Features에 comrak 파싱 지원 명시, 렌더링은 "Phase 2" 표기 |
| CLI 핵심 옵션 | 9개 (일상 사용 기준) |
| 배지 | shields.io 기반 4개 |
