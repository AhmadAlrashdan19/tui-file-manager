use std::path::PathBuf;
use std::{fs, io};
use crate::shell;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EntryType {
    Parent,
    Directory,
    File,
}

#[derive(Clone)]
pub struct Entry {
    pub path: PathBuf,
    pub entry_type: EntryType,
    pub size: u64,
}

#[derive(Clone)]
pub struct CurrentPath(pub PathBuf);

impl Entry {
    pub fn name(&self) -> &str {
        if self.entry_type == EntryType::Parent {
            ".."
        } else {
           self.path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("") 
        }
    }
}

pub fn read_directory(path: &CurrentPath, show_hidden: bool) -> Result<Vec<Entry>, io::Error> {
    let mut entries = Vec::new();
    shell::insert_parent_entry(&mut entries, path);

    for item in fs::read_dir(path.0.clone())? {
        let item = item?;
        let metadata = item.metadata()?;
        let entry_path = item.path();

        // Do not show the hidden files/directoies
        let name = entry_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        if !show_hidden && name.starts_with(".") {
            continue;
        }

        let entry_type = if metadata.is_dir() {
            EntryType::Directory
        } else {
            EntryType::File
        };

        entries.push(Entry {
            path: item.path(),
            entry_type: entry_type,
            size: metadata.len(),
        });
    }

    entries.sort_by(|a: &Entry, b: &Entry| {
        a.entry_type.cmp(&b.entry_type)
        .then(a.name().cmp(b.name()))
    });

    Ok(entries)
}