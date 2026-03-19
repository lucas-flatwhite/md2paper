# README Redesign Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the 2-line README.md with a comprehensive English README and add an identical-structure Korean README.ko.md.

**Architecture:** Two standalone Markdown files — `README.md` (English, primary) and `README.ko.md` (Korean translation). Each file has a language banner at the top linking to the other. No code changes required.

**Tech Stack:** GitHub Flavored Markdown, HTML (for alignment divs and badges), shields.io badges.

**Spec:** `docs/superpowers/specs/2026-03-19-readme-redesign-design.md`

---

## File Map

| File | Action | Responsibility |
|------|--------|----------------|
| `README.md` | Modify (full rewrite) | English README — 13 sections from spec |
| `README.ko.md` | Create | Korean README — same structure, all text translated |

---

### Task 1: Write README.md (English)

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Write the full README.md**

Replace the entire contents of `README.md` with the following:

```markdown
<div align="right">
  🇺🇸 English | <a href="README.ko.md">🇰🇷 한국어</a>
</div>

<div align="center">
  <h1>md2paper</h1>
  <p><em>Turn your Markdown into beautifully typeset paper.</em></p>

  <a href="https://crates.io/crates/md2paper"><img src="https://img.shields.io/crates/v/md2paper" alt="crates.io"></a>
  <a href="LICENSE"><img src="https://img.shields.io/crates/l/md2paper" alt="License: MIT"></a>
  <img src="https://img.shields.io/github/actions/workflow/status/lucas-flatwhite/md2paper/ci.yml" alt="Build Status">
  <img src="https://img.shields.io/badge/rust-2021-orange" alt="Rust 2021">

  <br><br>

  > 📸 Screenshot coming soon

</div>

---

✨ Beautiful by default · ⚡ Lightning fast · 📦 Zero dependency · 🎨 Themeable

---

## Features

- 🖋 **Full GFM Markdown** — tables, footnotes, task lists, strikethrough, autolinks
- ➗ **Math syntax** — `$inline$` and `$$display$$` (KaTeX-compatible, parsed via comrak; full rendering coming in Phase 2)
- ⚡ **Typst-powered** — orders of magnitude faster than LaTeX
- 📦 **Single binary** — no Node.js, Python, or LaTeX required
- 🎨 **TOML theme system** — fully customizable typography and layout
- 🌐 **Native CJK support** — Korean, Japanese, Chinese out of the box
- 📚 **Rust library** — use `md2paper-core` as a library in your own projects

## Installation

```bash
# via cargo
cargo install md2paper

# via Homebrew (coming soon)
# brew install md2paper
```

Or download a pre-built binary from the [GitHub Releases](https://github.com/lucas-flatwhite/md2paper/releases) page.

## Quick Start

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

## Themes

md2paper ships with four built-in themes:

| Theme | Description | Best for |
|-------|-------------|----------|
| `default` | Clean, readable serif layout | General purpose |
| `academic` | Narrow margins, serif-heavy, citation-ready | Papers, reports |
| `minimal` | Wide margins, minimal decoration | Essays, notes |
| `newspaper` | Two-column layout, serif headings | Newsletters, articles |

Use `--theme <name>` to select one, or pass a path to your own `.toml` file.

## CLI Reference

| Option | Description |
|--------|-------------|
| `-o, --output <PATH>` | Output PDF path |
| `-t, --theme <THEME>` | Theme name or path to `.toml` file |
| `--title <TEXT>` | Override document title |
| `--author <TEXT>` | Override document author |
| `--font <FAMILY>` | Override body font family |
| `--paper <FORMAT>` | Paper size: `a4`, `letter`, `legal` |
| `--toc` | Generate table of contents |
| `--cover` | Generate cover page from front matter |
| `--emit-typst` | Output Typst markup instead of PDF (useful for debugging custom themes) |

See `md2paper --help` for all options.

## Library API

Add `md2paper-core` and `md2paper-theme` to your `Cargo.toml`, then:

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

## Comparison

| | md2paper | Pandoc + LaTeX | md-to-pdf (Node) | WeasyPrint |
|---|---|---|---|---|
| **Runtime** | None (single binary) | LaTeX required | Node.js required | Python required |
| **Speed** | Fast (Typst) | Slow | Medium | Medium |
| **Output Quality** | High | Very High | Medium (CSS) | Medium (CSS) |
| **Theme System** | TOML | LaTeX template | CSS | CSS |
| **CJK Support** | Native | Complex setup | Depends | Depends |
| **Binary Size** | ~20 MB | ~2 GB+ | ~200 MB+ | ~100 MB+ |

## Roadmap

- **Phase 1 — Core MVP** ✅ Basic conversion, theme system, CLI
- **Phase 2 — Rich Features** GFM tables, math rendering, footnotes, TOC, cover pages
- **Phase 3 — DX & Ecosystem** `--watch`, `--preview`, GitHub Action, WASM

## Contributing

Contributions are welcome! Please read [DESIGN_SPEC.md](DESIGN_SPEC.md) to understand the architecture and pipeline before diving in.

```bash
cargo build
cargo test
```

## License

This project is licensed under the [MIT License](LICENSE).
```

