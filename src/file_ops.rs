use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use directories::ProjectDirs;

use crate::metadata::FileMeta;

const MAX_CONFLICT_RETRIES: u32 = 999;

pub fn ensure_dir(path: &Path) -> bool {
    fs::create_dir_all(path).is_ok()
}

pub fn move_file(src: &Path, dest_dir: &Path, meta: &FileMeta) {
    let dest = match resolve_unique_path(dest_dir, &meta.name) {
        Some(d) => d,
        None => {
            eprintln!(
                "Could not resolve unique destination for {} in {}",
                src.display(),
                dest_dir.display()
            );
            return;
        }
    };

    match fs::rename(src, &dest) {
        Ok(()) => {
            println!("MOVED: {} -> {}", src.display(), dest.display());
            log_operation(src, &dest);
        }
        Err(e) => {
            eprintln!("Failed to move {}: {}", src.display(), e);
        }
    }
}

fn resolve_unique_path(dest_dir: &Path, name: &str) -> Option<PathBuf> {
    let candidate = dest_dir.join(name);
    if !candidate.exists() {
        return Some(candidate);
    }

    let (stem, ext) = split_stem_ext(name);

    for i in 1..=MAX_CONFLICT_RETRIES {
        let new_name = format!("{} ({}){}", stem, i, ext);
        let candidate = dest_dir.join(&new_name);
        if !candidate.exists() {
            return Some(candidate);
        }
    }

    let ts = timestamp_suffix();
    let fallback = format!("{} ({}){}", stem, ts, ext);
    Some(dest_dir.join(fallback))
}

fn split_stem_ext(name: &str) -> (&str, &str) {
    match name.rfind('.') {
        Some(pos) if pos > 0 => (&name[..pos], &name[pos..]),
        Some(_) => (name, ""),
        None => (name, ""),
    }
}

fn timestamp_suffix() -> String {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}

pub fn log_operation(src: &Path, dest: &Path) {
    let log_dir = log_directory();
    if let Err(e) = fs::create_dir_all(&log_dir) {
        eprintln!("Failed to create log directory {}: {}", log_dir.display(), e);
        return;
    }

    let log_path = log_dir.join("organizer.log");
    let entry = format_log_entry(src, dest);

    match fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
    {
        Ok(mut file) => {
            if let Err(e) = file.write_all(entry.as_bytes()) {
                eprintln!("Failed to write log entry: {}", e);
            }
        }
        Err(e) => eprintln!("Failed to open log file {}: {}", log_path.display(), e),
    }
}

fn log_directory() -> PathBuf {
    ProjectDirs::from("", "", "smart-file-organizer")
        .map(|d| d.data_dir().to_path_buf())
        .unwrap_or_else(|| {
            let mut fallback = dirs_fallback_data();
            fallback.push("smart-file-organizer");
            fallback
        })
}

fn dirs_fallback_data() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".local").join("share")
    } else if let Ok(appdata) = std::env::var("APPDATA") {
        PathBuf::from(appdata)
    } else {
        PathBuf::from(".")
    }
}

fn format_log_entry(src: &Path, dest: &Path) -> String {
    let ts = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    format!("[{}] MOVED: {} -> {}\n", ts, src.display(), dest.display())
}
