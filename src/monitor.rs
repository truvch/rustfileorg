use std::path::Path;
use std::sync::mpsc::Sender;

use notify::{Event, RecommendedWatcher, RecursiveMode, Result as NotifyResult, Watcher};

pub fn start_watcher(
    dir: &Path,
    tx: Sender<NotifyResult<Event>>,
) -> NotifyResult<RecommendedWatcher> {
    let mut watcher = notify::recommended_watcher(move |res| {
        let _ = tx.send(res);
    })?;

    watcher.watch(dir, RecursiveMode::NonRecursive)?;

    Ok(watcher)
}
