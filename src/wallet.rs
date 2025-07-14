use anyhow::Result;
use k256::ecdsa::SigningKey;
use secp256k1::{SecretKey as Secp256k1SecretKey, PublicKey as Secp256k1PublicKey};
use bitcoin::{Address, Network, PublicKey};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::str::FromStr;
use crate::bip39::Bip39Generator;
use crate::qr::QrGenerator;
use ed25519_dalek::{PublicKey as Ed25519PublicKey, SecretKey as Ed25519SecretKey};


// WalletType enum for supported blockchains
#[derive(Debug, Clone, PartialEq)]
pub enum WalletType {
    Eth,
    Btc,
    Sol,
    Polygon,
    Avax,
    Bitcoin,
    Ethereum,
    Litecoin,
    Dogecoin,
}

impl std::fmt::Display for WalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WalletType::Eth => write!(f, "eth"),
            WalletType::Btc => write!(f, "btc"),
            WalletType::Sol => write!(f, "sol"),
            WalletType::Polygon => write!(f, "polygon"),
            WalletType::Avax => write!(f, "avax"),
            WalletType::Bitcoin => write!(f, "bitcoin"),
            WalletType::Ethereum => write!(f, "ethereum"),
            WalletType::Litecoin => write!(f, "litecoin"),
            WalletType::Dogecoin => write!(f, "dogecoin"),
        }
    }
}

// Wallet struct for storing wallet data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub wallet_type: String,
    pub address: String,
    pub private_key: String,
    pub seed_phrase: Option<String>,
    pub created_at: DateTime<Utc>,
    pub network: String,
    pub support_my_work: String,
}

impl Wallet {
    pub fn new(
        wallet_type: WalletType,
        address: String,
        private_key: String,
        seed_phrase: Option<String>,
    ) -> Self {
        let network = match wallet_type {
            WalletType::Eth => "Ethereum Mainnet".to_string(),
            WalletType::Btc => "Bitcoin Mainnet".to_string(),
            WalletType::Sol => "Solana Mainnet".to_string(),
            WalletType::Polygon => "Polygon Mainnet".to_string(),
            WalletType::Avax => "Avalanche Mainnet".to_string(),
            WalletType::Bitcoin => "Bitcoin Mainnet".to_string(),
            WalletType::Ethereum => "Ethereum Mainnet".to_string(),
            WalletType::Litecoin => "Litecoin Mainnet".to_string(),
            WalletType::Dogecoin => "Dogecoin Mainnet".to_string(),
        };

        Self {
            wallet_type: wallet_type.to_string(),
            address,
            private_key,
            seed_phrase,
            created_at: Utc::now(),
            network,
            support_my_work: "Solana: 5Bkgy7Xd6zj5K6HkCERoNNseCGx7WCX3LxrxmL8KgCk4".to_string(),
        }
    }

    pub fn display_info(&self) {
        println!("\n{}", "=".repeat(50));
        println!("WALLET INFO");
        println!("{}", "=".repeat(50));
        
        println!("Type: {}", self.wallet_type);
        println!("Network: {}", self.network);
        println!("Address: {}", self.address);
        println!("Private Key: {}", self.private_key);
        
        if let Some(phrase) = &self.seed_phrase {
            println!("Seed Phrase: {}", phrase);
        }
        
        println!("Created: {}", self.created_at.format("%Y-%m-%d %H:%M:%S UTC"));
        println!("Support my work: {}", self.support_my_work);
        println!("{}", "=".repeat(50));
    }

    pub async fn display_qr_code(&self) -> Result<()> {
        let qr_gen = QrGenerator::new();
        qr_gen.display_address_qr(&self.address).await?;
        Ok(())
    }

