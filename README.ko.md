<div align="right">
  <a href="README.md">🇺🇸 English</a> | 🇰🇷 한국어
</div>

<div align="center">
  <h1>md2paper</h1>
  <p><em>마크다운을 아름답게 조판된 문서로 변환하세요.</em></p>

  <a href="https://crates.io/crates/md2paper"><img src="https://img.shields.io/crates/v/md2paper" alt="crates.io"></a>
  <a href="LICENSE"><img src="https://img.shields.io/crates/l/md2paper" alt="License: MIT"></a>
  <img src="https://img.shields.io/github/actions/workflow/status/lucas-flatwhite/md2paper/ci.yml" alt="Build Status">
  <img src="https://img.shields.io/badge/rust-2021-orange" alt="Rust 2021">

  <br><br>

  > 📸 스크린샷 준비 중

</div>

---

✨ 기본으로 아름답게 · ⚡ 번개처럼 빠르게 · 📦 런타임 의존성 없음 · 🎨 완전한 커스터마이징

## 기능

- 🖋 **GFM Markdown** — 체크리스트, 취소선, 자동링크 지원 (표·각주는 Phase 2에서 지원 예정)
- ➗ **수학식 렌더링** — `$인라인$`, `$$블록$$` (KaTeX 호환 문법으로 파싱; 전체 렌더링은 Phase 2에서 지원 예정)
- ⚡ **Typst 기반** — LaTeX 대비 수십 배 빠른 변환
- 📦 **단일 바이너리** — Node.js, Python, LaTeX 불필요
- 🎨 **TOML 테마 시스템** — 타이포그라피와 레이아웃을 자유롭게 커스터마이징
- 🌐 **CJK 네이티브 지원** — 한국어, 일본어, 중국어 기본 지원
- 📚 **Rust 라이브러리로 사용 가능** — `md2paper-core` 크레이트

## 설치

```bash
# cargo를 통해 설치
cargo install md2paper

# Homebrew를 통해 설치 (준비 중)
# brew install md2paper
```

주요 플랫폼용 사전 빌드된 바이너리는 [GitHub Releases](https://github.com/lucas-flatwhite/md2paper/releases) 페이지에서 다운로드할 수 있습니다.

## 빠른 시작

```bash
# 기본 변환 (input.pdf로 출력)
md2paper input.md

# 출력 경로 지정
md2paper input.md -o output.pdf

# 내장 테마 사용
md2paper input.md --theme academic

# stdin에서 읽기
cat README.md | md2paper - -o readme.pdf
```

## 테마

| 테마 | 설명 | 적합한 용도 |
|------|------|------------|
| `default` | 깔끔하고 읽기 쉬운 세리프 레이아웃 | 일반 목적 |
| `academic` | 좁은 여백, 세리프 강조, 인용 적합 | 논문, 보고서 |
| `minimal` | 넓은 여백, 최소한의 장식 | 에세이, 노트 |
| `newspaper` | 2단 레이아웃, 세리프 제목 | 뉴스레터, 기사 |

내장 테마를 사용하려면 `--theme <이름>` 옵션을 사용하고, 커스텀 테마를 사용하려면 `.toml` 파일 경로를 지정하세요.

## 주요 옵션

전체 옵션 목록은 `md2paper --help`를 참조하세요.

| 옵션 | 설명 |
|------|------|
| `-o, --output <PATH>` | 출력 PDF 경로 |
| `-t, --theme <THEME>` | 테마 이름 또는 `.toml` 파일 경로 |
| `--title <TEXT>` | 문서 제목 재정의 |
| `--author <TEXT>` | 문서 저자 재정의 |
| `--font <FAMILY>` | 본문 폰트 패밀리 재정의 |
| `--paper <FORMAT>` | 용지 크기: `a4`, `letter`, `legal` |
| `--toc` | 목차 생성 |
| `--cover` | 프론트 매터로 표지 생성 |
| `--emit-typst` | PDF 대신 Typst 마크업 출력 (커스텀 테마 디버깅에 유용) |

## 라이브러리 API

```rust
use md2paper_core::{convert, convert_with_config, Config};
use md2paper_theme::loader::load_builtin;

// 가장 간단한 사용법
let pdf_bytes = convert("# Hello\n\nWorld")?;
std::fs::write("output.pdf", pdf_bytes)?;

// 설정과 함께 사용
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
| **CJK 지원** | 네이티브 | 복잡한 설정 필요 | 환경에 따라 다름 | 환경에 따라 다름 |
| **바이너리 크기** | ~20 MB | ~2 GB+ | ~200 MB+ | ~100 MB+ |

## 로드맵

- **Phase 1 — 핵심 MVP** ✅ 기본 변환, 테마 시스템, CLI
- **Phase 2 — 풍부한 기능** GFM 표, 수학식 렌더링, 각주, 목차, 표지
- **Phase 3 — 개발 경험 & 생태계** `--watch`, `--preview`, GitHub Action, WASM

## 기여하기

기여는 언제나 환영합니다! 버그 리포트, 기능 요청, 풀 리퀘스트 등 모든 의견을 소중히 여깁니다.

아키텍처와 설계 결정 사항은 [DESIGN_SPEC.md](DESIGN_SPEC.md)를 참고하세요.

시작하려면:

```bash
cargo build
cargo test
```

## 라이선스

[MIT](LICENSE)
