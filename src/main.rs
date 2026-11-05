mod entry;
mod shell;
mod control;

use crossterm::event;
use std::{io, process};
use std::path::PathBuf;
use std::fs;

use crate::control::entry_str::entry_str;
use crate::control::selector::run_selector;
use crate::entry::{CurrentPath, EntryType};
use crate::shell::UserCommand;

fn main() -> io::Result<()> {
    let current_dir = PathBuf::from(shell::default_path());
    let mut current_path: CurrentPath = CurrentPath(current_dir.clone());
    let mut hidden_status = false;

    match entry::read_directory(&current_path, false) {
        Ok(entries) => {
            for entry in entries {
                let type_str = entry_str(&entry);
                
                println!(
                    "[{}] {} ({} bytes)",
                    type_str,
                    entry.name(),
                    entry.size
                );
            }
        }
        Err(e) => println!("Error reading directory: {e}")
    }

    loop {
        println!();
        match entry::read_directory(&current_path, hidden_status) {
            Ok(entries) => {
                let (
                    entered_entry,
                    user_command,
                    show_hidden
                ) = run_selector(&entries, &current_path, hidden_status)?;

                if user_command == UserCommand::Exit {
                    shell::clear_screen();
                    process::exit(0x0);
                }

                hidden_status = show_hidden;

                let entry = &entered_entry;
                let type_str = entry_str(entry);

                match entry.entry_type {
                    EntryType::Directory | EntryType::Parent => current_path = CurrentPath(entered_entry.path.clone()),
                    EntryType::File => {
                        let content = fs::read_to_string(entered_entry.path.clone())?;
                        println!("\n{content}");
                        event::read()?;
                    },
                }

                println!(
                    "[{}] {} ({} bytes)",
                    type_str,
                    entry.name(),
                    entry.size
                );

                println!("\nCurrent path: {}", current_path.0.display());
            }
            Err(e) => println!("Error reading directory: {e}")
        }
    }
}
