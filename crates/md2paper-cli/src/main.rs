mod watch;
mod preview;

use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Parser;

use md2paper_core::{Config, to_typst};
use md2paper_render::render_to_pdf_with_base;
use md2paper_theme::loader::load;

/// md2paper — Convert Markdown into beautifully typeset PDF.
#[derive(Parser, Debug)]
#[command(name = "md2paper", version, about)]
struct Cli {
    /// Input Markdown file(s). Use '-' for stdin.
    #[arg(value_name = "INPUT")]
    inputs: Vec<String>,

    /// Output PDF path [default: <input>.pdf]
    #[arg(short, long, value_name = "PATH")]
    output: Option<PathBuf>,

    /// Theme name or path to .toml file [default: default]
    #[arg(short, long, value_name = "THEME", default_value = "default")]
    theme: String,

    /// Override document title
    #[arg(long, value_name = "TEXT")]
    title: Option<String>,

    /// Override document author
    #[arg(long, value_name = "TEXT")]
    author: Option<String>,

    /// Override document date
    #[arg(long, value_name = "TEXT")]
    date: Option<String>,

    /// Override body font family
    #[arg(long, value_name = "FAMILY")]
    font: Option<String>,

    /// Override body font size (e.g. "11pt")
    #[arg(long, value_name = "SIZE")]
    font_size: Option<String>,

    /// Override line height (e.g. 1.6)
    #[arg(long, value_name = "RATIO")]
    line_height: Option<f64>,

    /// Override letter spacing (e.g. "0.5pt")
    #[arg(long, value_name = "SIZE")]
    letter_spacing: Option<String>,

    /// Paper format: a4, letter, legal [default: a4]
    #[arg(long, value_name = "FORMAT")]
    paper: Option<String>,

    /// Set all margins (e.g. "2.5cm")
    #[arg(long, value_name = "SIZE")]
    margin: Option<String>,

    /// Generate table of contents
    #[arg(long)]
    toc: bool,

    /// TOC depth [default: 3]
    #[arg(long, value_name = "N", default_value = "3")]
    toc_depth: u8,

    /// Generate cover page from front matter
    #[arg(long)]
    cover: bool,

    /// Output generated Typst markup instead of PDF
    #[arg(long)]
    emit_typst: bool,

    /// Dump Markdown AST as JSON
    #[arg(long)]
    dump_ast: bool,

    /// Base directory for resolving relative assets
    #[arg(long, value_name = "DIR")]
    base_dir: Option<PathBuf>,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Suppress all output except errors
    #[arg(short, long)]
    quiet: bool,

    /// Watch input files and recompile on change
    #[arg(long)]
    watch: bool,

    /// Start a local preview server with live PDF reload
    #[arg(long)]
    preview: bool,
}

/// Compile arguments — mirrors CLI overrides, cloneable for watch/preview loops.
struct CompileArgs {
    inputs: Vec<String>,
    base_dir: Option<PathBuf>,
    theme_name: String,
    title: Option<String>,
    author: Option<String>,
    date: Option<String>,
    font: Option<String>,
    font_size: Option<String>,
    line_height: Option<f64>,
    letter_spacing: Option<String>,
    paper: Option<String>,
    margin: Option<String>,
    toc: bool,
    toc_depth: u8,
    cover: bool,
}

fn build_compile_args(cli: &Cli) -> CompileArgs {
    CompileArgs {
        inputs: cli.inputs.clone(),
        base_dir: cli.base_dir.clone(),
        theme_name: cli.theme.clone(),
        title: cli.title.clone(),
        author: cli.author.clone(),
        date: cli.date.clone(),
        font: cli.font.clone(),
        font_size: cli.font_size.clone(),
        line_height: cli.line_height,
        letter_spacing: cli.letter_spacing.clone(),
        paper: cli.paper.clone(),
        margin: cli.margin.clone(),
        toc: cli.toc,
        toc_depth: cli.toc_depth,
        cover: cli.cover,
    }
}

/// Compile and return the PDF as bytes (does not write to disk).
fn compile_once_bytes(args: &CompileArgs) -> Result<Vec<u8>> {
    let (markdown, base_dir) = read_inputs_raw(&args.inputs, args.base_dir.as_deref())?;
    let theme = load(&args.theme_name)
        .with_context(|| format!("Failed to load theme '{}'", args.theme_name))?;
    let mut builder = Config::builder().theme(theme);
    if let Some(v) = &args.title { builder = builder.title(v.clone()); }
    if let Some(v) = &args.author { builder = builder.author(v.clone()); }
    if let Some(v) = &args.date { builder = builder.date(v.clone()); }
    if let Some(v) = &args.font { builder = builder.font_family(v.clone()); }
    if let Some(v) = &args.font_size { builder = builder.font_size(v.clone()); }
    if let Some(v) = args.line_height { builder = builder.line_height(v); }
    if let Some(v) = &args.letter_spacing { builder = builder.letter_spacing(v.clone()); }
    if let Some(v) = &args.paper { builder = builder.paper(v.clone()); }
    if let Some(v) = &args.margin { builder = builder.margin(v.clone()); }
    builder = builder.toc(args.toc).toc_depth(args.toc_depth).cover(args.cover);
    let config = builder.build();
    let typst_src = to_typst(&markdown, &config)
        .context("Failed to convert Markdown to Typst markup")?;
    render_to_pdf_with_base(&typst_src, &base_dir)
        .context("Failed to render PDF")
}

