// file_writer.rs

use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub struct FileWriter {
    base_directory: String,
}

impl FileWriter {
    pub fn new(base_directory: &str) -> FileWriter {
        FileWriter {
            base_directory: base_directory.to_string(),
        }
    }

    pub fn write_todo_list_file(&self, file_name: &str, content: &str) {
        let file_path = self.get_file_path(file_name);
        println!("Writing to file: {}", file_path);
        if let Ok(mut file) = fs::File::create(file_path) {
            if let Err(e) = file.write_all(content.as_bytes()) {
                eprintln!("Failed to write to file: {}", e);
            }
        } else {
            eprintln!("Failed to create file: {}", file_name);
        }
    }

    pub fn read_todo_list_file(&self, file_name: &str) -> Option<String> {
        let file_path = self.get_file_path(file_name);
        if let Ok(content) = fs::read_to_string(file_path) {
            Some(content)
        } else {
            None
        }
    }

    fn get_file_path(&self, file_name: &str) -> String {
        let full_path = format!("{}/{}", self.base_directory, file_name);
        full_path
    }

    pub fn list_todo_list_files(&self) -> Vec<String> {
        let dir = Path::new(&self.base_directory);
        let mut files = Vec::new();
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() {
                        files.push(file_name.to_string());
                    }
                }
            }
        }
        files
    }
}
