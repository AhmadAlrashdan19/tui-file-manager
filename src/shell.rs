use crate::entry::{CurrentPath, Entry, EntryType};
use std::io::{self, Write};

/// (\x1B) => Escape Letter (ASCII 27) — He tells The Terminal: "What after is a private command, not a regular script".
/// ([2J) => Clear the screen.
/// ([1;1H) => The cursor returns the first line and the first column.
/// (io::stdout().flush()) => It is necessary because print! (Unlike println!)
/// If the buffer is emptied automatically, you need to force it to print immediately.
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

pub fn default_path() -> String {
    match std::env::var("HOME") {
        Ok(home) => home,
        Err(_) => String::from("/"),
    }
}

pub fn insert_parent_entry(entries: &mut Vec<Entry>, current_path: &CurrentPath) {
    if let Some(parent_path) = current_path.0.parent() {
        let parent_entry = Entry {
            path: parent_path.to_path_buf(),
            entry_type: EntryType::Parent,
            size: 0,
        };
        entries.push(parent_entry);
    }
}

#[derive(PartialEq, Eq)]
pub enum UserCommand {
    Enter,
    Exit,
}