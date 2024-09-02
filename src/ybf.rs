use std::{fs::File, io::Write, path::PathBuf};

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use anyhow::{Context, Result};
use argon2::{password_hash::SaltString, Argon2};
use rand::Rng;
use serde::{Deserialize, Serialize};

pub const YBF_MAGIC: [u8; 8] = [0x89, 0x59, 0x42, 0x46, 0x0d, 0x0a, 0x1a, 0x0a]; // \x89YBF\r\n\x1a\n <- Magic number to identfy a ybf file
pub const YBF_CURRENT_VERSION: u8 = 1;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ybf {
    magic: [u8; 8],
    version: u8,
    length: u32,
    data: Vec<u8>,
}

impl Default for Ybf {
    fn default() -> Self {
        Self::new(YBF_CURRENT_VERSION, vec![])
    }
}

impl Ybf {
    pub fn new(version: u8, data: Vec<u8>) -> Self {
        Self {
            magic: YBF_MAGIC,
            version,
            length: data.len() as u32,
            data,
        }
    }

    pub fn create(password: &str, data: Vec<u8>) -> Self {
        Self::new(YBF_CURRENT_VERSION, Self::encrypt(password, data))
    }

    fn generate_key_from_password(password: &[u8], salt: &[u8]) -> [u8; 32] {
        let argon2 = Argon2::default();
        let mut key = [0u8; 32];
        argon2.hash_password_into(password, salt, &mut key).unwrap();
        key
    }

    fn encrypt(password: &str, data: Vec<u8>) -> Vec<u8> {
        let salt = SaltString::generate(&mut OsRng);
        let key = Self::generate_key_from_password(password.as_bytes(), salt.as_str().as_bytes());
        let rng = OsRng.gen::<[u8; 12]>();
        let nonce = Nonce::from_slice(&rng);
        let key = Key::<Aes256Gcm>::from_slice(&key);
        let cipher = Aes256Gcm::new(key);
        let ciphertext = cipher
            .encrypt(nonce, data.as_ref())
            .expect("encryption failure!");
        let mut encrypted_data = Vec::new();
        encrypted_data.extend_from_slice(salt.as_str().as_bytes());
        encrypted_data.extend_from_slice(nonce);
        encrypted_data.extend_from_slice(&ciphertext);

        encrypted_data
    }

    fn decrypt(password: &str, data: Vec<u8>) -> Vec<u8> {
        let salt = &data[..22];
        let nonce = Nonce::from_slice(&data[22..34]);
        let ciphertext = &data[34..];
        let key = Self::generate_key_from_password(password.as_bytes(), salt);
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
        cipher
            .decrypt(nonce, ciphertext)
            .expect("decryption failure!")
    }

    pub fn decrypt_data(&self, password: &str) -> Result<String> {
        let bytes = Self::decrypt(password, self.data().to_vec());
        String::from_utf8(bytes).context("couldn't decrypt data")
    }

    pub fn write_file(&self, output_file_path: PathBuf) -> Result<()> {
        let mut output_file = File::create(output_file_path)?;
        let enc_data = bincode::serialize(self)?;
        output_file.write_all(&enc_data)?;

        Ok(())
    }

    pub fn from_file(file_path: PathBuf) -> Result<Self> {
        let input_data = std::fs::read(&file_path)?;
        let ybf = bincode::deserialize(&input_data)?;

        Ok(ybf)
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn is_valid(bytes: &[u8]) -> bool {
        bytes.len() >= 16 && bytes[0..8] == YBF_MAGIC
    }
}
