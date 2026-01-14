mod cli;
mod json_vault;
mod crypto;

use anyhow::{Context, anyhow};
use clap::Parser;
use std::path::PathBuf;
use rpassword::prompt_password;

use cli::{Cli, Commands, DEFAULT_PATH};
use json_vault::{Entry, open_vault, write_vault, add_entry, delete_entry, read_entry};
use crypto::{cipher, decipher};

fn handle_path(path: String) -> PathBuf {
    if path == DEFAULT_PATH {
        PathBuf::from(DEFAULT_PATH)
    } else {
        PathBuf::from(&path)
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let path = handle_path(cli.path);
    let mut entries = open_vault(&path)
        .context("Failed to open vault")?;

    match &cli.command {
        Commands::Add {label} => {
            if entries.contains_key(label) {
                println!("An entry with the label '{label}' already exists. If you continue, you will overwrite this entry.\nContinue? (y/n)");

                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .context("Failed to read user's choice for overwriting")?;

                if input.trim().to_lowercase() !="y" {
                    println!("Action cancelled");
                    return Ok(());
                } else {
                    println!("'{label}' is goind to be overwritten");
                }
            }

            println!("Enter the text to be encrypted :");
            let mut text = String::new();
            std::io::stdin()
                .read_line(&mut text)
                .context("Failed to read the text")?;

            let password = prompt_password("Enter password: ")
                .context("Failed to read the password")?;

            let entry: Entry = cipher(text, password)
                .context("Failed to create a new entry")?;
            add_entry(&mut entries, String::from(label), entry);
            
            write_vault(path, entries)
                .context("Failed to save the vault to disk")?;
            println!("Entry '{label}' was successfully saved")
        }
        Commands::Delete { label } => {
            if !&path.exists() {
                return Err(anyhow!("Failed to delete entry '{label}', because no vault file was found at path: {path:?}"));
            }

            if delete_entry(&mut entries, String::from(label)) {
                write_vault(path, entries)
                    .context("Failed to save the vault to disk")?;
                println!("Entry '{label}' was correctly deleted");
            } else {
                println!("Entry '{label}' does not exist, nothing to delete");
            }
        }
        Commands::Read { label } => {
            if !path.exists() {
                return Err(anyhow!("Failed to read entry '{label}', because no vault file was found at path: {path:?}"));
            }

            let Some(entry) = read_entry(&entries, &label) else {
                return Err(anyhow!("Entry '{label}' does not exists, nothing to read"));
            };
            let password = prompt_password("Enter password: ")
                .context("Error while reading the password")?;
            let text_decypher = decipher(&entry, password)
                .context("Failed to decrypt the entry")?;
            println!("{text_decypher}");
        }
    }
    Ok(())
}