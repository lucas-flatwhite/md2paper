use crate::model::Theme;

/// Generate Typst preamble (set/show rules) from a theme.
pub fn generate_preamble(theme: &Theme, doc_title: &str, doc_author: &str, doc_date: &str, cover: bool) -> String {
    let p = &theme.page;
    let f = &theme.font;
    let s = &theme.spacing;
    let c = &theme.color;
    let code = &theme.code;
    let hf = &theme.header_footer;

    let fallback_list = if f.fallback.is_empty() {
        String::new()
    } else {
        let joined = f.fallback.iter().map(|fb| format!("\"{}\"", fb)).collect::<Vec<_>>().join(", ");
        format!(", {}", joined)
    };

    let columns_setting = if p.columns > 1 {
        format!(", columns: {}", p.columns)
    } else {
        String::new()
    };

    // Header/footer generation
    let header = build_page_header(hf, doc_title);
    let footer = build_page_footer(hf);

    // Title/author/date block — full cover page or inline header
    let title_block = if cover && (!doc_title.is_empty() || !doc_author.is_empty()) {
        // Cover page: centred vertically and horizontally, followed by a page break
        let mut parts = Vec::new();
        if !doc_title.is_empty() {
            parts.push(format!("    text(size: 28pt, weight: \"bold\", \"{doc_title}\")"));
        }
        if !doc_author.is_empty() {
            parts.push(format!("    v(1em)\n    text(size: 14pt, \"{doc_author}\")"));
        }
        if !doc_date.is_empty() {
            parts.push(format!("    v(0.5em)\n    text(size: 11pt, style: \"italic\", \"{doc_date}\")"));
        }
        format!(
            "#align(center + horizon)[\n{}\n]\n#pagebreak()\n\n",
            parts.join("\n")
        )
    } else if !cover && (!doc_title.is_empty() || !doc_author.is_empty()) {
        // Inline title block at the top of content (original behaviour)
        let mut parts = Vec::new();
        if !doc_title.is_empty() {
            parts.push(format!("  align(center, text(size: 20pt, weight: \"bold\", \"{doc_title}\"))"));
        }
        if !doc_author.is_empty() {
            parts.push(format!("  align(center, text(size: 12pt, \"{doc_author}\"))"));
        }
        if !doc_date.is_empty() {
            parts.push(format!("  align(center, text(size: 10pt, style: \"italic\", \"{doc_date}\"))"));
        }
        format!("{}\n\n", parts.join("\n"))
    } else {
        String::new()
    };

    let code_block_rule = build_code_block_rule(code, f, c);

    format!(
        r#"#set page(
  paper: "{paper}",
  margin: (top: {mt}, bottom: {mb}, left: {ml}, right: {mr}){columns_setting},
  {header}
  {footer}
)
#set text(
  font: ("{body_family}"{fallback}),
  size: {body_size},
  weight: "{body_weight}",
  fill: rgb("{body_text}"),
  tracking: {letter_spacing},
)
#set par(
  leading: {line_height}em,
  spacing: {paragraph_spacing},
  justify: true,
)
#set raw(tab-size: 2)
#show heading: it => {{
  let sizes = (24pt, 18pt, 14pt, 12pt)
  let sz = if it.level <= 4 {{ sizes.at(it.level - 1) }} else {{ 11pt }}
  block(
    above: {heading_above},
    below: {heading_below},
    text(
      font: "{heading_family}",
      size: sz,
      weight: "{heading_weight}",
      fill: rgb("{heading_color}"),
      it.body,
    )
  )
}}
#show link: it => {{
  text(fill: rgb("{link_color}"), it)
}}
#show raw.where(block: false): it => {{
  box(
    fill: rgb("{code_bg}"),
    inset: (x: 3pt, y: 0pt),
    outset: (y: 3pt),
    radius: {border_radius},
    text(font: "{code_family}", size: {code_size}, fill: rgb("{code_fg}"), it),
  )
}}
{code_block_rule}
#show quote: it => {{
  block(
    inset: (left: 12pt),
    stroke: (left: 3pt + rgb("{bq_border}")),
    text(fill: rgb("{bq_text}"), style: "italic", it.body),
  )
}}
{title_block}"#,
        paper = p.paper,
        mt = p.margin_top,
        mb = p.margin_bottom,
        ml = p.margin_left,
        mr = p.margin_right,
        columns_setting = columns_setting,
        header = header,
        footer = footer,
        body_family = f.body_family,
        fallback = fallback_list,
        body_size = f.body_size,
        body_weight = f.body_weight,
        body_text = c.body_text,
        letter_spacing = s.letter_spacing,
        line_height = s.line_height,
        paragraph_spacing = s.paragraph_spacing,
        heading_above = s.heading_above,
        heading_below = s.heading_below,
        heading_family = f.heading_family,
        heading_weight = f.heading_weight,
        heading_color = c.heading,
        link_color = c.link,
        code_bg = c.code_background,
        border_radius = code.border_radius,
        code_family = f.code_family,
        code_size = f.code_size,
        code_fg = c.code_text,
        bq_border = c.blockquote_border,
        bq_text = c.blockquote_text,
        code_block_rule = code_block_rule,
        title_block = title_block,
    )
}

