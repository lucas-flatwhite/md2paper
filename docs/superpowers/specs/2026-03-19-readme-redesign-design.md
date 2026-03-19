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
  ```html
  <div align="right">
    <a href="README.md">🇺🇸 English</a> | <a href="README.ko.md">🇰🇷 한국어</a>
  </div>
  ```

---

## README.md 섹션 구성

### 1. 언어 선택 배너
`<div align="right">` 로 오른쪽 정렬, 영어/한국어 링크.

### 2. 제목 + 태그라인 + 배지
`<div align="center">` 블록 안에:
- `<h1>md2paper</h1>`
- 태그라인: *Turn your Markdown into beautifully typeset paper.*
- 배지 4개 (한 줄): crates.io 버전, License MIT, Build Status, Rust 2021
- 데모 이미지 placeholder (나중에 GIF 교체)

### 3. 핵심 가치 한 줄
> ✨ Beautiful by default · ⚡ Lightning fast · 📦 Zero dependency · 🎨 Themeable

### 4. Features
아이콘 + 한 줄 설명 불릿 리스트:
- 🖋 GFM Markdown 완벽 지원 (테이블, 수학식, 각주 등)
- ⚡ Typst 기반 — LaTeX 대비 수십 배 빠른 변환
- 📦 Single binary — 별도 런타임 불필요
- 🎨 TOML 테마 시스템 — 완전한 커스터마이징
- 🌐 CJK 네이티브 지원
- 📚 Rust 라이브러리로도 사용 가능

### 5. Installation
```bash
# via cargo
cargo install md2paper

# via Homebrew (coming soon)
brew install md2paper

# Pre-built binaries
# → GitHub Releases 링크
```

### 6. Quick Start
```bash
md2paper input.md
md2paper input.md -o output.pdf --theme academic
cat README.md | md2paper -o readme.pdf
```

### 7. Themes
빌트인 테마 4개 소개 테이블 (이름, 설명, 대상 독자).

### 8. CLI Reference
핵심 옵션 요약 테이블. 하단에 "See `md2paper --help` for full options" 안내.

### 9. Library API
```rust
use md2paper::{convert, Config, Theme};
// 간단 예제 + Config builder 예제
```

### 10. Comparison
| | md2paper | Pandoc+LaTeX | md-to-pdf | WeasyPrint |
비교표 (Runtime, Speed, Output Quality, Theme, CJK, Binary Size)

### 11. Roadmap
Phase 1 (✅ MVP), Phase 2 (Rich Features), Phase 3 (DX & Ecosystem) 간략 체크리스트.

### 12. Contributing
- 기여 환영 문구
- `DESIGN_SPEC.md` 링크 (아키텍처 참조용)
- `cargo test` 실행 방법

### 13. License
MIT

---

## README.ko.md 구성

`README.md`와 동일한 섹션 구조, 모든 텍스트를 한국어로 번역. 코드 블록은 그대로 유지.

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
| 헤더 스타일 | 태그라인 + 배지 + 핵심 가치 한 줄 |
| 언어 연결 | 최상단 언어 배너 (양방향 링크) |
| 콘텐츠 깊이 | 풍성하게 (비교표, 로드맵, Library API 포함) |
| 접근법 | B — README는 쇼케이스, 깊은 스펙은 DESIGN_SPEC.md로 링크 |
