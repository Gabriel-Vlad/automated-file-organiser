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
    "ebooks",
    "executables",
    "databases",
    "others",
];

// -- Extensions to organize into their own directories
// given by the array's name --
pub const IMAGE_EXTENSIONS: &[&'static str] = &[
    "png", "jpg", "jpeg", "gif", "webp", "bmp", "ico", "svg", "tiff", "tif", "psd", "ai", "eps",
    "raw", "cr2", "nef", "orf", "sr2", "heic", "heif",
];
pub const DOCUMENT_EXTENSIONS: &[&'static str] = &[
    "pdf", "doc", "docx", "odt", "rtf", "txt", "md", "tex", "wpd", "xls", "xlsx", "ods", "csv",
    "log",
];
pub const EBOOK_EXTENSIONS: &[&'static str] = &["epub", "mobi", "azw3"];
pub const DATA_INTERCHANGE_FORMAT_EXTENSIONS: &[&'static str] =
    &["json", "xml", "toml", "yaml", "yml"];
pub const AUDIO_EXTENSIONS: &[&'static str] = &[
    "mp3", "aac", "ogg", "wma", "m4a", "flac", "wav", "alac", "aiff",
];
pub const ARCHIVE_EXTENSIONS: &[&'static str] = &[
    "zip", "rar", "7z", "tar", "gz", "bz2", "xz", "iso", "dmg", "img", "vhd", "vmdk", "jar",
];
pub const VIDEO_EXTENSIONS: &[&'static str] = &[
    "mp4", "mkv", "mov", "avi", "wmv", "flv", "webm", "m4v", "mpg", "mpeg", "3gp",
];
pub const SOURCE_CODE_EXTENSIONS: &[&'static str] = &[
    "rs",
    "py",
    "js",
    "ts",
    "java",
    "c",
    "cpp",
    "h",
    "hpp",
    "go",
    "php",
    "cs",
    "swift",
    "kt",
    "dart",
    "rb",
    "pl",
    "hs",
    "lua",
    "html",
    "css",
    "scss",
    "sass",
    "sh",
    "bat",
    "ps1",
    "sql",
    "dockerfile",
];
pub const PRESENTATION_EXTENSIONS: &[&'static str] = &["pptx", "ppt", "odp", "key"];
pub const FONT_EXTENSIONS: &[&'static str] = &["ttf", "otf", "woff", "woff2", "eot"];
pub const EXECUTABLE_EXTENSIONS: &[&'static str] = &[
    "exe", "msi", "com", "deb", "rpm", "appimage", "bin", "app", "pkg",
];
pub const DATABASE_EXTENSIONS: &[&'static str] = &["db", "sqlite", "sqlite3", "mdb", "accdb"];
