use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

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
    /// In-memory cache for downloaded remote images.
    http_cache: Mutex<HashMap<String, Bytes>>,
}

/// Returns true if the given rootless path looks like an HTTP(S) URL.
pub(crate) fn is_http_url(path: &Path) -> bool {
    let s = path.to_string_lossy();
    s.starts_with("https://") || s.starts_with("http://")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_http_url_detects_https() {
        assert!(is_http_url(Path::new("https://example.com/image.png")));
    }

    #[test]
    fn test_is_http_url_detects_http() {
        assert!(is_http_url(Path::new("http://example.com/logo.svg")));
    }

    #[test]
    fn test_is_http_url_rejects_relative_path() {
        assert!(!is_http_url(Path::new("images/photo.jpg")));
    }

    #[test]
    fn test_is_http_url_rejects_absolute_path() {
        assert!(!is_http_url(Path::new("/usr/local/images/photo.jpg")));
    }
}

/// Download a URL and return its bytes. Results are not cached here;
/// caching is handled by the caller.
fn fetch_url(url: &str) -> FileResult<Bytes> {
    ureq::get(url)
        .call()
        .map_err(|_| FileError::Other(Some(format!("failed to download {url}").into())))
        .and_then(|resp| {
            let mut buf = Vec::new();
            use std::io::Read;
            resp.into_reader()
                .read_to_end(&mut buf)
                .map_err(|_| FileError::Other(Some(format!("failed to read response from {url}").into())))?;
            Ok(Bytes::new(buf))
        })
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
            http_cache: Mutex::new(HashMap::new()),
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

        // Handle HTTP(S) URLs: download and cache in memory
        if is_http_url(rel) {
            let url = rel.to_string_lossy().into_owned();
            {
                let cache = self.http_cache.lock().unwrap();
                if let Some(cached) = cache.get(&url) {
                    return Ok(cached.clone());
                }
            }
            let bytes = fetch_url(&url)?;
            self.http_cache.lock().unwrap().insert(url, bytes.clone());
            return Ok(bytes);
        }

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
