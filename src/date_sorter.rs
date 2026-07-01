//! # date_sorter
//!
//! Time-based classification into `YYYY/MM` subdirectories.
//!
//! ## Required function
//!
//! ```ignore
//! pub fn classify(base_dir: &Path, meta: &FileMeta) -> PathBuf
//! ```
//!
//! - `base_dir` is the watched directory (e.g. `~/Downloads`).
//! - Returns `base_dir.join("2026").join("07")`.
//! - Uses `meta.modified.0` (year) and `meta.modified.1` (month) to build
//!   the nested path.
//!
//! ## Notes for the implementer
//!
//! - Pad the month to two digits with leading zero (e.g. `02` for February).
//! - The caller (`file_ops`) is responsible for creating the full directory
//!   tree, so you only need to produce the `PathBuf`.

use std::path::{Path, PathBuf};

use crate::metadata::FileMeta;

pub fn classify(_base_dir: &Path, _meta: &FileMeta) -> PathBuf {
    todo!("Construct a YYYY/MM sub-path under base_dir from meta.modified")
}
