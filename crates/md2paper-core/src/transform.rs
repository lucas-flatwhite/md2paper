use std::collections::HashMap;
use comrak::nodes::{AstNode, ListType, NodeValue};

/// Walk the AST and produce Typst markup string.
pub fn ast_to_typst<'a>(node: &'a AstNode<'a>) -> String {
    // First pass: collect all footnote definitions (name → rendered content)
    let footnotes = collect_footnote_defs(node);
    let mut out = String::new();
    render_node(node, &mut out, false, &footnotes);
    out
}

/// First-pass: build a map of footnote name → rendered body text.
fn collect_footnote_defs<'a>(root: &'a AstNode<'a>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for node in root.descendants() {
        let data = node.data.borrow();
        if let NodeValue::FootnoteDefinition(def) = &data.value {
            let name = def.name.clone();
            drop(data);
            let mut body = String::new();
            // Render children with an empty footnote map (no nested footnotes)
            let empty: HashMap<String, String> = HashMap::new();
            for child in node.children() {
                render_node(child, &mut body, false, &empty);
            }
            map.insert(name, body.trim().to_string());
        } else {
            drop(data);
        }
    }
    map
}

fn render_node<'a>(node: &'a AstNode<'a>, out: &mut String, inline: bool, footnotes: &HashMap<String, String>) {
    let data = node.data.borrow();
    match &data.value {
        NodeValue::Document => {
            drop(data);
            render_children(node, out, false, footnotes);
        }
        NodeValue::Heading(heading) => {
            let level = heading.level;
            drop(data);
            let prefix = "=".repeat(level as usize);
            out.push_str(&prefix);
            out.push(' ');
            render_children(node, out, true, footnotes);
            out.push('\n');
            out.push('\n');
        }
        NodeValue::Paragraph => {
            drop(data);
            render_children(node, out, true, footnotes);
            if !inline {
                out.push('\n');
                out.push('\n');
            }
        }
        NodeValue::Text(text) => {
            let s = text.clone();
            drop(data);
            out.push_str(&escape_typst(&s));
        }
        NodeValue::SoftBreak => {
            drop(data);
            out.push(' ');
        }
        NodeValue::LineBreak => {
            drop(data);
            out.push_str("\\\n");
        }
        NodeValue::Strong => {
            drop(data);
            out.push('*');
            render_children(node, out, true, footnotes);
            out.push('*');
        }
        NodeValue::Emph => {
            drop(data);
            out.push('_');
            render_children(node, out, true, footnotes);
            out.push('_');
        }
        NodeValue::Strikethrough => {
            drop(data);
            out.push_str("#strike[");
            render_children(node, out, true, footnotes);
            out.push(']');
        }
        NodeValue::Code(code) => {
            let literal = code.literal.clone();
            drop(data);
            out.push('`');
            out.push_str(&literal);
            out.push('`');
        }
        NodeValue::CodeBlock(cb) => {
            let info = cb.info.clone();
            let literal = cb.literal.clone();
            drop(data);
            let lang = info.split_whitespace().next().unwrap_or("").trim();
            if lang == "typst" {
                // Passthrough: emit raw Typst source directly without wrapping
                out.push_str(&literal);
                if !literal.ends_with('\n') {
                    out.push('\n');
                }
                out.push('\n');
            } else if lang.is_empty() {
                out.push_str("```\n");
                out.push_str(&literal);
                out.push_str("```\n\n");
            } else {
                out.push_str(&format!("```{}\n", lang));
                out.push_str(&literal);
                out.push_str("```\n\n");
            }
        }
        NodeValue::BlockQuote => {
            drop(data);
            out.push_str("#quote[\n");
            render_children(node, out, false, footnotes);
            out.push_str("]\n\n");
        }
        NodeValue::List(list) => {
            let list_type = list.list_type;
            drop(data);
            render_list(node, out, list_type, 0, footnotes);
            out.push('\n');
        }
        NodeValue::Item(_) => {
            drop(data);
            render_children(node, out, false, footnotes);
        }
        NodeValue::Link(link) => {
            let url = link.url.clone();
            drop(data);
            out.push_str(&format!("#link(\"{}\")[", escape_typst_str(&url)));
            render_children(node, out, true, footnotes);
            out.push(']');
        }
        NodeValue::Image(img) => {
            let url = img.url.clone();
            drop(data);
            let alt = collect_text(node);
            let alt_str = if alt.is_empty() {
                String::new()
            } else {
                format!(", alt: \"{}\"", escape_typst_str(&alt))
            };
            out.push_str(&format!("#image(\"{}\"{alt_str})\n", escape_typst_str(&url)));
        }
        NodeValue::ThematicBreak => {
            drop(data);
            out.push_str("#line(length: 100%)\n\n");
        }
        NodeValue::HtmlBlock(_) => {
            // Skip HTML blocks
            drop(data);
        }
        NodeValue::HtmlInline(_) => {
            // Skip inline HTML
            drop(data);
        }
        NodeValue::Table(_) => {
            drop(data);
            render_table(node, out, footnotes);
        }
        NodeValue::TableRow(_) | NodeValue::TableCell => {
            // Handled by render_table
            drop(data);
        }
        NodeValue::FootnoteDefinition(_) => {
            // Collected in the first pass; skip standalone rendering
            drop(data);
        }
        NodeValue::FootnoteReference(r) => {
            let name = r.name.clone();
            drop(data);
            // Look up the footnote definition content collected in the first pass
            if let Some(body) = footnotes.get(&name) {
                out.push_str(&format!("#footnote[{}]", body));
            } else {
                // Fallback: emit name if definition not found
                out.push_str(&format!("#footnote[{}]", escape_typst(&name)));
            }
        }
        NodeValue::Math(math) => {
            let dollar_math = math.dollar_math;
            let literal = math.literal.clone();
            drop(data);
            if dollar_math {
                // Block math ($$...$$)
                out.push_str(&format!("$ {} $\n\n", literal.trim()));
            } else {
                // Inline math ($...$)
                out.push_str(&format!("${}$", literal.trim()));
            }
        }
        NodeValue::TaskItem(checked) => {
            let is_checked = checked.is_some();
            drop(data);
            if is_checked {
                out.push_str("[x] ");
            } else {
                out.push_str("[ ] ");
            }
            render_children(node, out, true, footnotes);
        }
        _ => {
            drop(data);
            render_children(node, out, inline, footnotes);
        }
    }
}

