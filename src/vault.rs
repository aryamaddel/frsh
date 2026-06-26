use std::fs;
use std::path::{Path, PathBuf};
use gpui::Context;

pub struct Vault {
    root_path: PathBuf,
    files: Vec<PathBuf>,
}

impl Vault {
    pub fn new(root_path: PathBuf, _cx: &mut Context<Self>) -> Self {
        if !root_path.exists() {
            let _ = fs::create_dir_all(&root_path);
        }
        let mut vault = Self { root_path, files: Vec::new() };
        vault.rescan();
        vault
    }

    #[allow(dead_code)]
    pub fn root_path(&self) -> &Path {
        &self.root_path
    }

    pub fn files(&self) -> &[PathBuf] {
        &self.files
    }

    pub fn rescan(&mut self) {
        let mut new_files = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.root_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "txt" || ext == "md" {
                            new_files.push(path);
                        }
                    }
                }
            }
        }
        new_files.sort();
        self.files = new_files;
    }

    pub fn read_file(&self, path: &Path) -> Result<String, std::io::Error> {
        fs::read_to_string(path)
    }

    pub fn save_file(&mut self, path: &Path, content: &str) -> Result<(), std::io::Error> {
        fs::write(path, content)?;
        self.rescan();
        Ok(())
    }

    pub fn create_file(&mut self, name: &str) -> Result<PathBuf, std::io::Error> {
        let mut path = self.root_path.join(name);
        if path.extension().is_none() {
            path.set_extension("md");
        }
        fs::write(&path, "")?;
        self.rescan();
        Ok(path)
    }

    pub fn delete_file(&mut self, path: &Path) -> Result<(), std::io::Error> {
        fs::remove_file(path)?;
        self.rescan();
        Ok(())
    }

    #[allow(dead_code)]
    pub fn rename_file(&mut self, old: &Path, new_name: &str) -> Result<PathBuf, std::io::Error> {
        let new_path = self.root_path.join(new_name);
        fs::rename(old, &new_path)?;
        self.rescan();
        Ok(new_path)
    }
}
