use std::io::{Result, stdout};

use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::terminal;
use crossterm::execute;

use crate::control::entry_str::entry_str;
use crate::entry::{self, CurrentPath, Entry};
use crate::shell::{self, UserCommand};

pub fn run_selector(entries: &[Entry], path: &CurrentPath, mut show_hidden: bool) -> Result<(Entry, UserCommand, bool)> {
    let mut local_entries = entries.to_vec();
    let mut selected_index: usize = 0;
    let mut user_command = UserCommand::Enter;
    terminal::enable_raw_mode()?;

    loop {
        shell::clear_screen();

        let (_, rows) = terminal::size()?;
        let visible = (rows as usize).saturating_sub(1).max(1);
        let start = (selected_index + 1).saturating_sub(visible);
        for (index, entry) in local_entries.iter().enumerate().skip(start).take(visible) {

            let type_str = entry_str(entry);

            let bg_line_color: Color;
            let text_color: Color;

            if index == selected_index {
                bg_line_color = Color::White;
                text_color = Color::Black;
            } else {
                bg_line_color = Color::Reset;
                text_color = Color::Reset;
            }

            let entry_name = entry.name();

            execute!(
                stdout(),
                SetBackgroundColor(bg_line_color),
                SetForegroundColor(text_color),
                Print(format!("[{}] {} ({} bytes)\r\n", type_str, entry_name, entry.size)),
                ResetColor
            )?;
        }

        let event = event::read()?;

        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Enter => {
                    break;
                },
                KeyCode::Up | KeyCode::Char('w') => {
                    if selected_index != 0 {
                        selected_index -= 1;
                    }
                },
                KeyCode::Down | KeyCode::Char('s') => {
                    if selected_index != local_entries.len() - 1 {
                        selected_index += 1;
                    }
                },
                KeyCode::Char('h') if key_event.modifiers == KeyModifiers::CONTROL => {
                    shell::clear_screen();
                    show_hidden = !show_hidden;

                    match entry::read_directory(&path, show_hidden) {
                        Ok(entry) => local_entries = entry,
                        Err(_e) => continue
                    }
                },
                KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => {
                    user_command = UserCommand::Exit;
                    break;
                }
                _ => {}
            }
        }
    }

    terminal::disable_raw_mode()?;
    Ok((
        local_entries[selected_index].clone(),
        user_command,
        show_hidden
    ))
}