fn render_children<'a>(node: &'a AstNode<'a>, out: &mut String, inline: bool, footnotes: &HashMap<String, String>) {
    for child in node.children() {
        render_node(child, out, inline, footnotes);
    }
}

fn render_list<'a>(node: &'a AstNode<'a>, out: &mut String, list_type: ListType, depth: usize, footnotes: &HashMap<String, String>) {
    let indent = "  ".repeat(depth);
    for item in node.children() {
        let data = item.data.borrow();
        if let NodeValue::Item(_) = &data.value {
            drop(data);
            let marker = match list_type {
                ListType::Bullet => "-".to_string(),
                ListType::Ordered => "+".to_string(),
            };
            out.push_str(&format!("{}{} ", indent, marker));
            // Render item children
            let mut first = true;
            for child in item.children() {
                let child_data = child.data.borrow();
                match &child_data.value {
                    NodeValue::Paragraph => {
                        drop(child_data);
                        if !first {
                            out.push_str(&format!("\n{}", " ".repeat(depth * 2 + 2)));
                        }
                        render_children(child, out, true, footnotes);
                    }
                    NodeValue::List(nested_list) => {
                        let nested_type = nested_list.list_type;
                        drop(child_data);
                        out.push('\n');
                        render_list(child, out, nested_type, depth + 1, footnotes);
                    }
                    _ => {
                        drop(child_data);
                        render_node(child, out, true, footnotes);
                    }
                }
                first = false;
            }
            out.push('\n');
        } else {
            drop(data);
        }
    }
}

fn render_table<'a>(node: &'a AstNode<'a>, out: &mut String, footnotes: &HashMap<String, String>) {
    // Collect rows
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut is_header: Vec<bool> = Vec::new();

    for row_node in node.children() {
        let row_data = row_node.data.borrow();
        if let NodeValue::TableRow(header) = &row_data.value {
            let header_flag = *header;
            drop(row_data);
            let mut cells: Vec<String> = Vec::new();
            for cell_node in row_node.children() {
                let cell_data = cell_node.data.borrow();
                if let NodeValue::TableCell = &cell_data.value {
                    drop(cell_data);
                    let mut cell_content = String::new();
                    render_children(cell_node, &mut cell_content, true, footnotes);
                    cells.push(cell_content.trim().to_string());
                } else {
                    drop(cell_data);
                }
            }
            rows.push(cells);
            is_header.push(header_flag);
        } else {
            drop(row_data);
        }
    }

    if rows.is_empty() {
        return;
    }

    let col_count = rows[0].len();
    // Build columns array: (1fr,) repeated col_count times
    let col_fracs = vec!["1fr"; col_count].join(", ");

    out.push_str(&format!("#table(\n  columns: ({}),", col_fracs));
    out.push_str("\n  stroke: 0.5pt,\n");

    // Header style
    for (i, row) in rows.iter().enumerate() {
        for cell in row.iter() {
            if is_header[i] {
                out.push_str(&format!("  table.cell(fill: rgb(\"#f3f4f6\"))[*{}*],\n", cell));
            } else {
                out.push_str(&format!("  [{}],\n", cell));
            }
        }
    }
    out.push_str(")\n\n");
}

/// Collect all text content from node recursively.
fn collect_text<'a>(node: &'a AstNode<'a>) -> String {
    let mut out = String::new();
    let data = node.data.borrow();
    if let NodeValue::Text(t) = &data.value {
        out.push_str(t);
    }
    drop(data);
    for child in node.children() {
        out.push_str(&collect_text(child));
    }
    out
}

/// Escape special Typst characters in body text.
fn escape_typst(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for ch in s.chars() {
        match ch {
            // Must be first: \ is the Typst escape character itself
            '\\' => out.push_str("\\\\"),
            '@' => out.push_str("\\@"),
            '#' => out.push_str("\\#"),
            '<' => out.push_str("\\<"),
            '>' => out.push_str("\\>"),
            // ~ creates a non-breaking space in Typst markup
            '~' => out.push_str("\\~"),
            // $ starts math mode
            '$' => out.push_str("\\$"),
            // ` starts inline/block code
            '`' => out.push_str("\\`"),
            _ => out.push(ch),
        }
    }
    out
}

/// Escape characters inside Typst string literals.
fn escape_typst_str(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}
