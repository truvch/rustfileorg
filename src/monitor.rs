//! # monitor
//!
//! Filesystem event monitoring using Linux inotify (`notify` crate).
//!
//! ## Implementation plan
//!
//! 1. Create a recommended watcher via `notify::recommended_watcher`.
//!    The watcher should forward all raw events through an `mpsc::Sender`.
//!
//! 2. Call `watcher.watch(dir, RecursiveMode::NonRecursive)` to start
//!    listening on the target directory.
//!
//! 3. Return the watcher handle. The caller (`main.rs`) owns the receiver
//!    side of the channel and runs the event loop.
//!
//! 4. (Optional) Expose a helper to configure `RecursiveMode` and to
//!    unwatch a path.
//!
//! ## Signature required
//!
//! ```ignore
//! pub fn start_watcher(
//!     dir: &Path,
//!     tx: Sender<Result<Event, notify::Error>>,
//! ) -> notify::Result<RecommendedWatcher>
//! ```
//!
//! ## Future enhancements
//!
//! - Debounce rapid-fire events (e.g. browsers writing temporary files).
//! - Filter by file extension so irrelevant files are ignored early.
//! - Watch multiple directories simultaneously.

use std::path::Path;
use std::sync::mpsc::Sender;

use notify::{Event, RecommendedWatcher, Result as NotifyResult};

pub fn start_watcher(
    _dir: &Path,
    _tx: Sender<NotifyResult<Event>>,
) -> NotifyResult<RecommendedWatcher> {
    todo!("Implement inotify watcher initialisation using the notify crate")
}
