use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Category {
    Images,
    Documents,
    Video,
    Audio,
    Code,
    Archives,
    Unknown
}

impl Category {
    pub fn from_extension(ext: &str) -> Category {
        match ext.to_lowercase().as_str() {
            "jpg" | "jpeg" | "png" | "gif" | "svg" | "webp" => Category::Images,
            "pdf" | "docx" | "doc" | "txt" | "xlsx" | "pptx" => Category::Documents,
            "mp4" | "mov" | "avi" | "mkv" => Category::Video,
            "mp3" | "wav" | "flac" | "aac" => Category::Audio,
            "rs" | "py" | "js" | "ts" | "html" | "css" | "json" => Category::Code,
            "zip" | "tar" | "gz" | "rar" => Category::Archives,
            _ => Category::Unknown,
        }
    }

    pub fn folder_name(&self) -> &str {
        match self {
            Category::Images => "images",
            Category::Documents => "documents",
            Category::Video => "video",
            Category::Audio => "audio",
            Category::Code => "code",
            Category::Archives => "archives",
            Category::Unknown => "unknown"
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.folder_name())
    }
}
