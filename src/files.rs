//! The module `files` manages scanning the filesystem for input markdown files.

use std::{
    collections::HashMap,
    fs::{self, DirEntry},
    path::Path,
};

use crate::error::Result;

/// The default path for Markdown files comprising the source of the wiki is ./pages in the project
/// root folder.
pub const FILES_DIR_PATH: &str = "./pages/";

/// [`Files`] associates filenames to raw content.
pub type Files = HashMap<String, String>;

/// Reads all files from the specified directory into a [`HashMap`]. Skips non-file entries
/// (directories, symlinks).
pub fn read_files_from_dir(dir_path: &str) -> Result<Files> {
    fs::read_dir(dir_path)?
        .filter_map(std::result::Result::ok)
        .filter(|entry| entry.path().is_file())
        .map(|e| read_file_entry(&e))
        .collect()
}

/// Transforms the specified [`DirEntry`] into a (filename, content) tuple.
fn read_file_entry(entry: &DirEntry) -> Result<(String, String)> {
    let path = entry.path();
    let filename = entry.file_name().into_string().map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8 filename")
    })?;
    let content = fs::read_to_string(&path)?;
    Ok((filename, content))
}

pub fn strip_extension_from_filename(filename: &str) -> Option<String> {
    Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .map(std::string::ToString::to_string)
}
