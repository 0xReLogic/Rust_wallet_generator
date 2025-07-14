mod bip39;
mod encryption;
mod gui;
mod qr;
mod utils;
mod wallet;

use clap::{Parser, ValueEnum};
use tokio;
use crate::wallet::{generate_single_wallet, generate_batch_wallets, import_wallet};

#[derive(Parser)]
#[command(name = "rust_wallet_gen")]
#[command(about = "A secure offline cryptocurrency wallet generator written in Rust")]
#[command(version = "0.1.0")]
#[command(propagate_version = true)]
struct Cli {
    /// Wallet type to generate
    #[arg(short, long, value_enum, default_value_t = WalletType::Eth)]
    wallet_type: WalletType,

    /// Generate seed phrase
    #[arg(long)]
    seed_phrase: bool,

    /// Save wallet to file
    #[arg(short, long)]
    save: bool,

    /// Generate QR code
    #[arg(short, long)]
    qr: bool,

    /// Output format (json, txt)
    #[arg(short, long, default_value = "json")]
    format: String,

    /// Encrypt wallet file
    #[arg(short, long)]
    encrypt: bool,

    /// Password for encryption
    #[arg(short, long)]
    password: Option<String>,

    /// Batch generation count
    #[arg(short, long)]
    batch: Option<u32>,

    /// Import from seed phrase
    #[arg(short, long)]
    import: Option<String>,

    /// Run GUI mode
    #[arg(short, long)]
    gui: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum WalletType {
    Eth,
    Btc,
    Sol,
    Polygon,
    Avax,
}

impl std::fmt::Display for WalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WalletType::Eth => write!(f, "eth"),
            WalletType::Btc => write!(f, "btc"),
            WalletType::Sol => write!(f, "sol"),
            WalletType::Polygon => write!(f, "polygon"),
            WalletType::Avax => write!(f, "avax"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.gui {
        // Run GUI mode
        gui::run_gui()?;
        return Ok(());
    }

    // CLI mode
    if let Some(phrase) = cli.import {
        // Import wallet from seed phrase
        import_wallet(
            &cli.wallet_type.to_string(),
            &phrase,
            cli.save,
            &cli.format,
        )
        .await?;
    } else if let Some(count) = cli.batch {
        // Batch generation
        generate_batch_wallets(
            &cli.wallet_type.to_string(),
            count,
            cli.seed_phrase,
            cli.save,
            &cli.format,
            cli.encrypt,
            cli.password.as_deref(),
        )
        .await?;
    } else {
        // Single wallet generation
        generate_single_wallet(
            &cli.wallet_type.to_string(),
            cli.seed_phrase,
            cli.save,
            cli.qr,
            &cli.format,
            cli.encrypt,
            cli.password.as_deref(),
        )
        .await?;
    }

    Ok(())
} 