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

✨ Beautiful by default · ⚡ Lightning fast · 📦 Zero runtime deps · 🎨 Themeable

## Features

- 🖋 **GFM Markdown** — task lists, strikethrough, autolinks (tables & footnotes coming in Phase 2)
- ➗ **Math rendering** — `$inline$` and `$$display$$` (KaTeX-compatible, parsed via comrak; full rendering coming in Phase 2)
- ⚡ **Typst-powered** — orders of magnitude faster than LaTeX
- 📦 **Single binary** — no Node.js, Python, or LaTeX required
- 🎨 **TOML theme system** — fully customizable typography and layout
- 🌐 **Native CJK support** — Korean, Japanese, Chinese out of the box
- 📚 **Use as a Rust library** — `md2paper-core` crate

## Installation

```bash
# via cargo
cargo install md2paper

# via Homebrew (coming soon)
# brew install md2paper
```

Pre-built binaries for major platforms are available on the [GitHub Releases](https://github.com/lucas-flatwhite/md2paper/releases) page.

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

| Theme | Description | Best for |
|-------|-------------|----------|
| `default` | Clean, readable serif layout | General purpose |
| `academic` | Narrow margins, serif-heavy, citation-ready | Papers, reports |
| `minimal` | Wide margins, minimal decoration | Essays, notes |
| `newspaper` | Two-column layout, serif headings | Newsletters, articles |

Use `--theme <name>` to select a built-in theme, or pass a path to a `.toml` file to use a custom theme.

## Common Options

See `md2paper --help` for the full list of options.

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

Contributions are welcome! Whether it's a bug report, feature request, or pull request — all input is appreciated.

For architecture and design decisions, see [DESIGN_SPEC.md](DESIGN_SPEC.md).

To get started:

```bash
cargo build
cargo test
```

## License

[MIT](LICENSE)