- [ ] **Step 2: Verify the file looks correct**

Open `README.md` and confirm:
- Language banner at top with Korean link
- `<div align="center">` header block with badges
- All 13 sections present and in order
- Code blocks are properly fenced

- [ ] **Step 3: Commit**

```bash
git add README.md
git commit -m "docs: rewrite README with full English content"
```

---

### Task 2: Write README.ko.md (Korean)

**Files:**
- Create: `README.ko.md`

- [ ] **Step 1: Write the full README.ko.md**

Create `README.ko.md` with the following content (same structure as README.md, all prose translated to Korean, code blocks preserved as-is):

```markdown
<div align="right">
  <a href="README.md">🇺🇸 English</a> | 🇰🇷 한국어
</div>

<div align="center">
  <h1>md2paper</h1>
  <p><em>Markdown을 아름답게 조판된 문서로 변환하세요.</em></p>

  <a href="https://crates.io/crates/md2paper"><img src="https://img.shields.io/crates/v/md2paper" alt="crates.io"></a>
  <a href="LICENSE"><img src="https://img.shields.io/crates/l/md2paper" alt="License: MIT"></a>
  <img src="https://img.shields.io/github/actions/workflow/status/lucas-flatwhite/md2paper/ci.yml" alt="빌드 상태">
  <img src="https://img.shields.io/badge/rust-2021-orange" alt="Rust 2021">

  <br><br>

  > 📸 스크린샷 준비 중

</div>

---

✨ 기본으로 아름답게 · ⚡ 번개처럼 빠르게 · 📦 의존성 없이 · 🎨 완전한 커스터마이징

---

## 기능

- 🖋 **완전한 GFM Markdown** — 표, 각주, 체크리스트, 취소선, 자동링크
- ➗ **수학식 문법** — `$인라인$` 및 `$$블록$$` (KaTeX 호환 문법으로 파싱; 렌더링은 Phase 2에서 지원)
- ⚡ **Typst 기반** — LaTeX 대비 수십 배 빠른 변환
- 📦 **단일 바이너리** — Node.js, Python, LaTeX 불필요
- 🎨 **TOML 테마 시스템** — 타이포그라피와 레이아웃을 자유롭게 커스터마이징
- 🌐 **CJK 네이티브 지원** — 한국어, 일본어, 중국어 기본 지원
- 📚 **Rust 라이브러리** — `md2paper-core` 크레이트를 라이브러리로 사용 가능

## 설치

```bash
# cargo로 설치
cargo install md2paper

# Homebrew (출시 예정)
# brew install md2paper
```

또는 [GitHub Releases](https://github.com/lucas-flatwhite/md2paper/releases) 페이지에서 사전 빌드된 바이너리를 다운로드하세요.

## 빠른 시작

```bash
# 기본 변환 (input.pdf로 출력)
md2paper input.md

# 출력 파일 지정
md2paper input.md -o output.pdf

# 빌트인 테마 사용
md2paper input.md --theme academic