    pub async fn save_to_file(&self, format: &str) -> Result<()> {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("{}_{}_{}.{}", self.wallet_type, timestamp, "wallet", format);
        
        match format {
            "json" => {
                let json_content = serde_json::to_string_pretty(self)?;
                std::fs::write(&filename, json_content)?;
                println!("Saved to: {}", filename);
                // Export Solana keypair JSON if wallet_type is sol
                if self.wallet_type == "sol" {
                    use ed25519_dalek::{SecretKey, PublicKey};
                    let priv_bytes = hex::decode(&self.private_key)?;
                    let secret = SecretKey::from_bytes(&priv_bytes)?;
                    let public = PublicKey::from(&secret);
                    let mut keypair_bytes = Vec::new();
                    keypair_bytes.extend_from_slice(&secret.to_bytes());
                    keypair_bytes.extend_from_slice(public.as_bytes());
                    let json = serde_json::to_string(&keypair_bytes)?;
                    let keypair_filename = format!("{}_solana_keypair.json", self.address);
                    std::fs::write(&keypair_filename, json)?;
                    println!("Saved Solana keypair to: {}", keypair_filename);
                }
            }
            "txt" => {
                let mut content = String::new();
                content.push_str(&format!("Wallet Type: {}\n", self.wallet_type));
                content.push_str(&format!("Network: {}\n", self.network));
                content.push_str(&format!("Address: {}\n", self.address));
                content.push_str(&format!("Private Key: {}\n", self.private_key));
                if let Some(phrase) = &self.seed_phrase {
                    content.push_str(&format!("Seed Phrase: {}\n", phrase));
                }
                content.push_str(&format!("Created: {}\n", self.created_at.format("%Y-%m-%d %H:%M:%S UTC")));
                content.push_str(&format!("Support my work: {}\n", self.support_my_work));
                
                std::fs::write(&filename, content)?;
                println!("Saved to: {}", filename);
            }
            _ => {
                return Err(anyhow::anyhow!("Unsupported format: {}", format));
            }
        }
        
        Ok(())
    }
}

// WalletGenerator struct for wallet operations
pub struct WalletGenerator {
    bip39_gen: Bip39Generator,
}

impl WalletGenerator {
    pub fn new() -> Self {
        Self {
            bip39_gen: Bip39Generator::new(),
        }
    }

    pub async fn generate_wallet(
        &self,
        wallet_type: WalletType,
        generate_seed_phrase: bool,
        word_count: u8,
    ) -> Result<Wallet> {
        let (private_key, seed_phrase) = if generate_seed_phrase {
            let phrase = self.bip39_gen.generate_phrase(word_count)?;
            let private_key = match wallet_type {
                WalletType::Sol => self.bip39_gen.phrase_to_private_key_solana(&phrase)?,
                _ => self.bip39_gen.phrase_to_private_key(&phrase)?,
            };
            (private_key, Some(phrase))
        } else {
            let private_key = self.generate_random_private_key()?;
            (private_key, None)
        };

        let address = self.generate_address(&private_key, &wallet_type).await?;
        let private_key_hex = hex::encode(private_key);
        let private_key_str = private_key_hex; // Tanpa 0x prefix

        let wallet = Wallet::new(
            wallet_type.clone(),
            address,
            private_key_str, // Tanpa 0x prefix
            seed_phrase,
        );

        Ok(wallet)
    }

    pub async fn import_from_phrase(
        &self,
        phrase: &str,
        wallet_type: WalletType,
    ) -> Result<Wallet> {
        let private_key = match wallet_type {
            WalletType::Sol => self.bip39_gen.phrase_to_private_key_solana(phrase)?,
            _ => self.bip39_gen.phrase_to_private_key(phrase)?,
        };
        let address = self.generate_address(&private_key, &wallet_type).await?;
        let private_key_hex = hex::encode(&private_key); // Tanpa 0x prefix

        Ok(Wallet::new(
            wallet_type,
            address,
            private_key_hex, // Tanpa 0x prefix
            Some(phrase.to_string()),
        ))
    }

    #[allow(dead_code)]
    pub async fn validate_address(&self, address: &str, wallet_type: WalletType) -> Result<bool> {
        match wallet_type {
            WalletType::Eth => {
                // Ethereum address validation
                if !address.starts_with("0x") || address.len() != 42 {
                    return Ok(false);
                }
                // Basic hex validation
                Ok(address[2..].chars().all(|c| c.is_ascii_hexdigit()))
            }
            WalletType::Btc => {
                // Bitcoin address validation
                Ok(Address::from_str(address).is_ok())
            }
            WalletType::Sol => {
                // Solana address validation (base58 decode check)
                Ok(bs58::decode(address).into_vec().is_ok())
            }
            WalletType::Polygon => {
                // Polygon address validation (base58 decode check)
                Ok(bs58::decode(address).into_vec().is_ok())
            }
            WalletType::Avax => {
                // Avalanche address validation (base58 decode check)
                Ok(bs58::decode(address).into_vec().is_ok())
            }
            WalletType::Bitcoin => {
                // Bitcoin address validation
                Ok(Address::from_str(address).is_ok())
            }
            WalletType::Ethereum => {
                // Ethereum address validation
                if !address.starts_with("0x") || address.len() != 42 {
                    return Ok(false);
                }
                // Basic hex validation
                Ok(address[2..].chars().all(|c| c.is_ascii_hexdigit()))
            }
            WalletType::Litecoin => {
                // Litecoin address validation (base58 decode check)
                Ok(bs58::decode(address).into_vec().is_ok())
            }
            WalletType::Dogecoin => {
                // Dogecoin address validation (base58 decode check)
                Ok(bs58::decode(address).into_vec().is_ok())
            }
        }
    }

