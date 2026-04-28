use std::fmt;

#[derive(Debug)]
pub enum NeatlyError {
    Io(std::io::Error),
    InvalidDirectory(String),
    UndoFailed(String)
}

impl fmt::Display for NeatlyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NeatlyError::Io(e) => write!(f, "IO error: {}", e),
            NeatlyError::InvalidDirectory(path) => write!(f, "Invalid directory: {}", path),
            NeatlyError::UndoFailed(reason) => write!(f, "Undo failed: {}", reason)
        }
    }
}

impl std::error::Error for NeatlyError {}

impl  From<std::io::Error> for NeatlyError {
    fn from(e: std::io::Error) -> Self {
        NeatlyError::Io(e)
    }
}
impl From<serde_json::Error> for NeatlyError {
    fn from(e: serde_json::Error) -> Self {
        NeatlyError::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    }
}