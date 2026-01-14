use clap::{Parser, Subcommand};

pub const DEFAULT_PATH: &str = "vault.json";

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, global=true, default_value=DEFAULT_PATH)]
    pub path: String,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Add or overwrite an entry. If the file doesn't already exist, a file will be created.
    Add {
        label: String,
    },
    /// Delete an entry
    Delete {
        label: String,
    },
    /// Decipher an entry and print it in the terminal
    Read {
        label: String,
    }
}