    fn generate_random_private_key(&self) -> Result<Vec<u8>> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut private_key = [0u8; 32];
        rng.fill(&mut private_key);
        Ok(private_key.to_vec())
    }

    async fn generate_address(&self, private_key: &[u8], wallet_type: &WalletType) -> Result<String> {
        match wallet_type {
            WalletType::Eth => self.generate_eth_address(private_key),
            WalletType::Btc => self.generate_btc_address(private_key),
            WalletType::Sol => self.generate_sol_address(private_key),
            WalletType::Polygon => self.generate_polygon_address(private_key),
            WalletType::Avax => self.generate_avax_address(private_key),
            WalletType::Bitcoin => self.generate_btc_address(private_key),
            WalletType::Ethereum => self.generate_eth_address(private_key),
            WalletType::Litecoin => self.generate_btc_address(private_key),
            WalletType::Dogecoin => self.generate_btc_address(private_key),
        }
    }

    // Generate Ethereum address from private key
    fn generate_eth_address(&self, private_key: &[u8]) -> Result<String> {
        // Gunakan secp256k1 yang sama dengan MetaMask
        let secp = secp256k1::Secp256k1::new();
        let secret_key = secp256k1::SecretKey::from_slice(private_key)?;
        let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
        
        // Convert to uncompressed format (65 bytes)
        let public_key_bytes = public_key.serialize_uncompressed();
        
        // Ethereum address is the last 20 bytes of the Keccak-256 hash of the public key
        use sha3::{Digest, Keccak256};
        let mut hasher = Keccak256::new();
        hasher.update(&public_key_bytes[1..]); // Skip first byte (0x04)
        let result = hasher.finalize();
        
        let address = &result[12..]; // Last 20 bytes
        Ok(format!("0x{}", hex::encode(address)))
    }

    // Generate Bitcoin address from private key
    fn generate_btc_address(&self, private_key: &[u8]) -> Result<String> {
        let secp = secp256k1::Secp256k1::new();
        let secret_key = Secp256k1SecretKey::from_slice(private_key)?;
        let _public_key = Secp256k1PublicKey::from_secret_key(&secp, &secret_key);
        
        // Convert to bitcoin's secp256k1 format
        let bitcoin_secp = bitcoin::secp256k1::Secp256k1::new();
        let bitcoin_secret = bitcoin::secp256k1::SecretKey::from_slice(private_key)?;
        let bitcoin_public = bitcoin::secp256k1::PublicKey::from_secret_key(&bitcoin_secp, &bitcoin_secret);
        
        let bitcoin_public_key = PublicKey::new(bitcoin_public);
        let address = Address::p2pkh(&bitcoin_public_key, Network::Bitcoin);
        
        Ok(address.to_string())
    }

    // Generate Solana address from private key
    fn generate_sol_address(&self, private_key: &[u8]) -> Result<String> {
        let secret = Ed25519SecretKey::from_bytes(private_key)?;
        let public = Ed25519PublicKey::from(&secret);
        // Solana address is the base58-encoded public key
        Ok(bs58::encode(public.as_bytes()).into_string())
    }

    // Generate Polygon address from private key
    fn generate_polygon_address(&self, private_key: &[u8]) -> Result<String> {
        // Polygon uses same format as Ethereum (secp256k1 + Keccak256)
        self.generate_eth_address(private_key)
    }

    // Generate Avalanche address from private key
    fn generate_avax_address(&self, private_key: &[u8]) -> Result<String> {
        // Avalanche uses same format as Ethereum (secp256k1 + Keccak256)
        self.generate_eth_address(private_key)
    }

    pub async fn generate_wallet_by_type(&self, chain: &str, include_seed_phrase: bool) -> Result<Wallet> {
        let wallet_type = match chain {
            "eth" => WalletType::Eth,
            "btc" => WalletType::Btc,
            "sol" => WalletType::Sol,
            "polygon" => WalletType::Polygon,
            "avax" => WalletType::Avax,
            "bitcoin" => WalletType::Bitcoin,
            "ethereum" => WalletType::Ethereum,
            "litecoin" => WalletType::Litecoin,
            "dogecoin" => WalletType::Dogecoin,
            _ => return Err(anyhow::anyhow!("Unsupported chain: {}", chain)),
        };
        
        self.generate_wallet(wallet_type, include_seed_phrase, 12).await
    }
} 

 

