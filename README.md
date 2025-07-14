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

## Quick Start (For Beginners)

### Option 1: Download Pre-built Binary (Easiest)
1. **Download** the latest release from [GitHub Releases](https://github.com/0xReLogic/Rust_wallet_generator/releases)
2. **Extract** the ZIP file to a folder
3. **Open Command Prompt/PowerShell** in that folder
4. **Run the GUI** (recommended for beginners):
   ```cmd
   rust_wallet_genenerator.exe --gui
   ```

### Option 2: Build from Source
Follow the installation instructions below.

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
    The binary will be located at `target/release/rust_wallet_genenerator`.

## Binary Releases and Verification

For convenience, pre-compiled binaries for Windows, macOS, and Linux are available in the [GitHub Releases](https://github.com/0xReLogic/Rust_wallet_generator/releases) section.

To ensure the integrity of the downloaded files, we provide a `sha256sum.txt` file with each release. You can verify the checksum of your downloaded binary:

```bash
# Example for Linux
sha256sum -c sha256sum.txt --ignore-missing

# Example for Windows PowerShell
Get-FileHash -Algorithm SHA256 release/rust_wallet_genenerator.exe | ForEach-Object { if ($_.Hash.ToLower() -eq (Get-Content sha256sum.txt | ForEach-Object { $_.Split()[0] })) { Write-Host "Checksum verified successfully!" } else { Write-Host "Checksum verification failed!" } }
```

## How to Run the Executable (.exe) File

### Windows Users

#### Method 1: Using Command Prompt/PowerShell (Recommended)
1. **Download the binary** from [GitHub Releases](https://github.com/0xReLogic/Rust_wallet_generator/releases)
2. **Extract the ZIP file** to a folder of your choice
3. **Open Command Prompt or PowerShell**:
   - Press `Win + R`, type `cmd` or `powershell`, then press Enter
   - Or right-click on the folder containing the .exe file and select "Open in Terminal"
4. **Navigate to the folder** containing the .exe file:
   ```cmd
   cd "C:\path\to\your\extracted\folder"
   ```
5. **Run the executable**:
   ```cmd
   # For GUI mode (recommended for beginners)
   rust_wallet_genenerator.exe --gui
   
   # For CLI mode
   rust_wallet_genenerator.exe generate -t eth
   ```

#### Method 2: Double-Click (Not Recommended)
- **Warning**: Double-clicking may not work properly due to Windows security settings
- If you get a security warning, click "More info" → "Run anyway"
- The program will open in CLI mode by default

#### Method 3: Right-Click Method
1. Right-click on `rust_wallet_genenerator.exe`
2. Select "Open with" → "Command Prompt" or "PowerShell"
3. The terminal will open in the correct directory automatically

### Troubleshooting Windows Issues

#### "Windows protected your PC" Error
1. Right-click on the .exe file
2. Select "Properties"
3. Check "Unblock" at the bottom of the properties window
4. Click "Apply" → "OK"
5. Try running again

#### "This app can't run on your PC" Error
1. Make sure you downloaded the correct version (x64 for 64-bit Windows)
2. Try running as Administrator:
   - Right-click on the .exe file
   - Select "Run as administrator"

#### "Command not found" Error
1. Make sure you're in the correct directory containing the .exe file
2. Use `dir` or `ls` to list files and verify the .exe is there
3. Type the exact filename: `rust_wallet_genenerator.exe`

### Linux Users
```bash
# Make the file executable
chmod +x rust_wallet_genenerator

# Run the program
./rust_wallet_genenerator --gui
```

### macOS Users
```bash
# Remove quarantine attribute
xattr -d com.apple.quarantine rust_wallet_genenerator

# Run the program
./rust_wallet_genenerator --gui
```

## Usage

### From Source (Development)
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

### From Binary (.exe file)
```bash
# GUI Mode (Recommended for beginners)
rust_wallet_genenerator.exe --gui

# Generate a new Ethereum wallet
rust_wallet_genenerator.exe generate -t eth

# Generate a Solana wallet with seed phrase
rust_wallet_genenerator.exe generate -t sol --seed-phrase

# Generate and save encrypted wallet
rust_wallet_genenerator.exe generate -t eth --save --encrypt --password "YourPassword"

# Batch generate 5 Bitcoin wallets
rust_wallet_genenerator.exe batch -t btc --count 5

# Import wallet from seed phrase
rust_wallet_genenerator.exe import -t eth --phrase "your twelve word seed phrase goes here"

# Show help
rust_wallet_genenerator.exe --help
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

## Frequently Asked Questions (FAQ)

### Q: How do I run the .exe file on Windows?
A: See the detailed guide in the [How to Run the Executable](#how-to-run-the-executable-exe-file) section above.

### Q: I get "Windows protected your PC" error, what should I do?
A: Right-click the .exe file → Properties → Check "Unblock" → Apply → OK, then try running again.

### Q: The program doesn't start when I double-click it
A: Use Command Prompt/PowerShell instead. Right-click the folder → "Open in Terminal" → run `rust_wallet_genenerator.exe --gui`

### Q: How do I verify the binary is safe?
A: Use the checksum verification method shown in the [Binary Releases and Verification](#binary-releases-and-verification) section.

### Q: Can I use this offline?
A: Yes! This tool is 100% offline and doesn't require any internet connection.

### Q: Which wallet types are supported?
A: Ethereum (ETH), Bitcoin (BTC), Solana (SOL), Polygon (MATIC), and Avalanche (AVAX).

### Q: How do I backup my wallets?
A: Always save your seed phrases securely. The tool can export wallets to encrypted files for additional security.

## Support My Work
If you find this tool useful, please consider supporting its development. Thank you!

[![Buy Me a Coffee](https://img.shields.io/badge/Buy%20Me%20a%20Coffee-FFDD00?style=for-the-badge&logo=buy-me-a-coffee&logoColor=black)](https://buymeacoffee.com/0xrelogic)

- **Solana**: `5Bkgy7Xd6zj5K6HkCERoNNseCGx7WCX3LxrxmL8KgCk4`
- **Buy Me a Coffee**: [https://buymeacoffee.com/0xrelogic](https://buymeacoffee.com/0xrelogic)

## Disclaimer
This software is provided "as is". The author is not responsible for any loss of funds. Always generate wallets on a trusted, secure, and offline machine. Verify all addresses and manage your keys securely.

---

**Made with ❤️ by [0xReLogic](https://github.com/0xReLogic)** 