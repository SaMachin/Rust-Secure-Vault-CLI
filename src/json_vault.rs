use anyhow::Context;
use serde::{Serialize, Deserialize};
use serde_with::{serde_as, base64::Base64};
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    #[serde_as(as = "Base64")]
    pub salt: Vec<u8>,
    #[serde_as(as = "Base64")]
    pub nonce: Vec<u8>,
    #[serde_as(as = "Base64")]
    pub cipher_text: Vec<u8>,
}

pub fn open_vault(path: &PathBuf) -> anyhow::Result<HashMap<String, Entry>> {
    if path.exists() {
        let file_content = fs::read_to_string(&path)
            .context("Failed to read the vault file")?;
        let entries = serde_json::from_str(&file_content)
            .context("Vault file is corrupted or is not valid JSON")?;
        Ok(entries)
    } else {
        Ok(HashMap::new())
    }
}

pub fn write_vault(path: PathBuf, entries: HashMap<String, Entry>) -> anyhow::Result<()> {
    let json_string = serde_json::to_string_pretty(&entries)
        .context("Failed to serialize the vault entries")?;

    if !path.exists() { println!("No vault at the path {path:?} exists, a new vault is going to be created") }

    fs::write(path, json_string)
        .context("Failed to save the vault file to disk")?;
    Ok(())
}

pub fn add_entry(entries: &mut HashMap<String, Entry>, label: String, entry: Entry) {
    entries.insert(label, entry);
}

pub fn delete_entry(entries: &mut HashMap<String, Entry>, label: String) -> bool {
    entries.remove(&label).is_some()
}

pub fn read_entry<'a>(entries: &'a HashMap<String, Entry>, label: &String) -> Option<&'a Entry> {
    entries.get(label)
}

#[cfg(test)]
mod tests {
    use tempfile::{NamedTempFile, env::temp_dir};

    use super::*;

    #[test]
    fn test_create_empty_vault() {
        let temp_dir = temp_dir();
        let path = temp_dir.join("\\temp_test_empty_vault.json");

        let empty_vault = open_vault(&path).unwrap();
        
        assert!(empty_vault.is_empty());
    }
    
    #[test]
    fn test_add_delete_cycle() {
        let label: String = String::from("Very top secret entry !_@#");
        let mut entries = HashMap::new();
        let entry = Entry {
            salt: "salt".into(),
            nonce: "nonce".into(),
            cipher_text: "Very_top_secret_text!_@#".into(),
        };
        
        add_entry(&mut entries, label.clone(), entry);

        assert!(delete_entry(&mut entries, label), "Failed to delete the entry");
        assert!(entries.is_empty(), "'Entries' was not empty after deletion");
    }

    #[test]
    fn test_write_read_cycle() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        let label: String = String::from("Very top secret entry !_@#");
        let mut entries = HashMap::new();
        let entry = Entry {
            salt: "salt".into(),
            nonce: "nonce".into(),
            cipher_text: "Very_top_secret_text!_@#".into(),
        };

        add_entry(&mut entries, label.clone(), entry.clone());
        write_vault(path.clone(), entries.clone()).unwrap();

        let entries_disk = open_vault(&path).unwrap();
        let entry_read = read_entry(&entries_disk, &label).unwrap();
        
        assert_eq!(entry.cipher_text, entry_read.cipher_text);
    }
}