pub async fn generate_single_wallet(
    wallet_type: &str,
    seed_phrase: bool,
    save: bool,
    qr: bool,
    format: &str,
    _encrypt: bool,
    _password: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let generator = WalletGenerator::new();
    let wallet = generator.generate_wallet_by_type(wallet_type, seed_phrase).await?;
    
    // Display wallet info
    wallet.display_info();
    
    // Generate QR code if requested
    if qr {
        wallet.display_qr_code().await?;
    }
    
    // Save to file if requested
    if save {
        wallet.save_to_file(format).await?;
    }
    
    Ok(())
}

pub async fn generate_batch_wallets(
    wallet_type: &str,
    count: u32,
    seed_phrase: bool,
    save: bool,
    format: &str,
    _encrypt: bool,
    _password: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let generator = WalletGenerator::new();
    let mut wallets = Vec::new();
    
    for i in 1..=count {
        println!("Generating wallet {}/{}...", i, count);
        let wallet = generator.generate_wallet_by_type(wallet_type, seed_phrase).await?;
        wallets.push(wallet);
    }
    
    // Display summary
    println!("\nBatch generation completed!");
    println!("Generated {} {} wallets", count, wallet_type);
    
    // Save all wallets to file if requested
    if save {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("batch_{}_{}_{}.{}", wallet_type, count, timestamp, format);
        
        match format {
            "json" => {
                let json_content = serde_json::to_string_pretty(&wallets)?;
                std::fs::write(&filename, json_content)?;
                println!("Saved batch to: {}", filename);
            }
            "txt" => {
                let mut content = String::new();
                content.push_str(&format!("Batch Generation Report\n"));
                content.push_str(&format!("Generated: {} wallets\n", count));
                content.push_str(&format!("Type: {}\n", wallet_type));
                content.push_str(&format!("Timestamp: {}\n\n", timestamp));
                
                for (i, wallet) in wallets.iter().enumerate() {
                    content.push_str(&format!("=== Wallet {} ===\n", i + 1));
                    content.push_str(&format!("Type: {}\n", wallet.wallet_type));
                    content.push_str(&format!("Network: {}\n", wallet.network));
                    content.push_str(&format!("Address: {}\n", wallet.address));
                    content.push_str(&format!("Private Key: {}\n", wallet.private_key));
                    if let Some(phrase) = &wallet.seed_phrase {
                        content.push_str(&format!("Seed Phrase: {}\n", phrase));
                    }
                    content.push_str(&format!("Created: {}\n", wallet.created_at.format("%Y-%m-%d %H:%M:%S UTC")));
                    content.push_str(&format!("Support my work: {}\n\n", wallet.support_my_work));
                }
                
                std::fs::write(&filename, content)?;
                println!("Saved batch to: {}", filename);
            }
            _ => {
                return Err(anyhow::anyhow!("Unsupported format: {}", format).into());
            }
        }
    }
    
    Ok(())
}

pub async fn import_wallet(
    wallet_type: &str,
    phrase: &str,
    save: bool,
    format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let generator = WalletGenerator::new();
    let wallet_type_enum = match wallet_type {
        "eth" => WalletType::Eth,
        "btc" => WalletType::Btc,
        "sol" => WalletType::Sol,
        "polygon" => WalletType::Polygon,
        "avax" => WalletType::Avax,
        "bitcoin" => WalletType::Bitcoin,
        "ethereum" => WalletType::Ethereum,
        "litecoin" => WalletType::Litecoin,
        "dogecoin" => WalletType::Dogecoin,
        _ => return Err(anyhow::anyhow!("Unsupported wallet type: {}", wallet_type).into()),
    };
    
    let wallet = generator.import_from_phrase(phrase, wallet_type_enum).await?;
    
    // Display wallet info
    wallet.display_info();
    
    // Save to file if requested
    if save {
        wallet.save_to_file(format).await?;
    }
    
    Ok(())
} 

pub async fn generate_wallet_async(
    wallet_type: WalletType,
    _encrypt: bool,
    _password: Option<&str>,
    include_seed_phrase: bool,
) -> Result<Wallet, String> {
    let generator = WalletGenerator::new();
    
    // Convert wallet type to string for compatibility
    let wallet_type_str = match wallet_type {
        WalletType::Bitcoin => "btc",
        WalletType::Ethereum => "eth", 
        WalletType::Litecoin => "btc", // Litecoin uses same format as Bitcoin
        WalletType::Dogecoin => "btc", // Dogecoin uses same format as Bitcoin
        WalletType::Eth => "eth",
        WalletType::Btc => "btc",
        WalletType::Sol => "sol",
        WalletType::Polygon => "polygon",
        WalletType::Avax => "avax",
    };
    
    match generator.generate_wallet_by_type(wallet_type_str, include_seed_phrase).await {
        Ok(wallet) => Ok(wallet),
        Err(e) => Err(e.to_string()),
    }
} 