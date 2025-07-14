use anyhow::Result;
use bip39::{Mnemonic, Language};
use rand::Rng;
use ethers::signers::{coins_bip39::English, MnemonicBuilder};
use slip10::{derive_key_from_path, Curve};
use ed25519_dalek::SecretKey;


pub struct Bip39Generator;

impl Bip39Generator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_phrase(&self, word_count: u8) -> Result<String> {
        let entropy_bits = match word_count {
            12 => 128,
            24 => 256,
            _ => return Err(anyhow::anyhow!("Word count must be 12 or 24")),
        };

        let entropy_bytes = entropy_bits / 8;
        let mut entropy = vec![0u8; entropy_bytes];
        rand::thread_rng().fill(&mut entropy[..]);
        
        let mnemonic = Mnemonic::from_entropy(&entropy)?;
        Ok(mnemonic.to_string())
    }

    pub fn phrase_to_private_key(&self, phrase: &str) -> Result<Vec<u8>> {
        // Build wallet from mnemonic with default derivation path (m/44'/60'/0'/0/0) for Ethereum
        let wallet = MnemonicBuilder::<English>::default()
            .phrase(phrase)
            .build()
            .map_err(|e| anyhow::anyhow!("ethers error: {e}"))?;
        let signing_key = wallet.signer();
        Ok(signing_key.to_bytes().to_vec())
    }

    pub fn phrase_to_private_key_solana(&self, phrase: &str) -> Result<Vec<u8>> {
        // Parse mnemonic and create seed
        let mnemonic = Mnemonic::parse_normalized(phrase)
            .map_err(|e| anyhow::anyhow!("Invalid mnemonic: {}", e))?;
        let seed = mnemonic.to_seed("");
        
        // Derive key using Solana's BIP44 path: m/44'/501'/0'/0'
        let path = "m/44'/501'/0'/0'";
        let derived = derive_key_from_path(&seed, Curve::Ed25519, path)
            .map_err(|e| anyhow::anyhow!("Derivation failed: {}", e))?;
        
        // Convert to Ed25519 secret key
        let secret = SecretKey::from_bytes(&derived.key)
            .map_err(|e| anyhow::anyhow!("Invalid secret key: {}", e))?;
        
        Ok(secret.to_bytes().to_vec())
    }

    #[allow(dead_code)]
    pub fn validate_phrase(&self, phrase: &str) -> Result<bool> {
        match Mnemonic::parse_in_normalized(Language::English, phrase) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    #[allow(dead_code)]
    pub fn get_word_count(&self, phrase: &str) -> Result<u8> {
        let words: Vec<&str> = phrase.split_whitespace().collect();
        let count = words.len() as u8;
        
        match count {
            12 | 24 => Ok(count),
            _ => Err(anyhow::anyhow!("Invalid word count: {}", count)),
        }
    }
} 