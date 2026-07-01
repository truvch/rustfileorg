//! # rule_sorter
//!
//! Extension-based classification into category subfolders.
//!
//! ## Categories
//!
//! | Category    | Extensions                                                     |
//! |-------------|----------------------------------------------------------------|
//! | `Images`    | jpg, jpeg, png, gif, bmp, webp, svg, ico, tiff, heic          |
//! | `Documents` | pdf, doc, docx, txt, md, odt, rtf, xls, xlsx, csv, ppt, pptx  |
//! | `Videos`    | mp4, mkv, avi, mov, wmv, flv, webm, m4v                       |
//! | `Music`     | mp3, wav, flac, ogg, aac, wma, m4a                            |
//! | `Archives`  | zip, tar, gz, rar, 7z, bz2, xz                                |
//! | `Code`      | rs, py, js, ts, html, css, java, c, cpp, h, go, json, xml, yaml, toml |
//! | `Other`     | everything else                                                |
//!
//! ## Required function
//!
//! ```ignore
//! pub fn classify(base_dir: &Path, meta: &FileMeta) -> PathBuf
//! ```
//!
//! - `base_dir` is the watched directory (e.g. `~/Downloads`).
//! - Returns `base_dir.join(category)`, e.g. `~/Downloads/Images`.
//! - The returned path is the **destination directory** — the caller will
//!   append the original file name later.
//!
//! ## Notes for the implementer
//!
//! - Use a `HashMap<&str, &str>` or a `match` block on `meta.extension`.
//! - The comparison should be case-insensitive.
//! - If the extension map or rules grow large, consider an external
//!   configuration file (JSON/TOML).

use std::path::{Path, PathBuf};

use crate::metadata::FileMeta;

pub fn classify(_base_dir: &Path, _meta: &FileMeta) -> PathBuf {
    todo!("Map file extension to a category folder and return base_dir.join(category)")
}
