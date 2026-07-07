use std::path::{Path, PathBuf};

use crate::metadata::FileMeta;

pub fn classify(base_dir: &Path, meta: &FileMeta) -> PathBuf {
    let category = ext_to_category(&meta.extension);
    base_dir.join(category)
}

fn ext_to_category(ext: &str) -> &'static str {
    match ext {
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg" | "ico" | "tiff" | "heic"
        | "heif" | "raw" | "cr2" | "nef" | "orf" | "sr2" => "Images",

        "pdf" | "doc" | "docx" | "docm" | "txt" | "md" | "odt" | "rtf" | "xls" | "xlsx"
        | "csv" | "ppt" | "pptx" | "ppsx" | "pages" | "numbers" | "key" | "epub" | "mobi"
        | "tex" | "log" => "Documents",

        "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm" | "m4v" | "mpg" | "mpeg"
        | "3gp" | "ogv" | "ts" => "Videos",

        "mp3" | "wav" | "flac" | "ogg" | "aac" | "wma" | "m4a" | "opus" | "aiff" | "alac"
        | "ape" | "wv" => "Music",

        "zip" | "tar" | "gz" | "rar" | "7z" | "bz2" | "xz" | "tgz" | "zst" | "lz" | "lz4"
        | "lzma" | "lzo" | "arj" | "cab" | "deb" | "rpm" | "iso" | "dmg" | "pkg" => "Archives",

        "rs" | "py" | "js" | "ts" | "jsx" | "tsx" | "html" | "htm" | "css" | "scss" | "sass"
        | "less" | "java" | "c" | "cpp" | "cc" | "cxx" | "h" | "hpp" | "go" | "json" | "xml"
        | "yaml" | "yml" | "toml" | "ini" | "cfg" | "sh" | "bash" | "zsh" | "fish" | "ps1"
        | "bat" | "cmd" | "rb" | "php" | "swift" | "kt" | "kts" | "scala" | "clj" | "cljs"
        | "edn" | "lua" | "r" | "m" | "mm" | "sql" | "graphql" | "proto" | "dockerfile"
        | "lock" | "gradle" | "sbt" => "Code",

        "exe" | "msi" | "apk" | "app" | "appimage" | "flatpakref" | "snap" => "Programs",

        "ttf" | "otf" | "woff" | "woff2" | "eot" => "Fonts",

        _ => "Others",
    }
}
