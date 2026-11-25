use std::fs;
use std::path::{Path, PathBuf};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FsError {
    #[error("Invalid target folder path")]
    InvalidFolder,
    #[error("File name is empty after sanitization")]
    EmptyFileName,
    #[error("Failed to write file: {0}")]
    Io(#[from] std::io::Error),
}

pub type FsResult<T> = Result<T, FsError>;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedFileInfo {
    pub file_name: String,
    pub full_path: String,
    pub bytes_written: usize,
}

fn sanitize_file_stem(name: &str) -> String {
    let cleaned: String = name
        .trim()
        .chars()
        .map(|c| match c {
            '\\' | '/' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect();

    cleaned.trim_matches('.').trim().to_string()
}

pub fn write_cs_file(
    folder_path: &str,
    file_stem_or_name: &str,
    content: &str,
    overwrite: bool,
) -> FsResult<SavedFileInfo> {
    let folder = Path::new(folder_path.trim());
    if folder.as_os_str().is_empty() {
        return Err(FsError::InvalidFolder);
    }

    let without_ext = file_stem_or_name
        .trim()
        .strip_suffix(".cs")
        .unwrap_or(file_stem_or_name);
    let safe_stem = sanitize_file_stem(without_ext);

    if safe_stem.is_empty() {
        return Err(FsError::EmptyFileName);
    }

    fs::create_dir_all(folder)?;

    let file_name = format!("{safe_stem}.cs");
    let target_path: PathBuf = folder.join(&file_name);

    if !overwrite && target_path.exists() {
        return Err(FsError::Io(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("File already exists: {}", target_path.display()),
        )));
    }

    let bytes = content.as_bytes();
    fs::write(&target_path, bytes)?;

    Ok(SavedFileInfo {
        file_name,
        full_path: target_path.to_string_lossy().into_owned(),
        bytes_written: bytes.len(),
    })
}