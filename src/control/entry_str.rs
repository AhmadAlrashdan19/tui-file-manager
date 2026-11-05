use crate::entry::{Entry, EntryType};

pub fn entry_str(entry: &Entry) -> &'static str {
    match entry.entry_type {
        EntryType::File => "FIL",
        EntryType::Parent | EntryType::Directory => "DIR",
    }
}