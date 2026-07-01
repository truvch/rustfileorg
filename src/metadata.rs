//! # metadata
//!
//! Reads file-system metadata needed for sorting decisions.
//!
//! ## Struct: `FileMeta`
//!
//! ```ignore
//! pub struct FileMeta {
//!     /// Original file name (e.g. `"report.pdf"`).
//!     pub name: String,
//!     /// Lowercase extension **without** the dot, or empty string if none.
//!     /// Examples: `"pdf"`, `"jpg"`, `""`.
//!     pub extension: String,
//!     /// File size in bytes.
//!     pub size: u64,
//!     /// Last modification time as a chrono-style tuple `(year, month, day)` or
//!     /// a `SystemTime`. Choose whatever is easiest; the sorters only need
//!     /// year and month.
//!     pub modified: (i32, u32), // (year, month)
//! }
//! ```
//!
//! ## Required function
//!
//! ```ignore
//! pub fn read_metadata(file_path: &Path) -> Option<FileMeta>
//! ```
//!
//! - Uses `std::fs::metadata` to obtain size and modification time.
//! - Converts modification time into `(year, month)` using the `time` crate or
//!   manual UTC conversion from `SystemTime`.
//! - Returns `None` if the file is inaccessible (permissions, deleted between
//!   detection and processing, etc.).
//!
//! ## Notes for the implementer
//!
//! - Files without an extension should yield `extension: ""`.
//! - Hidden files (names starting with `.`) are **not** filtered here; that
//!   decision belongs to the sorter or the event loop.
//! - Be careful with `SystemTime::duration_since(UNIX_EPOCH)` on Linux — it
//!   may panic if the modification time is before 1970. Use `duration_since`
//!   only when the time is after the epoch, or prefer a crate like `filetime`.

use std::path::Path;

pub struct FileMeta {
    pub name: String,
    pub extension: String,
    pub size: u64,
    pub modified: (i32, u32),
}

pub fn read_metadata(_file_path: &Path) -> Option<FileMeta> {
    todo!("Extract file name, extension, size, and modification-year/month from std::fs::metadata")
}