/// Compile and write to `out_path`.
fn compile_once(args: &CompileArgs, out_path: &Path) -> Result<()> {
    let bytes = compile_once_bytes(args)?;
    std::fs::write(out_path, &bytes)
        .with_context(|| format!("Failed to write {}", out_path.display()))
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Read all inputs into a single Markdown string
    let (markdown, base_dir) = read_inputs(&cli)?;

    // Build theme
    let theme = load(&cli.theme)
        .with_context(|| format!("Failed to load theme '{}'", cli.theme))?;

    if cli.verbose && !cli.quiet {
        eprintln!("Theme: {} v{}", theme.meta.name, theme.meta.version);
    }

    // Build config
    let mut builder = Config::builder().theme(theme);
    if let Some(v) = cli.title.clone() { builder = builder.title(v); }
    if let Some(v) = cli.author.clone() { builder = builder.author(v); }
    if let Some(v) = cli.date.clone() { builder = builder.date(v); }
    if let Some(v) = cli.font.clone() { builder = builder.font_family(v); }
    if let Some(v) = cli.font_size.clone() { builder = builder.font_size(v); }
    if let Some(v) = cli.line_height { builder = builder.line_height(v); }
    if let Some(v) = cli.letter_spacing.clone() { builder = builder.letter_spacing(v); }
    if let Some(v) = cli.paper.clone() { builder = builder.paper(v); }
    if let Some(v) = cli.margin.clone() { builder = builder.margin(v); }
    builder = builder
        .toc(cli.toc)
        .toc_depth(cli.toc_depth)
        .cover(cli.cover)
        .emit_typst(cli.emit_typst)
        .dump_ast(cli.dump_ast);

    let config = builder.build();

    // Dump AST and exit if requested
    if cli.dump_ast {
        print!("{}", md2paper_core::ast_as_debug_string(&markdown));
        return Ok(());
    }

    // Generate Typst markup
    let typst_src = to_typst(&markdown, &config)
        .context("Failed to convert Markdown to Typst markup")?;

    if cli.emit_typst {
        print!("{typst_src}");
        return Ok(());
    }

    if cli.verbose && !cli.quiet {
        eprintln!("Rendering PDF...");
    }

    // Render to PDF
    let pdf_bytes = render_to_pdf_with_base(&typst_src, &base_dir)
        .context("Failed to render PDF")?;

    // Write output
    let out_path = determine_output(&cli.inputs, cli.output.as_deref())?;
    std::fs::write(&out_path, &pdf_bytes)
        .with_context(|| format!("Failed to write output to {}", out_path.display()))?;

    if !cli.quiet {
        eprintln!("Written: {}", out_path.display());
    }

    // Watch / Preview modes
    if cli.preview || cli.watch {
        let input_paths: Vec<PathBuf> = cli.inputs.iter()
            .filter(|i| i.as_str() != "-")
            .map(PathBuf::from)
            .collect();
        if input_paths.is_empty() {
            anyhow::bail!("--watch/--preview requires at least one input file path (not stdin)");
        }
        let args = build_compile_args(&cli);
        let out = out_path.clone();

        if cli.preview {
            preview::run_preview(input_paths, move || compile_once_bytes(&args))?;
        } else {
            watch::run_watch_loop(input_paths, move || compile_once(&args, &out))?;
        }
    }

    Ok(())
}

fn read_inputs(cli: &Cli) -> Result<(String, PathBuf)> {
    read_inputs_raw(&cli.inputs, cli.base_dir.as_deref())
}

fn read_inputs_raw(inputs: &[String], explicit_base: Option<&Path>) -> Result<(String, PathBuf)> {
    if inputs.is_empty() || inputs == ["-"] {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).context("Failed to read stdin")?;
        let base = explicit_base.map(|p| p.to_path_buf())
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
        return Ok((buf, base));
    }

    let mut parts = Vec::new();
    let mut base_dir: Option<PathBuf> = explicit_base.map(|p| p.to_path_buf());

    for input in inputs {
        if input == "-" {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf).context("Failed to read stdin")?;
            parts.push(buf);
        } else {
            let path = Path::new(input);
            let content = std::fs::read_to_string(path)
                .with_context(|| format!("Failed to read input file: {input}"))?;
            if base_dir.is_none() {
                base_dir = path.parent().map(|p| {
                    if p.as_os_str().is_empty() {
                        PathBuf::from(".")
                    } else {
                        p.to_path_buf()
                    }
                });
            }
            parts.push(content);
        }
    }

    let markdown = parts.join("\n\n");
    let base = base_dir.unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    Ok((markdown, base))
}

fn determine_output(inputs: &[String], explicit: Option<&Path>) -> Result<PathBuf> {
    if let Some(p) = explicit {
        return Ok(p.to_path_buf());
    }
    if let Some(first) = inputs.first() {
        if first != "-" {
            let p = Path::new(first);
            let stem = p.file_stem().unwrap_or_default();
            let dir = p.parent().unwrap_or(Path::new("."));
            return Ok(dir.join(format!("{}.pdf", stem.to_string_lossy())));
        }
    }
    Ok(PathBuf::from("output.pdf"))
}
