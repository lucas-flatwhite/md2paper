use md2paper_theme::model::Theme;

/// Runtime configuration for a conversion.
#[derive(Debug, Clone)]
pub struct Config {
    pub theme: Theme,
    pub title: Option<String>,
    pub author: Option<String>,
    pub date: Option<String>,
    pub toc: bool,
    pub toc_depth: u8,
    pub cover: bool,
    pub emit_typst: bool,
    pub dump_ast: bool,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

#[derive(Default)]
pub struct ConfigBuilder {
    theme: Option<Theme>,
    title: Option<String>,
    author: Option<String>,
    date: Option<String>,
    toc: bool,
    toc_depth: u8,
    cover: bool,
    emit_typst: bool,
    dump_ast: bool,
    font_family: Option<String>,
    font_size: Option<String>,
    line_height: Option<f64>,
    letter_spacing: Option<String>,
    paper: Option<String>,
    margin: Option<String>,
}

impl ConfigBuilder {
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme);
        self
    }
    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.title = Some(v.into());
        self
    }
    pub fn author(mut self, v: impl Into<String>) -> Self {
        self.author = Some(v.into());
        self
    }
    pub fn date(mut self, v: impl Into<String>) -> Self {
        self.date = Some(v.into());
        self
    }
    pub fn font_family(mut self, v: impl Into<String>) -> Self {
        self.font_family = Some(v.into());
        self
    }
    pub fn font_size(mut self, v: impl Into<String>) -> Self {
        self.font_size = Some(v.into());
        self
    }
    pub fn line_height(mut self, v: f64) -> Self {
        self.line_height = Some(v);
        self
    }
    pub fn letter_spacing(mut self, v: impl Into<String>) -> Self {
        self.letter_spacing = Some(v.into());
        self
    }
    pub fn paper(mut self, v: impl Into<String>) -> Self {
        self.paper = Some(v.into());
        self
    }
    pub fn margin(mut self, v: impl Into<String>) -> Self {
        self.margin = Some(v.into());
        self
    }
    pub fn toc(mut self, v: bool) -> Self {
        self.toc = v;
        self
    }
    pub fn toc_depth(mut self, v: u8) -> Self {
        self.toc_depth = v;
        self
    }
    pub fn cover(mut self, v: bool) -> Self {
        self.cover = v;
        self
    }
    pub fn emit_typst(mut self, v: bool) -> Self {
        self.emit_typst = v;
        self
    }
    pub fn dump_ast(mut self, v: bool) -> Self {
        self.dump_ast = v;
        self
    }

    pub fn build(self) -> Config {
        let mut theme = self.theme.unwrap_or_else(|| {
            md2paper_theme::loader::load_builtin("default").expect("default theme must always load")
        });

        // Apply builder overrides
        if let Some(v) = self.font_family {
            theme.font.body_family = v;
        }
        if let Some(v) = self.font_size {
            theme.font.body_size = v;
        }
        if let Some(v) = self.line_height {
            theme.spacing.line_height = v;
        }
        if let Some(v) = self.letter_spacing {
            theme.spacing.letter_spacing = v;
        }
        if let Some(v) = self.paper {
            theme.page.paper = v;
        }
        if let Some(v) = self.margin {
            theme.page.margin_top = v.clone();
            theme.page.margin_bottom = v.clone();
            theme.page.margin_left = v.clone();
            theme.page.margin_right = v;
        }

        Config {
            theme,
            title: self.title,
            author: self.author,
            date: self.date,
            toc: self.toc,
            toc_depth: if self.toc_depth == 0 { 3 } else { self.toc_depth },
            cover: self.cover,
            emit_typst: self.emit_typst,
            dump_ast: self.dump_ast,
        }
    }
}

/// Apply front matter overrides onto a Config.
pub fn apply_front_matter(config: &mut Config, fm: &crate::parser::FrontMatter) {
    if let Some(t) = &fm.title {
        if config.title.is_none() {
            config.title = Some(t.clone());
        }
    }
    if let Some(a) = &fm.author {
        if config.author.is_none() {
            config.author = Some(a.clone());
        }
    }
    if let Some(d) = &fm.date {
        if config.date.is_none() {
            config.date = Some(d.clone());
        }
    }
    if let Some(font_override) = &fm.font {
        if let Some(v) = &font_override.body_family {
            config.theme.font.body_family = v.clone();
        }
        if let Some(v) = &font_override.body_size {
            config.theme.font.body_size = v.clone();
        }
        if let Some(v) = &font_override.code_family {
            config.theme.font.code_family = v.clone();
        }
        if let Some(v) = &font_override.code_size {
            config.theme.font.code_size = v.clone();
        }
    }
    if let Some(spacing_override) = &fm.spacing {
        if let Some(v) = spacing_override.line_height {
            config.theme.spacing.line_height = v;
        }
        if let Some(v) = &spacing_override.paragraph_spacing {
            config.theme.spacing.paragraph_spacing = v.clone();
        }
        if let Some(v) = &spacing_override.letter_spacing {
            config.theme.spacing.letter_spacing = v.clone();
        }
    }
    if let Some(page_override) = &fm.page {
        if let Some(v) = &page_override.paper {
            config.theme.page.paper = v.clone();
        }
        if let Some(v) = &page_override.margin_top {
            config.theme.page.margin_top = v.clone();
        }
        if let Some(v) = &page_override.margin_bottom {
            config.theme.page.margin_bottom = v.clone();
        }
        if let Some(v) = &page_override.margin_left {
            config.theme.page.margin_left = v.clone();
        }
        if let Some(v) = &page_override.margin_right {
            config.theme.page.margin_right = v.clone();
        }
        if let Some(v) = page_override.columns {
            config.theme.page.columns = v;
        }
    }
}
