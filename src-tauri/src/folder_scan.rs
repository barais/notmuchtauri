use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FolderNode {
    pub name: String,
    pub path: String,
    pub is_maildir: bool,
    pub children: Vec<FolderNode>,
}

pub struct FolderScanner;

impl FolderScanner {
    /// Recursively scans a directory to find Maildirs.
    /// A directory is a Maildir if it contains 'cur', 'new', and 'tmp' subdirectories.
    pub fn scan(root: &Path) -> Result<Vec<FolderNode>, Box<dyn Error>> {
        let mut nodes = Vec::new();
        println!("Scanning directory: {:?}", root);
        if let Ok(entries) = fs::read_dir(root) {
            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    let name = entry.file_name().to_str()
                        .unwrap_or("Unknown")
                        .to_string();

                    if Self::is_maildir(&path) {
                        nodes.push(FolderNode {
                            name,
                            path: path.to_string_lossy().to_string(),
                            is_maildir: true,
                            children: vec![],
                        });
                    } else {
                        // Not a maildir, but might contain maildirs, so we recurse
                        
                        let pathname = path.file_name();
                        if pathname.is_some() && !pathname.unwrap().to_string_lossy().to_string().starts_with(".") {
                        let children = Self::scan(&path).unwrap_or_default();
                        nodes.push(FolderNode {
                            name,
                            path: path.to_string_lossy().to_string(),
                            is_maildir: false,
                            children,
                        });

                        }
                    }
                }
            }
        }

        Ok(nodes)
    }

    fn is_maildir(path: &Path) -> bool {
        let has_cur = path.join("cur").is_dir();
        let has_new = path.join("new").is_dir();
        let has_tmp = path.join("tmp").is_dir();
        has_cur && has_new && has_tmp
    }
}
