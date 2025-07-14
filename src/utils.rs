#[allow(dead_code)]
pub fn display_banner() {
    println!("\n{}", "=".repeat(60));
    println!("RUST WALLET GENERATOR");
    println!("Offline Cryptocurrency Wallet Generator");
    println!("{}", "=".repeat(60));
    println!("Supports: ETH | BTC | SOL");
    println!("Features: QR Code | BIP39 | File Export");
    println!("{}", "=".repeat(60));
}

#[allow(dead_code)]
pub fn display_security_warning() {
    println!("\nSECURITY WARNING");
    println!("{}", "=".repeat(40));
    println!("- Keep your private keys secure");
    println!("- Never share your seed phrase");
    println!("- Store backups offline");
    println!("- Use on trusted devices only");
    println!("{}", "=".repeat(40));
}

#[allow(dead_code)]
pub fn display_success_message() {
    println!("\nWallet generated successfully!");
    println!("Tip: Save your wallet info securely");
}

#[allow(dead_code)]
pub fn display_error_message(message: &str) {
    println!("Error: {}", message);
}

#[allow(dead_code)]
pub fn format_address(address: &str) -> String {
    if address.len() > 20 {
        format!("{}...{}", &address[..10], &address[address.len()-10..])
    } else {
        address.to_string()
    }
}

#[allow(dead_code)]
pub fn format_private_key(private_key: &str) -> String {
    if private_key.len() > 20 {
        format!("{}...{}", &private_key[..10], &private_key[private_key.len()-10..])
    } else {
        private_key.to_string()
    }
}

#[allow(dead_code)]
pub fn get_timestamp() -> String {
    use chrono::Utc;
    Utc::now().format("%Y%m%d_%H%M%S").to_string()
}

#[allow(dead_code)]
pub fn create_wallet_directory() -> std::io::Result<std::path::PathBuf> {
    let home_dir = dirs::home_dir().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "Home directory not found")
    })?;
    let wallet_dir = home_dir.join(".wallets");
    std::fs::create_dir_all(&wallet_dir)?;
    Ok(wallet_dir)
} 