# 표준 입력에서 읽기
cat README.md | md2paper - -o readme.pdf
```

## 테마

md2paper는 4가지 빌트인 테마를 제공합니다:

| 테마 | 설명 | 적합한 용도 |
|------|------|------------|
| `default` | 깔끔하고 가독성 좋은 세리프 레이아웃 | 범용 |
| `academic` | 좁은 여백, 세리프 중심, 인용 준비 완료 | 논문, 리포트 |
| `minimal` | 넓은 여백, 최소한의 장식 | 에세이, 메모 |
| `newspaper` | 2단 레이아웃, 세리프 제목 | 뉴스레터, 기사 |

`--theme <이름>` 으로 선택하거나, 직접 만든 `.toml` 파일 경로를 전달하세요.

## CLI 레퍼런스

| 옵션 | 설명 |
|------|------|
| `-o, --output <PATH>` | 출력 PDF 경로 |
| `-t, --theme <THEME>` | 테마 이름 또는 `.toml` 파일 경로 |
| `--title <TEXT>` | 문서 제목 오버라이드 |
| `--author <TEXT>` | 저자 오버라이드 |
| `--font <FAMILY>` | 본문 폰트 패밀리 오버라이드 |
| `--paper <FORMAT>` | 용지 크기: `a4`, `letter`, `legal` |
| `--toc` | 목차 생성 |
| `--cover` | Front matter 기반 표지 페이지 생성 |
| `--emit-typst` | PDF 대신 Typst 마크업 출력 (커스텀 테마 디버깅에 유용) |

전체 옵션은 `md2paper --help` 를 참조하세요.

## 라이브러리 API

`Cargo.toml`에 `md2paper-core`와 `md2paper-theme`를 추가한 후:

```rust
use md2paper_core::{convert, convert_with_config, Config};
use md2paper_theme::loader::load_builtin;

// 가장 간단한 사용법
let pdf_bytes = convert("# Hello\n\nWorld")?;
std::fs::write("output.pdf", pdf_bytes)?;

// 설정 커스터마이징
let theme = load_builtin("academic")?;
let config = Config::builder()
    .theme(theme)
    .font_family("Pretendard")
    .line_height(1.8)
    .toc(true)
    .build();

let pdf_bytes = convert_with_config("# Hello\n\nWorld", &config)?;
```

## 비교

| | md2paper | Pandoc + LaTeX | md-to-pdf (Node) | WeasyPrint |
|---|---|---|---|---|
| **런타임** | 없음 (단일 바이너리) | LaTeX 필요 | Node.js 필요 | Python 필요 |
| **속도** | 빠름 (Typst) | 느림 | 보통 | 보통 |
| **출력 품질** | 높음 | 매우 높음 | 보통 (CSS) | 보통 (CSS) |
| **테마 시스템** | TOML | LaTeX 템플릿 | CSS | CSS |
| **CJK 지원** | 네이티브 | 복잡한 설정 | 환경 의존 | 환경 의존 |
| **바이너리 크기** | ~20 MB | ~2 GB+ | ~200 MB+ | ~100 MB+ |

## 로드맵

- **Phase 1 — Core MVP** ✅ 기본 변환, 테마 시스템, CLI
- **Phase 2 — Rich Features** GFM 테이블, 수학식 렌더링, 각주, 목차, 표지
- **Phase 3 — DX & Ecosystem** `--watch`, `--preview`, GitHub Action, WASM

## 기여하기

기여를 환영합니다! 아키텍처와 파이프라인을 이해하려면 먼저 [DESIGN_SPEC.md](DESIGN_SPEC.md)를 읽어주세요.

```bash
cargo build
cargo test
```

## 라이선스

이 프로젝트는 [MIT 라이선스](LICENSE)를 따릅니다.
```

- [ ] **Step 2: Verify the file looks correct**

Open `README.ko.md` and confirm:
- Language banner at top with English link, Korean as plain text
- Same badge block as README.md
- All 13 sections present in Korean
- Code blocks identical to README.md

- [ ] **Step 3: Commit**

```bash
git add README.ko.md
git commit -m "docs: add Korean README (README.ko.md)"
```

---

### Task 3: Final Verification

- [ ] **Step 1: Check cross-links work**

Confirm:
- `README.md` has `<a href="README.ko.md">🇰🇷 한국어</a>` at top
- `README.ko.md` has `<a href="README.md">🇺🇸 English</a>` at top
- Both files reference `DESIGN_SPEC.md` in the Contributing section
- Both files reference `LICENSE` in the License section

- [ ] **Step 2: Verify section count**

Both files should have all 13 sections:
1. Language banner
2. Title + tagline + badges
3. Core values line
4. Features
5. Installation
6. Quick Start
7. Themes
8. CLI Reference
9. Library API
10. Comparison table
11. Roadmap
12. Contributing
13. License

- [ ] **Step 3: Final commit (if any fixups needed)**

```bash
git add README.md README.ko.md
git commit -m "docs: fix README cross-links and formatting"
```
