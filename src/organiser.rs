use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};
use crate::category::Category;
use crate::error::NeatlyError;

pub struct FileEntry {
    pub path: PathBuf,
    pub category: Category
}
#[derive(Serialize, Deserialize)]
pub struct LogEntry {
    pub from: String,
    pub to: String
}

const LOG_FILE: &str = ".neatly_log.json";

pub fn scan(dir: &Path) -> Result<Vec<FileEntry>, NeatlyError> {
    // validate it is a directory
    if !dir.is_dir() {
        return Err(NeatlyError::InvalidDirectory(dir.to_string_lossy().to_string()));
    }

    let entries = fs::read_dir(dir)?
    .filter_map(|e| e.ok()) // skip unreadable entries
    .filter(|e| e.path().is_file()) // skip subdirectories
    .filter(|e| { // skip hidden files
        e.path()
        .file_name()
        .and_then(|n| n.to_str())
        .map(|n| !n.starts_with('.'))
        .unwrap_or(false)
    })
    .map(|e| {
        let path = e.path();
        let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    let category = Category::from_extension(ext);
    FileEntry{ path, category}
    })
    .collect();

    Ok(entries)
}

pub fn preview(entries: &[FileEntry]) {
    if entries.is_empty() {
        println!("Nothing to organise.");
        return;
    }

    println!("\nPreview - no files will be moved:\n");
    for entry in entries {
        let filename = entry.path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");
    println!(" {} -> {}/", filename, entry.category.folder_name());
    }

    println!("\n{} files(s) would be moved.", entries.len());
}

pub fn organise(dir: &Path, entries: &[FileEntry]) -> Result<(), NeatlyError> {
    let mut log: Vec<LogEntry> = Vec::new();

    for entry in entries {
        let category_dir = dir.join(entry.category.folder_name());

        if !category_dir.exists() {
            fs::create_dir_all(&category_dir)?;
        }

        let filename = entry.path
        .file_name()
        .ok_or_else(|| NeatlyError::Io(
            std::io::Error::new(std::io::ErrorKind::Other, "Invalid filename")
        ))?;

        let destination = category_dir.join(filename);

        // record the move before doing it
        log.push(LogEntry { 
            from: entry.path.to_string_lossy().to_string(), 
            to: destination.to_string_lossy().to_string()
         });

         fs::rename(&entry.path, &destination)?;

         println!(" {} -> {}/", filename.to_string_lossy(), entry.category.folder_name());

    }

    // write undo log
    let log_path = dir.join(LOG_FILE);
    let log_contents = serde_json::to_string_pretty(&log)
    .map_err(|e| NeatlyError::UndoFailed(e.to_string()))?;

    fs::write(log_path, log_contents)?;

    println!("\n{} file(s) organised.", entries.len());

    Ok(())
}

pub fn undo(dir: &Path) -> Result<(), NeatlyError> {
    let log_path = dir.join(LOG_FILE);

    if !log_path.exists() {
        return Err(NeatlyError::UndoFailed("No undo log found. has neatly been run here?".to_string()));
    }

    let contents = fs::read_to_string(&log_path)?;
    let log: Vec<LogEntry> = serde_json::from_str(&contents)
    .map_err(|e| NeatlyError::UndoFailed(e.to_string()))?;

    for entry in &log {
        fs::rename(&entry.to, &entry.from)?;
        println!(" Restored: {}", entry.from);
    }

    let category_dirs = log.iter()
    .map(|e| {
        Path::new(&e.from)
        .parent()
        .map(|p| p.to_path_buf())
    })
    .filter_map(|p| p)
    .collect::<std::collections::HashSet<_>>();

    for folder in category_dirs {
        if folder != dir {
            let _ = fs::remove_dir(&dir); // only removes if empty
        }
    }

    fs::remove_file(&log_path)?;
    println!("\n{} file(s) restored.", log.len());
    
    Ok(())
}