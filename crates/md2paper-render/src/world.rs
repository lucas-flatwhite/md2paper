use std::path::{Path, PathBuf};

use typst::foundations::{Bytes, Datetime};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};
use typst::diag::{FileError, FileResult};
use typst::syntax::{FileId, Source, VirtualPath};

/// A minimal Typst World implementation for in-memory compilation.
pub struct MemWorld {
    /// The source code to compile.
    source: Source,
    /// The standard library.
    library: LazyHash<Library>,
    /// Font metadata book.
    book: LazyHash<FontBook>,
    /// Loaded fonts.
    fonts: Vec<Font>,
    /// Base directory for resolving relative file paths (e.g., images).
    base_dir: PathBuf,
}

impl MemWorld {
    pub fn new(typst_src: &str, base_dir: &Path) -> Self {
        let main_id = FileId::new(None, VirtualPath::new("main.typ"));
        let source = Source::new(main_id, typst_src.to_string());

        let mut book = FontBook::new();
        let mut fonts = Vec::new();

        // Load embedded fonts from typst-assets
        for data in typst_assets::fonts() {
            let bytes = Bytes::new(data.to_vec());
            for font in Font::iter(bytes) {
                book.push(font.info().clone());
                fonts.push(font);
            }
        }

        Self {
            source,
            library: LazyHash::new(Library::builder().build()),
            book: LazyHash::new(book),
            fonts,
            base_dir: base_dir.to_path_buf(),
        }
    }
}

impl World for MemWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn main(&self) -> FileId {
        self.source.id()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.source.id() {
            Ok(self.source.clone())
        } else {
            Err(FileError::NotFound(
                id.vpath().as_rootless_path().to_path_buf(),
            ))
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        let rel = id.vpath().as_rootless_path();
        let full_path = self.base_dir.join(rel);
        std::fs::read(&full_path)
            .map(|v| Bytes::new(v))
            .map_err(|err| {
                if err.kind() == std::io::ErrorKind::NotFound {
                    FileError::NotFound(full_path)
                } else {
                    FileError::AccessDenied
                }
            })
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .ok()?;
        let secs = now.as_secs() as i64 + offset.unwrap_or(0) * 3600;
        let days = secs / 86400;
        // Approximate: epoch is 1970-01-01
        let year = 1970 + (days / 365) as i32;
        Datetime::from_ymd(year, 1, 1)
    }
}
