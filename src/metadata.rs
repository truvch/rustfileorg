//! # metadata
//!
//! Reads file-system metadata needed for sorting decisions.
//!
//! Produces the `FileMeta` struct consumed by `rule_sorter::classify`
//! and `date_sorter::classify`.

use std::fs;
use std::path::Path;

use chrono::{DateTime, Datelike, Utc};

/// Metadata extracted from a file, used by the sorting modules.
pub struct FileMeta {
    /// Original file name (e.g. `"report.pdf"`).
    pub name: String,
    /// Lowercase extension **without** the dot, or empty string if none.
    /// Examples: `"pdf"`, `"jpg"`, `""`.
    pub extension: String,
    /// File size in bytes.
    pub size: u64,
    /// Last modification time as `(year, month)`.
    pub modified: (i32, u32),
}

/// Reads metadata for `file_path`.
///
/// Returns `None` if the file is inaccessible — permissions, deleted
/// between detection and processing, or a non-UTF-8 name — rather than
/// panicking, so the caller can just skip the file and move on.
pub fn read_metadata(file_path: &Path) -> Option<FileMeta> {
    let fs_meta = fs::metadata(file_path).ok()?;

    let name = file_path.file_name()?.to_str()?.to_string();

    let extension = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_default();

    let size = fs_meta.len();

    let modified_time = fs_meta.modified().ok()?;
    let datetime: DateTime<Utc> = modified_time.into();
    let modified = (datetime.year(), datetime.month());

    Some(FileMeta {
        name,
        extension,
        size,
        modified,
    })
}