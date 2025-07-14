use anyhow::Result;
use aes_gcm::{Aes256Gcm, KeyInit};
use aes_gcm::aead::{Aead, Key, Nonce};
use pbkdf2::pbkdf2;
use rand::Rng;
use sha2::Sha256;
use std::path::PathBuf;
use std::println;

#[allow(dead_code)]
pub struct FileEncryption;

#[allow(dead_code)]
impl FileEncryption {
    pub fn new() -> Self {
        Self
    }

    // Encrypt data with password using AES-256-GCM
    pub fn encrypt_with_password(&self, data: &[u8], password: &str) -> Result<Vec<u8>> {
        // Generate salt and derive key
        let mut salt = [0u8; 32];
        rand::thread_rng().fill(&mut salt);
        
        let mut key = [0u8; 32];
        pbkdf2::<pbkdf2::hmac::Hmac<Sha256>>(password.as_bytes(), &salt, 10000, &mut key)
            .map_err(|e| anyhow::anyhow!("PBKDF2 key derivation failed: {}", e))?;
        
        // Generate nonce
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill(&mut nonce_bytes);
        let nonce = Nonce::<Aes256Gcm>::from_slice(&nonce_bytes);
        
        // Create cipher
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
        
        // Encrypt data
        let ciphertext = cipher.encrypt(nonce, data)
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;
        
        // Combine salt + nonce + ciphertext
        let mut result = Vec::new();
        result.extend_from_slice(&salt);
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);
        
        Ok(result)
    }

    // Decrypt data with password
    pub fn decrypt_with_password(&self, encrypted_data: &[u8], password: &str) -> Result<Vec<u8>> {
        if encrypted_data.len() < 44 {
            return Err(anyhow::anyhow!("Invalid encrypted data format"));
        }
        
        // Extract salt, nonce, and ciphertext
        let salt = &encrypted_data[..32];
        let nonce_bytes = &encrypted_data[32..44];
        let ciphertext = &encrypted_data[44..];
        
        // Derive key
        let mut key = [0u8; 32];
        pbkdf2::<pbkdf2::hmac::Hmac<Sha256>>(password.as_bytes(), salt, 10000, &mut key)
            .map_err(|e| anyhow::anyhow!("PBKDF2 key derivation failed: {}", e))?;
        
        // Create cipher
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
        let nonce = Nonce::<Aes256Gcm>::from_slice(nonce_bytes);
        
        // Decrypt data
        let plaintext = cipher.decrypt(nonce, ciphertext)
            .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;
        
        Ok(plaintext)
    }

    // Encrypt wallet file with password
    pub fn encrypt_wallet_file(&self, wallet_data: &str, password: &str) -> Result<Vec<u8>> {
        self.encrypt_with_password(wallet_data.as_bytes(), password)
    }

    // Decrypt wallet file with password
    pub fn decrypt_wallet_file(&self, encrypted_data: &[u8], password: &str) -> Result<String> {
        let decrypted = self.decrypt_with_password(encrypted_data, password)?;
        String::from_utf8(decrypted).map_err(|e| anyhow::anyhow!("Invalid UTF-8: {}", e))
    }
}

#[allow(dead_code)]
pub async fn encrypt_file(input: &PathBuf, output: &PathBuf, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read input file
    let file_content = std::fs::read_to_string(input)?;
    
    // Encrypt content
    let encryption = FileEncryption::new();
    let encrypted_data = encryption.encrypt_wallet_file(&file_content, password)?;
    
    // Write encrypted file
    std::fs::write(output, encrypted_data)?;
    println!("File encrypted successfully: {}", output.display());
    
    Ok(())
}

#[allow(dead_code)]
pub async fn decrypt_file(input: &PathBuf, output: &PathBuf, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read encrypted file
    let encrypted_data = std::fs::read(input)?;
    
    // Decrypt content
    let encryption = FileEncryption::new();
    let decrypted_content = encryption.decrypt_wallet_file(&encrypted_data, password)?;
    
    // Write decrypted file
    std::fs::write(output, decrypted_content)?;
    println!("File decrypted successfully: {}", output.display());
    
    Ok(())
} 