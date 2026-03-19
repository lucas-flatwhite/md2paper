use anyhow::{Context, Result, bail};
use std::path::Path;

use crate::model::Theme;

const DEFAULT_THEME: &str = include_str!("../../../themes/default.toml");
const ACADEMIC_THEME: &str = include_str!("../../../themes/academic.toml");
const MINIMAL_THEME: &str = include_str!("../../../themes/minimal.toml");
const NEWSPAPER_THEME: &str = include_str!("../../../themes/newspaper.toml");

pub fn load_builtin(name: &str) -> Result<Theme> {
    let src = match name {
        "default" => DEFAULT_THEME,
        "academic" => ACADEMIC_THEME,
        "minimal" => MINIMAL_THEME,
        "newspaper" => NEWSPAPER_THEME,
        other => bail!("Unknown built-in theme: '{other}'. Available: default, academic, minimal, newspaper"),
    };
    parse_toml(src, name)
}

pub fn load_file(path: &Path) -> Result<Theme> {
    let src = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read theme file: {}", path.display()))?;
    parse_toml(&src, &path.display().to_string())
}

/// Load theme by name or path. If the value looks like a path (contains '/' or ends with '.toml')
/// treat it as a file path, otherwise load as built-in.
pub fn load(theme_value: &str) -> Result<Theme> {
    if theme_value.ends_with(".toml") || theme_value.contains('/') || theme_value.contains('\\') {
        load_file(Path::new(theme_value))
    } else {
        load_builtin(theme_value)
    }
}

fn parse_toml(src: &str, label: &str) -> Result<Theme> {
    toml::from_str(src).with_context(|| format!("Failed to parse theme TOML '{label}'"))
}
