use anyhow::{Context, Result};
use chrono::Local;
use clap::Parser;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Parser)]
enum Cli {
    Note(NoteCommand),
    Rant(RantCommand),
    List,
    Show,
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

fn list_entries() -> Result<()> {
    let devlog_dir = get_devlog_dir()?;
    let entries = fs::read_dir(&devlog_dir)?;
    for entry in entries {
        let entry = entry?;
        let file_path = entry.path();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        println!("{}", file_name);
    }
    Ok(())
}

fn show_entry() -> Result<()> {
    let file_path = get_devlog_path()?;
    let entries = fs::read_to_string(&file_path)?;
    println!("{}", entries);
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
        Cli::Show => {
            show_entry()?;
        }
    }

    Ok(())
}
