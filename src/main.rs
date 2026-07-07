mod monitor;
mod metadata;
mod rule_sorter;
mod date_sorter;
mod file_ops;

use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::process;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use directories::UserDirs;
use notify::event::{AccessKind, AccessMode};
use notify::{Event, EventKind};

#[derive(Debug, Clone, Copy, PartialEq)]
enum SortMode {
    Rule,
    Date,
}

const DEBOUNCE_DURATION: Duration = Duration::from_secs(2);
const POLL_INTERVAL: Duration = Duration::from_millis(500);

fn main() {
    let mode = parse_args();
    let downloads = resolve_downloads_dir();
    let (tx, rx) = mpsc::channel::<Result<Event, notify::Error>>();

    let _watcher =
        monitor::start_watcher(&downloads, tx).expect("Failed to start filesystem watcher");

    println!("=== Smart File Organizer ===");
    println!("Mode:      {:?}", mode);
    println!("Watching:  {}", downloads.display());
    println!("Press Ctrl+C to stop.\n");

    let mut pending: HashMap<PathBuf, Instant> = HashMap::new();

    loop {
        match rx.recv_timeout(POLL_INTERVAL) {
            Ok(Ok(event)) => {
                let now = Instant::now();

                for path in &event.paths {
                    if !path.is_file() {
                        continue;
                    }

                    let is_close_write = matches!(
                        event.kind,
                        EventKind::Access(AccessKind::Close(AccessMode::Write))
                    );

                    if is_close_write {
                        pending.remove(path);
                        if path.exists() {
                            handle_new_file(&downloads, path, mode);
                        }
                    } else if matches!(
                        event.kind,
                        EventKind::Create(_) | EventKind::Modify(_)
                    ) {
                        pending.insert(path.clone(), now);
                    }
                }
            }
            Ok(Err(e)) => {
                eprintln!("Watch error: {e}");
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                let cutoff = Instant::now()
                    .checked_sub(DEBOUNCE_DURATION)
                    .unwrap_or(Instant::now());
                pending.retain(|path, last_seen| {
                    if *last_seen <= cutoff {
                        if path.exists() {
                            handle_new_file(&downloads, path, mode);
                        }
                        false
                    } else {
                        true
                    }
                });
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                println!("\nShutting down.");
                break;
            }
        }
    }
}

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

fn dirs_fallback_download() -> PathBuf {
    if let Ok(home) = env::var("HOME") {
        PathBuf::from(home).join("Downloads")
    } else {
        eprintln!("HOME not set — cannot determine download directory.");
        process::exit(1);
    }
}

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
