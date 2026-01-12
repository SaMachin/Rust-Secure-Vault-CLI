use anyhow::Context;
use serde::{Serialize, Deserialize};
use serde_with::{serde_as, base64::Base64};
use std::fs;
use std::path::Path;
use std::collections::HashMap;

const PATH: &str = "vault.json";

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

pub fn open_vault() -> anyhow::Result<HashMap<String, Entry>> {
    let path = Path::new(PATH);
    if path.exists() {
        let file_content = fs::read_to_string(&PATH)
            .context("Failed to read the vault file")?;
        let entries = serde_json::from_str(&file_content)
            .context("Vault file is corrupted or is not valid JSON")?;
        Ok(entries)
    } else {
        Ok(HashMap::new())
    }
}

pub fn write_vault(entries: HashMap<String, Entry>) -> anyhow::Result<()> {
    let json_string = serde_json::to_string_pretty(&entries)
        .context("Failed to serialize the vault entries")?;
    fs::write(&PATH, json_string)
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