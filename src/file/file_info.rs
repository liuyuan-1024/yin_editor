use std::path::PathBuf;

use crate::file::FileType;

#[derive(Clone)]
pub struct FileInfo {
    name: String,
    file_type: FileType,
    path: Option<PathBuf>,
}

impl FileInfo {
    pub fn from(file_path: &str) -> Self {
        let path = PathBuf::from(file_path);

        let name = path
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or(file_path)
            .to_string();

        let file_type = match path.extension() {
            Some(ext) if ext.eq_ignore_ascii_case("rs") => FileType::Rust,
            Some(ext) if ext.eq_ignore_ascii_case("txt") => FileType::Text,
            _ => FileType::Unsupport,
        };

        FileInfo {
            name,
            file_type,
            path: Some(path),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_file_type(&self) -> &FileType {
        &self.file_type
    }

    pub fn get_path(&self) -> &Option<PathBuf> {
        &self.path
    }

    pub fn get_path_str(&self) -> String {
        // 1. as_ref()：将 &Option<PathBuf> 转为 Option<&PathBuf>（避免移动所有权）
        // 2. and_then：调用 to_str()，将 Option<&PathBuf> 转为 Option<&str>
        // 3. map：将 &str 转为 String
        self.path
            .as_ref()
            .and_then(|pb| pb.to_str())
            .map_or("路径未知".to_string(), |s| s.to_string())
    }
}

impl Default for FileInfo {
    fn default() -> Self {
        Self {
            name: "untitled".to_string(),
            file_type: FileType::Unsupport,
            path: None,
        }
    }
}
