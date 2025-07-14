# Rust Wallet Generator

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-blue?style=for-the-badge)](https://github.com/0xReLogic/Rust_wallet_generator)
[![Offline](https://img.shields.io/badge/Status-100%25%20Offline-green?style=for-the-badge)](https://github.com/0xReLogic/Rust_wallet_generator)
[![Security](https://img.shields.io/badge/Security-Community%20Audited%20%26%20Verified-orange?style=for-the-badge)](https://github.com/0xReLogic/Rust_wallet_generator)
[![Crypto](https://img.shields.io/badge/Crypto-ETH%20%7C%20SOL%20%7C%20BTC%20%7C%20MATIC%20%7C%20AVAX-purple?style=for-the-badge)](https://github.com/0xReLogic/Rust_wallet_generator)

A pioneering, security-focused offline wallet generator built entirely in Rust. It provides a fast, transparent, and secure way to generate cryptocurrency wallets without ever needing an internet connection.

## Why Rust Wallet Generator?

In a world of complex, online-dependent wallet tools, Rust Wallet Generator stands apart by prioritizing simplicity, security, and transparency. Here’s why it's a superior choice for managing your crypto assets securely:

*   **100% Offline Operation**: Generate, encrypt, and manage wallets on a completely air-gapped machine. The tool makes zero network requests, ensuring your private keys never touch the internet.
*   **Security First, Built in Rust**: Leverages Rust's memory safety and performance to eliminate common vulnerabilities found in other languages. Your security is guaranteed by the power of modern, systems-level code.
*   **Fully Transparent & Auditable**: Every line of code is open-source and available for review. We use well-vetted, standard cryptographic libraries. You don't have to trust us—you can verify the code yourself.
*   **Blazing Fast**: Generate single or batch wallets in an instant. No waiting, no delays.
*   **No Dependencies, No Bloat**: Compiles to a single, static binary that you can run anywhere. No need to install complex frameworks or runtimes.

## Core Features

- **Native GUI & CLI**: Modern Iced framework GUI with full CLI support
- **Multi-Chain Support**: Generate wallets for ETH, BTC, SOL, Polygon, and Avalanche
- **BIP39 Seed Phrases**: Industry-standard 12 or 24-word seed phrase support for wallet recovery
- **Batch Generation**: Create hundreds of wallets with a single command
- **Wallet Import**: Recover wallets from existing seed phrases
- **Copy to Clipboard**: One-click copy address, private key, and seed phrase
- **QR Code Display**: Generate QR codes for addresses, private keys, and seed phrases
- **Export Formats**: Save to JSON or TXT with timestamp
- **Solana Keypair Export**: JSON format compatible with Phantom/Sollet
- **100% Offline**: No telemetry, no network requests, air-gapped ready

## Fully Open-Source and Auditable

This project is built on the principle of full transparency.
- **Source Code**: All `.rs` source files are available for public audit.
- **Dependencies (`crates`)**: We rely on popular, well-maintained crates from the Rust ecosystem. You can review every dependency in the `Cargo.toml` and `Cargo.lock` files.

We encourage security researchers and the community to review the codebase.

## Installation

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable version)

### From Source
1.  **Clone the repository:**
    ```bash
    git clone https://github.com/0xReLogic/Rust_wallet_generator.git
    cd Rust_wallet_generator
    ```

2.  **Build the release binary:**
    ```bash
    cargo build --release
    ```
    The binary will be located at `target/release/rust_wallet_generator`.

## Binary Releases and Verification

For convenience, pre-compiled binaries for Windows, macOS, and Linux are available in the [GitHub Releases](https://github.com/0xReLogic/Rust_wallet_generator/releases) section.

To ensure the integrity of the downloaded files, we provide a `sha256sum.txt` file with each release. You can verify the checksum of your downloaded binary:

```bash
# Example for Linux
sha256sum -c sha256sum.txt --ignore-missing
```

## Usage

```bash
# Generate a new Ethereum wallet and display it
cargo run -- generate -t eth

# Generate a Solana wallet with a seed phrase and save it to an encrypted file
cargo run -- generate -t sol --seed-phrase --save --encrypt --password "YourSecretPassword"

# Batch generate 10 Bitcoin wallets
cargo run -- batch -t btc --count 10

# Import a wallet from a seed phrase
cargo run -- import -t eth --phrase "your twelve word seed phrase goes here"
```

## Compatibility & Verification

All generated wallets are **100% compatible** with popular wallet applications:

### Ethereum, Polygon, Avalanche
- **MetaMask**: Import private key or seed phrase
- **Trust Wallet**: Import seed phrase
- **Address verification**: Generated addresses match exactly

### Solana
- **Phantom**: Import seed phrase (recommended)
- **Sollet**: Import seed phrase
- **Keypair export**: JSON format compatible with Phantom/Sollet

### Bitcoin
- **Trust Wallet**: Import seed phrase (generates Native SegWit bc1...)
- **Electrum**: Import seed phrase
- **Address format**: Legacy (1...) by default, Trust Wallet converts to Native SegWit

### GUI Mode
```bash
cargo run --release -- --gui
```
Run the native GUI for easy wallet generation and management.

## Support My Work
If you find this tool useful, please consider supporting its development. Thank you!

[![Buy Me a Coffee](https://img.shields.io/badge/Buy%20Me%20a%20Coffee-FFDD00?style=for-the-badge&logo=buy-me-a-coffee&logoColor=black)](https://buymeacoffee.com/0xrelogic)

- **Solana**: `5Bkgy7Xd6zj5K6HkCERoNNseCGx7WCX3LxrxmL8KgCk4`
- **Buy Me a Coffee**: [https://buymeacoffee.com/0xrelogic](https://buymeacoffee.com/0xrelogic)

## Disclaimer
This software is provided "as is". The author is not responsible for any loss of funds. Always generate wallets on a trusted, secure, and offline machine. Verify all addresses and manage your keys securely.

---

**Made with ❤️ by [0xReLogic](https://github.com/0xReLogic)** 
