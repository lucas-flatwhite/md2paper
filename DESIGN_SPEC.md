# md2paper — Design Specification

> Turn your Markdown into beautifully typeset paper.

---

## 1. Overview

**md2paper**는 Markdown 문서를 아름답게 조판된 PDF로 변환하는 오픈소스 CLI 도구입니다.

Rust로 작성되며, 차세대 조판 엔진 [Typst](https://typst.app)를 내부에 임베드하여 LaTeX 없이도 빠르고 고품질의 PDF를 생성합니다. Single binary로 배포되어 별도 런타임(Node.js, Python, LaTeX 등)이 필요 없습니다.

### 핵심 가치

| 가치 | 설명 |
|------|------|
| **Beautiful by default** | 테마를 지정하지 않아도 타이포그라피가 아름다운 기본 결과물 |
| **Fast** | Typst 기반으로 LaTeX 대비 수십 배 빠른 변환 |
| **Zero dependency** | Single binary, `cargo install` 또는 `brew install` 한 줄로 설치 |
| **Themeable** | TOML 기반 테마 시스템으로 완전한 커스터마이징 |
| **Extensible** | 라이브러리로도 사용 가능한 모듈형 아키텍처 |

---

## 2. Conversion Pipeline

```
┌──────────┐    ┌──────────┐    ┌──────────────┐    ┌─────────────┐    ┌──────┐
│ Markdown │───▶│   AST    │───▶│ Typst Markup │───▶│ Typst Engine│───▶│ PDF  │
│  (input) │    │ (comrak) │    │ (generated)  │    │ (embedded)  │    │(out) │
└──────────┘    └──────────┘    └──────────────┘    └─────────────┘    └──────┘
                                       ▲
                                       │
                                ┌──────┴───────┐
                                │ Theme (TOML) │
                                └──────────────┘
```

### Stage 1 — Markdown Parsing

- **Parser**: `comrak` crate (GitHub Flavored Markdown 완벽 지원)
- **지원 문법**:
  - CommonMark 기본 문법 전체
  - GFM 확장: 테이블, 취소선, 자동링크, task list
  - 각주 (footnotes)
  - 수학식: `$inline$` 및 `$$display$$` (KaTeX 호환 문법)
  - Front matter (YAML): 메타데이터 추출용

### Stage 2 — AST → Typst Markup 변환

Markdown AST를 순회하며 Typst 마크업으로 변환합니다. 이 레이어가 프로젝트의 핵심입니다.

**매핑 테이블:**

| Markdown | Typst |
|----------|-------|
| `# Heading` | `= Heading` |
| `## Heading` | `== Heading` |
| `**bold**` | `*bold*` |
| `*italic*` | `_italic_` |
| `` `code` `` | `` `code` `` |
| ```` ```lang ```` | ````raw(lang: "lang", block: true)[...]```` |
| `> quote` | `#quote[...]` |
| `- list` | `- list` |
| `1. list` | `+ list` (또는 `enum`) |
| `[link](url)` | `#link("url")[text]` |
| `![alt](img)` | `#image("img", alt: "alt")` |
| `$math$` | `$math$` |
| `---` | `#line(length: 100%)` |
| table | `#table(...)` |
| footnote | `#footnote[...]` |

### Stage 3 — Theme 적용

변환된 Typst 마크업 앞에 테마에서 정의한 `#set`, `#show` 규칙을 주입합니다.

### Stage 4 — PDF 렌더링

`typst` crate을 라이브러리로 직접 임베드하여 Typst 마크업을 PDF로 컴파일합니다. 외부 바이너리 호출이 아닌 in-process 변환이므로 오버헤드가 최소화됩니다.

---

## 3. Theme System

### 3.1 테마 파일 형식 (TOML)

```toml
# theme.toml
[meta]
name = "default"
description = "Clean, readable default theme"
version = "1.0.0"

[page]
paper = "a4"               # a4, letter, legal, ...
margin_top = "2.5cm"
margin_bottom = "2.5cm"
margin_left = "2.5cm"
margin_right = "2.5cm"
columns = 1                 # 1 or 2

[font]
body_family = "Noto Serif"
body_size = "11pt"
body_weight = "regular"

heading_family = "Noto Sans"
heading_weight = "bold"

code_family = "JetBrains Mono"
code_size = "9pt"

fallback = ["Noto Sans KR", "Noto Sans JP"]   # CJK fallback

[spacing]
line_height = 1.6           # 행간 (leading)
paragraph_spacing = "0.8em" # 단락 간격
letter_spacing = "0pt"      # 자간 (tracking)
heading_above = "1.2em"     # 제목 위 간격
heading_below = "0.6em"     # 제목 아래 간격

[color]
body_text = "#333333"
heading = "#1a1a2e"
link = "#2563eb"
code_text = "#e06c75"
code_background = "#f6f8fa"
blockquote_border = "#d1d5db"
blockquote_text = "#6b7280"

[code]
theme = "catppuccin-mocha"  # 신택스 하이라이팅 테마
line_numbers = false
border_radius = "4pt"

[header_footer]
header_left = ""
header_center = ""
header_right = "{title}"
footer_left = ""
footer_center = "{page}"
footer_right = ""
```

### 3.2 빌트인 테마

| 테마 | 설명 | 대상 |
|------|------|------|
| `default` | 깔끔하고 가독성 좋은 기본 테마 | 범용 |
| `academic` | 학술 논문 스타일 (serif 중심, 좁은 여백) | 논문, 리포트 |
| `minimal` | 최소한의 장식, 넓은 여백 | 에세이, 메모 |
| `newspaper` | 2단 레이아웃, 세리프 제목 | 뉴스레터, 기사 |

### 3.3 테마 해석 우선순위

```
CLI 인라인 옵션  >  Front matter  >  사용자 테마 파일  >  빌트인 테마  >  default
```

---

## 4. Front Matter

Markdown 파일 상단의 YAML front matter로 문서별 메타데이터와 테마 오버라이드를 지정할 수 있습니다.

```yaml
---
title: "문서 제목"
author: "작성자"
date: "2026-03-19"
theme: academic
font:
  body_family: "Pretendard"
  body_size: "10.5pt"
spacing:
  line_height: 1.8
---
```

- `title`, `author`, `date`는 테마의 표지(title page) 또는 헤더/푸터에서 사용됩니다.
- 나머지 키는 테마 TOML의 동일 섹션을 오버라이드합니다.

---

## 5. CLI Interface

### 5.1 기본 사용법

```bash
# 기본 변환 (output: input과 같은 이름의 .pdf)
md2paper input.md

# 출력 파일 지정
md2paper input.md -o output.pdf

# 테마 지정
md2paper input.md --theme academic

# 커스텀 테마 파일
md2paper input.md --theme ./my-theme.toml

# 여러 파일 일괄 변환
md2paper chapter1.md chapter2.md chapter3.md -o book.pdf

# 표준 입력에서 읽기
cat README.md | md2paper -o readme.pdf
```

### 5.2 전체 옵션

```
md2paper [OPTIONS] [INPUT...]

Arguments:
  [INPUT...]                 Input markdown file(s). Use '-' for stdin.

Options:
  -o, --output <PATH>        Output PDF path [default: <input>.pdf]
  -t, --theme <THEME>        Theme name or path to .toml file [default: default]
      --title <TEXT>          Override document title
      --author <TEXT>         Override document author
      --date <TEXT>           Override document date

Typography:
      --font <FAMILY>         Override body font family
      --font-size <SIZE>      Override body font size (e.g. "11pt")
      --line-height <RATIO>   Override line height (e.g. 1.6)
      --letter-spacing <SIZE> Override letter spacing (e.g. "0.5pt")

Page:
      --paper <FORMAT>        Paper format: a4, letter, legal [default: a4]
      --margin <SIZE>         Set all margins (e.g. "2.5cm")

Output:
      --toc                   Generate table of contents
      --toc-depth <N>         TOC depth [default: 3]
      --page-numbers          Show page numbers [default: true]
      --cover                 Generate cover page from front matter

Debug:
      --emit-typst            Output generated Typst markup instead of PDF
      --dump-ast              Dump Markdown AST as JSON
  -v, --verbose              Verbose output
  -q, --quiet                Suppress all output except errors

  -h, --help                 Print help
  -V, --version              Print version
```

### 5.3 Watch Mode (Phase 2)

```bash
# 파일 변경 시 자동 재변환
md2paper input.md --watch

# 브라우저에서 실시간 미리보기
md2paper input.md --preview
```

---

## 6. Library API (mdpdf-core)

CLI 외에도 Rust 라이브러리로 사용할 수 있어야 합니다.

```rust
use md2paper::{convert, Config, Theme};

fn main() -> anyhow::Result<()> {
    // 가장 간단한 사용법
    let pdf_bytes = convert("# Hello\n\nWorld")?;
    std::fs::write("output.pdf", pdf_bytes)?;

    // 설정 커스터마이징
    let config = Config::builder()
        .theme(Theme::builtin("academic")?)
        .font_family("Pretendard")
        .line_height(1.8)
        .toc(true)
        .build();

    let pdf_bytes = convert_with_config("# Hello\n\nWorld", &config)?;
    std::fs::write("output.pdf", pdf_bytes)?;

    Ok(())
}
```

---

## 7. Project Structure

```
md2paper/
├── Cargo.toml                 # workspace root
├── DESIGN_SPEC.md
├── LICENSE
├── README.md
│
├── crates/
│   ├── md2paper-cli/          # CLI 엔트리포인트
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs        # clap 기반 CLI
│   │
│   ├── md2paper-core/         # 핵심 변환 로직
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs         # public API (convert, convert_with_config)
│   │       ├── parser.rs      # Markdown → AST (comrak wrapper)
│   │       ├── transform.rs   # AST → Typst markup
│   │       └── config.rs      # Config builder
│   │
│   ├── md2paper-theme/        # 테마 시스템
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── model.rs       # Theme struct, serde 정의
│   │       ├── loader.rs      # TOML 파싱, 빌트인 로딩
│   │       └── inject.rs      # Typst set/show 규칙 생성
│   │
│   └── md2paper-render/       # Typst 엔진 래퍼
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── world.rs       # typst::World 구현 (폰트, 파일 해석)
│           └── render.rs      # Typst markup → PDF bytes
│
├── themes/                    # 빌트인 테마 TOML 파일들
│   ├── default.toml
│   ├── academic.toml
│   ├── minimal.toml
│   └── newspaper.toml
│
└── tests/
    ├── fixtures/              # 테스트용 Markdown 샘플
    │   ├── basic.md
    │   ├── gfm_table.md
    │   ├── math.md
    │   ├── cjk.md
    │   └── full_featured.md
    └── integration/
        ├── convert_test.rs
        └── theme_test.rs
```

---

## 8. Key Dependencies

| Crate | 용도 | 비고 |
|-------|------|------|
| `comrak` | Markdown 파싱 (GFM) | CommonMark + 확장 문법 |
| `typst` | 조판 엔진 (PDF 렌더링) | 라이브러리로 임베드 |
| `typst-pdf` | Typst → PDF export | typst 엔진의 PDF 백엔드 |
| `clap` | CLI 인자 파싱 | derive 매크로 사용 |
| `serde` + `toml` | 테마 TOML 파싱 | |
| `serde_yaml` | Front matter 파싱 | |
| `anyhow` | 에러 처리 | |
| `syntect` | 코드 신택스 하이라이팅 | Typst 내장 지원과 병행 검토 |

---

## 9. Typography Principles

아름다운 기본 출력을 위해 `default` 테마에 적용할 타이포그라피 원칙:

### 9.1 서체 선택

- **본문**: Serif 계열 (Noto Serif, Libertinus Serif) — 인쇄물 가독성 최적화
- **제목**: Sans-serif 계열 (Noto Sans) — 시각적 대비, 위계 구분
- **코드**: Monospace (JetBrains Mono) — 리거처 지원, 가독성
- **CJK Fallback**: Noto Sans KR/JP/SC — 다국어 지원

### 9.2 수치 기준

| 요소 | 값 | 근거 |
|------|-----|------|
| 본문 크기 | 11pt | 학술/비즈니스 문서 표준 |
| 행간 (line-height) | 1.5–1.6 | 가독성 최적 범위 |
| 단락 간격 | 0.8em | 행간보다 넓되 과하지 않게 |
| 자간 (tracking) | 0pt (기본) | 본문에선 서체 기본값 유지 |
| 페이지 여백 | 2.5cm | A4 기준 적정 여백 |
| 코드 블록 폰트 | 9pt | 본문 대비 약간 작게 |

### 9.3 시각적 위계

```
H1  — 24pt, bold, sans-serif, 위 간격 크게
H2  — 18pt, bold, sans-serif
H3  — 14pt, bold, sans-serif
H4  — 12pt, bold, sans-serif
Body — 11pt, regular, serif
Code — 9pt, regular, monospace, 배경색 처리
Caption — 9pt, regular, muted color
```

---

## 10. Image & Asset Handling

- Markdown의 `![alt](path)` 이미지는 **입력 파일 기준 상대 경로**로 해석합니다.
- 지원 포맷: PNG, JPEG, SVG, GIF (첫 프레임)
- HTTP(S) URL 이미지는 변환 시점에 다운로드하여 임시 캐시에 저장합니다.
- `--base-dir` 옵션으로 이미지 기준 디렉터리를 오버라이드할 수 있습니다.
- 이미지 크기: Typst의 `width`, `height` 속성으로 제어. 기본값은 페이지 폭에 맞춤.

---

## 11. Error Handling

| 상황 | 동작 |
|------|------|
| 존재하지 않는 이미지 경로 | 경고 출력 + placeholder 표시, 변환 계속 |
| 잘못된 Markdown 문법 | 가능한 한 관대하게 처리 (comrak 기본 동작) |
| 잘못된 테마 TOML | 에러 메시지와 함께 해당 키 표시, 변환 중단 |
| 지원하지 않는 폰트 | 경고 출력 + fallback 폰트 사용 |
| 잘못된 수학식 | 경고 출력 + raw text로 렌더링 |

---

## 12. Phased Roadmap

### Phase 1 — Core (MVP)

> 목표: Markdown → PDF 기본 변환이 동작하는 최소 제품

- [ ] Cargo workspace 및 crate 구조 세팅
- [ ] `comrak` 기반 Markdown 파서 통합
- [ ] AST → Typst 마크업 변환기 (기본 문법: heading, paragraph, bold, italic, code, link, image, list, blockquote, horizontal rule)
- [ ] Typst 엔진 임베딩 및 PDF 출력
- [ ] `default` 테마 구현
- [ ] TOML 테마 로더
- [ ] `clap` 기반 CLI (`md2paper input.md -o output.pdf --theme <name>`)
- [ ] Front matter 파싱 (title, author, date)
- [ ] 기본 테스트 suite

### Phase 2 — Rich Features

> 목표: 실사용에 필요한 풍부한 기능

- [ ] GFM 테이블 변환
- [ ] 코드 블록 신택스 하이라이팅
- [ ] 수학식 렌더링 (`$...$`, `$$...$$`)
- [ ] 각주 (footnotes)
- [ ] 자동 목차 생성 (`--toc`)
- [ ] 표지 페이지 생성 (`--cover`)
- [ ] 헤더/푸터 (페이지 번호, 제목 등)
- [ ] `academic`, `minimal`, `newspaper` 빌트인 테마 추가
- [ ] HTTP 이미지 다운로드
- [ ] `--emit-typst` 디버그 옵션
- [ ] 여러 파일 합쳐서 변환 (multi-file → single PDF)

### Phase 3 — DX & Ecosystem

> 목표: 개발자 경험과 생태계 확장

- [ ] `--watch` mode (파일 변경 감지 → 자동 재변환)
- [ ] `--preview` mode (로컬 서버 + 브라우저 실시간 미리보기)
- [ ] 커스텀 Typst 스니펫 삽입 (`:::typst` fenced block)
- [ ] 커뮤니티 테마 레지스트리 구조 설계
- [ ] `md2paper init` — 프로젝트에 설정 파일 스캐폴딩
- [ ] CI/CD용 GitHub Action 제공
- [ ] WASM 빌드 (브라우저에서 변환)
- [ ] Web UI 미리보기 앱 (WASM 기반)

---

## 13. Comparison with Existing Tools

| | md2paper | Pandoc + LaTeX | md-to-pdf (Node) | WeasyPrint |
|---|---|---|---|---|
| **Runtime** | None (single binary) | LaTeX 필요 | Node.js 필요 | Python 필요 |
| **Speed** | Fast (Typst) | Slow (LaTeX) | Medium | Medium |
| **Output Quality** | High (Typst 조판) | Very High | Medium (CSS) | Medium (CSS) |
| **Theme System** | TOML | LaTeX template | CSS | CSS |
| **CJK Support** | Good (Typst native) | Complex setup | Depends | Depends |
| **Binary Size** | ~20MB | ~2GB+ | ~200MB+ | ~100MB+ |
| **Learning Curve** | Low | High | Low | Medium |

---

## 14. Future Considerations

- **Plugin System**: WASM 기반 플러그인으로 커스텀 AST 변환 로직을 주입할 수 있는 구조. 예: 특정 Markdown 확장 문법 처리, 커스텀 블록 렌더링.
- **Slide Mode**: Markdown → 프레젠테이션 PDF (Typst의 `#polylux` 패키지 활용)
- **Batch/CI 파이프라인**: GitHub Action, GitLab CI template 등으로 자동 문서 빌드
- **i18n**: 날짜 형식, 목차 제목("Table of Contents" vs "목차") 등의 로케일 지원
- **Accessibility**: PDF/UA 준수, 태그 구조화된 PDF 출력
