use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod category;
mod error;
mod organiser;

use error::NeatlyError;

#[derive(Parser)]
#[command(name = "neatly", about = "A clean file organiser")]
struct Cli {
    #[command(subcommand)]
    command: Command
}

#[derive(Subcommand)]
enum Command {
    /// Organise files in a directory
    Run {
        path: PathBuf
    },
    /// Preview what would happen without moving files
    Preview {
        path: PathBuf
    },
    /// Undo the last organisation
    Undo {
        path: PathBuf
    }
}

fn main() -> Result<(), NeatlyError> {
    let cli = Cli::parse();

    match cli.command {
        Command::Run { path} => {
            let entries = organiser::scan(&path)?;
            organiser::organise(&path, &entries)?;
        },
        Command::Preview { path} => {
            let entries = organiser::scan(&path)?;
            organiser::preview(&entries);
        },
        Command::Undo { path} => {
            organiser::undo(&path)?;
        }   
    }

    Ok(())
}
