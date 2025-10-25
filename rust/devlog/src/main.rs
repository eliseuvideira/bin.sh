use anyhow::{Context, Result};
use chrono::Local;
use clap::Parser;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, IsTerminal, Write};
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Parser)]
enum Cli {
    Note(NoteCommand),
    Rant(RantCommand),
    List,
    Show(ShowCommand),
    Edit(EditCommand),
}

#[derive(Debug, Parser)]
struct ShowCommand {
    #[arg(index = 1)]
    date: Option<String>,
}

#[derive(Debug, Parser)]
struct EditCommand {
    #[arg(index = 1)]
    date: Option<String>,
}

#[derive(Debug, Parser)]
struct NoteCommand {
    #[arg(index = 1)]
    content: String,
}

#[derive(Debug, Parser)]
struct RantCommand {
    #[arg(index = 1)]
    content: String,
}

fn get_devlog_dir() -> Result<PathBuf> {
    let home = env::var("HOME").context("HOME environment variable not set")?;
    let devlog_dir = PathBuf::from(home).join(".devlog");
    fs::create_dir_all(&devlog_dir).context("Failed to create .devlog directory")?;
    Ok(devlog_dir)
}

fn get_devlog_path() -> Result<PathBuf> {
    let devlog_dir = get_devlog_dir()?;
    let today = Local::now().format("%Y-%m-%d").to_string();
    let file_path = devlog_dir.join(format!("{}.txt", today));

    Ok(file_path)
}

fn append_entry(entry_type: &str, content: &str) -> Result<()> {
    let file_path = get_devlog_path()?;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)
        .context("Failed to open devlog file")?;

    writeln!(file, "{}: {}", entry_type, content).context("Failed to write to devlog file")?;

    println!("{}", file_path.display());

    Ok(())
}

fn format_entry_for_terminal(file_name: &str) -> String {
    if let Some(date_str) = file_name.strip_suffix(".txt") {
        if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            let day_of_week = date.format("%A");
            return format!("{} ({})", date_str, day_of_week);
        }
        return date_str.to_string();
    }
    file_name.to_string()
}

fn list_entries() -> Result<()> {
    let devlog_dir = get_devlog_dir()?;
    let mut entries: Vec<_> = fs::read_dir(&devlog_dir)?
        .filter_map(|e| e.ok())
        .collect();

    entries.sort_by_key(|e| e.file_name());

    let is_terminal = io::stdout().is_terminal();

    for entry in entries {
        let file_path = entry.path();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        if is_terminal {
            println!("{}", format_entry_for_terminal(file_name));
        } else {
            println!("{}", file_name);
        }
    }
    Ok(())
}

fn show_entry(date: Option<String>) -> Result<()> {
    let file_path = if let Some(date) = date {
        let devlog_dir = get_devlog_dir()?;
        devlog_dir.join(format!("{}.txt", date))
    } else {
        get_devlog_path()?
    };

    let entries = fs::read_to_string(&file_path)
        .context(format!("Failed to read devlog file: {}", file_path.display()))?;
    println!("{}", entries);
    Ok(())
}

fn edit_entry(date: Option<String>) -> Result<()> {
    let file_path = if let Some(date) = date {
        let devlog_dir = get_devlog_dir()?;
        devlog_dir.join(format!("{}.txt", date))
    } else {
        get_devlog_path()?
    };

    let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());

    Command::new(editor)
        .arg(&file_path)
        .status()
        .context("Failed to open editor")?;

    Ok(())
}

fn main() -> Result<()> {
    let command = Cli::parse();

    match command {
        Cli::Note(note_command) => {
            append_entry("note", &note_command.content)?;
        }
        Cli::Rant(rant_command) => {
            append_entry("rant", &rant_command.content)?;
        }
        Cli::List => {
            list_entries()?;
        }
        Cli::Show(show_command) => {
            show_entry(show_command.date)?;
        }
        Cli::Edit(edit_command) => {
            edit_entry(edit_command.date)?;
        }
    }

    Ok(())
}
