# rustfileorg
a file organizer for linux built on rust

| Module | Responsibility | What to do |
| :--- | :--- | :--- |
| `main.rs` | System initialization and orchestration. | Set up the Cargo project, resolve the target directory (e.g., Downloads), run the main event loop, and connect all module interfaces together. |
| `monitor.rs` | Real-time directory monitoring. | Use the `notify` crate to watch the target directory and emit events whenever a new file is completely written to disk. |
| `metadata.rs` | File metadata extraction. | Use `std::fs::metadata` to extract the file's extension, file size, and modification/creation dates to pass to the sorters. |
| `rule_sorter.rs` | Rule-based sorting logic. | Evaluate the file extension and map it to predefined categorical subfolders (Documents, Images, Videos, Archives, Others). |
| `date_sorter.rs` | Date-based sorting logic. | Evaluate the file's modification date and determine the destination path in a chronological `YYYY/MM` folder structure. |
| `file_ops.rs` | File operations and logging. | Create missing subdirectories using `std::fs::create_dir_all`, safely move the file to its new path, and record the action in a log file/terminal. |