/// Build the `#show raw.where(block: true)` rule.
/// When `line_numbers` is enabled, iterates `it.lines` to render a line-numbered grid.
/// Otherwise emits a plain container without overriding token fill colors.
fn build_code_block_rule(
    code: &crate::model::CodeSection,
    f: &crate::model::FontSection,
    c: &crate::model::ColorSection,
) -> String {
    if code.line_numbers {
        format!(
            r##"#show raw.where(block: true): it => {{
  block(
    fill: rgb("{code_bg}"),
    radius: {border_radius},
    width: 100%,
    clip: true,
    grid(
      inset: (x: 10pt, y: 8pt),
      columns: (auto, 1fr),
      column-gutter: 0.6em,
      row-gutter: 0pt,
      ..it.lines.enumerate().map(((i, line)) => (
        text(font: "{code_family}", size: {code_size}, fill: rgb("#888888"), str(i + 1)),
        text(font: "{code_family}", size: {code_size}, line.body),
      )).flatten(),
    ),
  )
}}"##,
            code_bg = c.code_background,
            border_radius = code.border_radius,
            code_family = f.code_family,
            code_size = f.code_size,
        )
    } else {
        format!(
            r#"#show raw.where(block: true): it => {{
  block(
    fill: rgb("{code_bg}"),
    inset: 10pt,
    radius: {border_radius},
    width: 100%,
    text(font: "{code_family}", size: {code_size}, it),
  )
}}"#,
            code_bg = c.code_background,
            border_radius = code.border_radius,
            code_family = f.code_family,
            code_size = f.code_size,
        )
    }
}

fn build_page_header(hf: &crate::model::HeaderFooterSection, title: &str) -> String {
    let right = hf.header_right.replace("{title}", title);
    let left = hf.header_left.replace("{title}", title);
    let center = hf.header_center.replace("{title}", title);

    if left.is_empty() && center.is_empty() && right.is_empty() {
        return String::new();
    }

    format!(
        r#"header: grid(
    columns: (1fr, 1fr, 1fr),
    align(left, text(size: 9pt, "{left}")),
    align(center, text(size: 9pt, "{center}")),
    align(right, text(size: 9pt, "{right}")),
  ),"#,
        left = left,
        center = center,
        right = right,
    )
}

fn build_page_footer(hf: &crate::model::HeaderFooterSection) -> String {
    let left = &hf.footer_left;
    let right = &hf.footer_right;
    let center = &hf.footer_center;

    if left.is_empty() && center.is_empty() && right.is_empty() {
        return String::new();
    }

    let center_expr = if center.contains("{page}") {
        "align(center, text(size: 9pt, str(counter(page).display())))".to_string()
    } else if center.is_empty() {
        format!("align(center, text(size: 9pt, \"{center}\"))")
    } else {
        format!("align(center, text(size: 9pt, \"{center}\"))")
    };

    format!(
        r#"footer: context grid(
    columns: (1fr, 1fr, 1fr),
    align(left, text(size: 9pt, "{left}")),
    {center_expr},
    align(right, text(size: 9pt, "{right}")),
  ),"#,
        left = left,
        center_expr = center_expr,
        right = right,
    )
}
