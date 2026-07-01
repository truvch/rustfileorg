mod monitor;
mod metadata;
mod rule_sorter;
mod date_sorter;
mod file_ops;

use std::env;
use std::path::{Path, PathBuf};
use std::process;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use directories::UserDirs;
use notify::{Event, EventKind, RecursiveMode, Watcher};

/// Sorting strategy for organizing files.
#[derive(Debug, Clone, Copy, PartialEq)]
enum SortMode {
    Rule,
    Date,
}

fn main() {
    let mode = parse_args();
    let downloads = resolve_downloads_dir();
    let (tx, rx) = mpsc::channel::<Result<Event, notify::Error>>();

    let mut watcher = notify::recommended_watcher(move |res| {
        let _ = tx.send(res);
    })
    .expect("Failed to create filesystem watcher");

    watcher
        .watch(&downloads, RecursiveMode::NonRecursive)
        .expect("Failed to start watching the downloads directory");

    println!("=== Smart File Organizer ===");
    println!("Mode:      {:?}", mode);
    println!("Watching:  {}", downloads.display());
    println!("Press Ctrl+C to stop.\n");

    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                if !matches_event_kind(&event.kind) {
                    continue;
                }
                for path in event.paths {
                    if !path.is_file() {
                        continue;
                    }
                    thread::sleep(Duration::from_millis(100));

                    if !path.exists() || !path.is_file() {
                        continue;
                    }

                    handle_new_file(&downloads, &path, mode);
                }
            }
            Ok(Err(e)) => {
                eprintln!("Watch error: {e}");
            }
            Err(_) => {
                println!("\nShutting down.");
                break;
            }
        }
    }
}

/// Parses `--mode rule` or `--mode date` from CLI args. Defaults to `Rule`.
fn parse_args() -> SortMode {
    let args: Vec<String> = env::args().collect();
    for i in 0..args.len() {
        if args[i] == "--mode" {
            if let Some(val) = args.get(i + 1) {
                return match val.as_str() {
                    "date" => SortMode::Date,
                    _ => SortMode::Rule,
                };
            }
        }
    }
    SortMode::Rule
}

/// Resolves the user's Downloads directory following XDG conventions.
fn resolve_downloads_dir() -> PathBuf {
    UserDirs::new()
        .and_then(|u| u.download_dir().map(Path::to_path_buf))
        .unwrap_or_else(|| {
            let fallback = dirs_fallback_download();
            eprintln!(
                "XDG Downloads directory not found; falling back to {}",
                fallback.display()
            );
            fallback
        })
}

/// Fallback when XDG user dirs are unavailable.
fn dirs_fallback_download() -> PathBuf {
    if let Ok(home) = env::var("HOME") {
        PathBuf::from(home).join("Downloads")
    } else {
        eprintln!("HOME not set — cannot determine download directory.");
        process::exit(1);
    }
}

/// Returns `true` for event kinds that indicate a new or modified file.
fn matches_event_kind(kind: &EventKind) -> bool {
    matches!(
        kind,
        EventKind::Create(_) | EventKind::Modify(_)
    )
}

/// Routes a newly detected file through the full organisation pipeline.
fn handle_new_file(base_dir: &Path, file_path: &Path, mode: SortMode) {
    let meta = match metadata::read_metadata(file_path) {
        Some(m) => m,
        None => return,
    };

    let destination_dir = match mode {
        SortMode::Rule => rule_sorter::classify(base_dir, &meta),
        SortMode::Date => date_sorter::classify(base_dir, &meta),
    };

    if file_ops::ensure_dir(&destination_dir) {
        file_ops::move_file(file_path, &destination_dir, &meta);
    }
}
