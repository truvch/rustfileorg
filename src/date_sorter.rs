use std::path::{Path, PathBuf};

use crate::metadata::FileMeta;

pub fn classify(base_dir: &Path, meta: &FileMeta) -> PathBuf {
    let (year, month) = meta.modified;
    base_dir.join(year.to_string()).join(format!("{:02}", month))
}
