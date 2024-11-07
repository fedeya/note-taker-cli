mod cli;
use anyhow::{Context, Result};
use clap::Parser;
use rayon::prelude::*;

use glob::glob;
use serde::{Deserialize, Serialize};
use std::fs;

use cli::{Cli, Commands};

#[derive(Debug, Deserialize, Serialize)]
struct Note {
    content: String,
    category: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    setup()?;

    match &cli.command {
        Commands::Add { content, category } => {
            add_note(content.clone(), category.clone())
                .context("Failed to add note")
                .unwrap();

            println!("Note added!");
        }
        Commands::List { category } => {
            let notes = get_notes().context("Failed to get notes")?;

            for note in notes {
                if let Some(c) = &category {
                    if let Some(cat) = &note.category {
                        if cat == c {
                            println!("{}", note.content);
                        }
                    }
                } else {
                    println!("{}", note.content);
                }
            }
        }
    }

    Ok(())
}

fn setup() -> Result<()> {
    let path = format!("{}/.local/share/note-cli-rs", get_home_dir()?);

    fs::create_dir_all(path)?;

    Ok(())
}

fn add_note(content: String, category: Option<String>) -> Result<()> {
    let note = Note { content, category };

    let note_json = serde_json::to_string(&note)?;

    let timestamp = chrono::Utc::now().timestamp();

    let path = format!(
        "{}/.local/share/note-cli-rs/{}.json",
        get_home_dir()?,
        timestamp
    );

    fs::write(path, note_json)?;

    Ok(())
}

fn get_notes() -> Result<Vec<Note>> {
    let path = format!("{}/.local/share/note-cli-rs/*.json", get_home_dir()?);

    let entries = glob(&path)
        .context("Failed to read glob pattern")?
        .collect::<Vec<_>>();

    let notes: Vec<Note> = entries
        .par_iter()
        .map(|entry| -> Result<Note> {
            let path = entry.as_ref().unwrap().clone();

            let content = fs::read_to_string(path).context("Failed to read file")?;

            let note: Note = serde_json::from_str(&content).context("Failed to parse JSON")?;

            Ok(note)
        })
        .filter_map(Result::ok)
        .collect();

    Ok(notes)
}

fn get_home_dir() -> Result<String> {
    let dir = dirs::home_dir()
        .context("Failed to get home dir")?
        .to_str()
        .context("Failed to convert to str")?
        .to_string();

    Ok(dir)
}
