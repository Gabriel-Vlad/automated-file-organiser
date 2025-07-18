// -- An array containing directory names
// to be ignored from traversing and modifying --
pub const SPECIAL_FOLDERS: &[&'static str] = &[
    "images",
    "documents",
    "archives",
    "data_interchange",
    "audio",
    "video",
    "source_code",
    "presentations",
    "fonts",
    "others",
];

// -- Extensions to organize into their own directories
// given by the array's name --
pub const IMAGE_EXTENSIONS: &[&'static str] = &["png", "jpg", "jpeg", "gif", "webp", "bmp", "ico"];
pub const DOCUMENT_EXTENSIONS: &[&'static str] = &["md", "rtf", "log", "txt", "odt", "html", "pdf"];
pub const DATA_INTERCHANGE_FORMAT_EXTENSIONS: &[&'static str] = &["json", "csv", "xml", "toml"];
pub const AUDIO_EXTENSIONS: &[&'static str] = &["mp3", "aac", "ogg", "wma"];
pub const ARCHIVES_EXTENSIONS: &[&'static str] = &["zip", "rar", "7z", "tar", "gzip", "iso", "jar"];
pub const VIDEO_EXTENSIONS: &[&'static str] = &["mp4", "mov", "avi", "mkv", "wmv", "flv", "webm"];
pub const SOURCE_CODE_EXTENSIONS: &[&'static str] = &[
    "rs", "py", "js", "ts", "java", "c", "cpp", "go", "php", "cs", "sh", "bat",
];
pub const PRESENTATION_EXTENSIONS: &[&'static str] = &["pptx", "ppt", "odp", "key"];
pub const FONT_EXTENSIONS: &[&'static str] = &["ttf", "otf", "woff", "woff2"];
