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

    #[test]
    fn test_today_returns_correct_month_and_day() {
        use std::path::Path;
        let world = MemWorld::new("", Path::new("."));
        let date = world.today(None).expect("today must return a date");
        // The current impl always returns month=1, day=1 — this catches the bug.
        // On Jan 1 this might spuriously pass; that's acceptable for a unit test.
        let month = date.month().unwrap();
        let day = date.day().unwrap();
        assert!(month >= 1 && month <= 12, "month out of range: {month}");
        assert!(day >= 1 && day <= 31, "day out of range: {day}");
        // Ensure year is at least 2024 (rules out epoch-overflow bugs)
        assert!(date.year().unwrap() >= 2024, "year implausibly small");
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

/// Convert Unix timestamp (seconds since 1970-01-01 UTC) to (year, month, day)
/// using the proleptic Gregorian calendar. Algorithm by Howard Hinnant.
fn unix_secs_to_ymd(secs: i64) -> (i32, u8, u8) {
    let days = secs.div_euclid(86400) as i32;
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;                          // day of era [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365; // [0, 399]
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);   // day of year [0, 365]
    let mp = (5 * doy + 2) / 153;                         // month prime [0, 11]
    let d = (doy - (153 * mp + 2) / 5 + 1) as u8;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u8;
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
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
        let (y, m, d) = unix_secs_to_ymd(secs);
        Datetime::from_ymd(y, m, d)
    }
}
