use anyhow::{Context, anyhow};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce, Key,
};
use argon2::Argon2;
use rand::RngCore;
use generic_array::{GenericArray, typenum::U12};

use crate::json_vault::Entry;

pub fn cipher(text: String, password: String) -> anyhow::Result<Entry>{
    let mut salt_bytes = [0u8; 16];
    rand::rng().fill_bytes(&mut salt_bytes);

    let argon2 = Argon2::default();
    let mut key_bytes = [0u8; 32];
    argon2.hash_password_into(password.as_bytes(), &salt_bytes, &mut key_bytes)
        .map_err(|e| anyhow!("Argon2 error : {e}"))
        .context("Failed to generate a key")?;

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes: [u8; 12] = [0u8; 12];
    rand::rng().fill_bytes(&mut nonce_bytes);
    let nonce: &GenericArray<u8, U12> = Nonce::from_slice(&nonce_bytes);

    let cipher_text = cipher.encrypt(nonce, text.as_bytes())
        .map_err(|e| anyhow!("Aes error : {e}"))
        .context("Encryption error")?;

    Ok(Entry {
        salt: salt_bytes.to_vec(),
        nonce: nonce_bytes.to_vec(),
        cipher_text: cipher_text,
    })
}

pub fn decipher(entry: &Entry, password: String) -> anyhow::Result<String> {    
    let mut key_bytes = [0u8; 32];
    
    let argon2 = Argon2::default();
    argon2.hash_password_into(password.as_bytes(), &entry.salt, &mut key_bytes)
        .map_err(|e| anyhow!("Argon2 error : {e}"))
        .context("Failed to generate a key")?;

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let nonce = Nonce::from_slice(&entry.nonce);
    let decrypted_bytes = cipher.decrypt(nonce, entry.cipher_text.as_slice())
        .map_err(|e| anyhow!("Aes error : {e}"))
        .context("Failed decrypting : wrong password or corrupted data")?;

    let text_decipher = String::from_utf8(decrypted_bytes)
        .context("The text is not valid UTF-8")?;
    Ok(text_decipher)
}
