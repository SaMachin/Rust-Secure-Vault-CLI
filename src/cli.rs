use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Add or overwrite an entry
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