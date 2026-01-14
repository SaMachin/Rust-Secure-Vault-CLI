use anyhow::Context;
use serde::{Serialize, Deserialize};
use serde_with::{serde_as, base64::Base64};
use std::fs;
use std::path::Path;
use std::collections::HashMap;

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[serde_as(as = "Base64")]
    pub salt: Vec<u8>,
    #[serde_as(as = "Base64")]
    pub nonce: Vec<u8>,
    #[serde_as(as = "Base64")]
    pub cipher_text: Vec<u8>,
}

pub fn open_vault(path: &String) -> anyhow::Result<HashMap<String, Entry>> {
    let path = Path::new(&path);
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

pub fn write_vault(path: &String, entries: HashMap<String, Entry>) -> anyhow::Result<()> {
    let json_string = serde_json::to_string_pretty(&entries)
        .context("Failed to serialize the vault entries")?;
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
    use std::env::temp_dir;

    use super::*;

    #[test]
    fn test_create_empty_vault() {
        let temp_dir = temp_dir();
        let path = temp_dir.join("temp_test_vault.json");
        let path_string: String = String::from(path.to_str().unwrap());

        let empty_vault = open_vault(&path_string).unwrap();
        
        assert!(empty_vault.is_empty());
    }
}