use std::path::PathBuf;

use crate::file::FileType;

#[derive(Clone)]
pub struct FileInfo {
    name: String,
    file_type: FileType,
    path: PathBuf,
}

impl FileInfo {
    pub fn from(file_path: &str) -> Self {
        let original_path = PathBuf::from(file_path);

        // 若文件没有扩展名，则添加 .txt 后缀
        let path = if original_path.extension().is_none() {
            original_path.with_extension("txt")
        } else {
            original_path
        };

        let name = path
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or(file_path)
            .to_string();

        let file_type = match path.extension() {
            Some(ext) if ext.eq_ignore_ascii_case("rs") => FileType::Rust,
            _ => FileType::Text, // 此时无扩展名的情况已被处理（添加了txt），这里会匹配txt
        };

        FileInfo {
            name,
            file_type,
            path: path,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_file_type(&self) -> &FileType {
        &self.file_type
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn get_path_str(&self) -> String {
        self.path
            .to_str()
            .map_or("路径未知".to_string(), |s| s.to_string())
    }
}

impl Default for FileInfo {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            file_type: FileType::Text,
            path: PathBuf::from(""),
        }
    }
}
