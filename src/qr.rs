use anyhow::Result;
use qr2term::print_qr;
use qrcode::QrCode;
use image::Luma;
use std::path::PathBuf;

pub struct QrGenerator;

impl QrGenerator {
    pub fn new() -> Self {
        Self
    }

    // Display address as QR code in terminal
    pub async fn display_address_qr(&self, address: &str) -> Result<()> {
        println!("\nQR CODE (Address)");
        println!("{}", "=".repeat(30));
        
        match print_qr(address) {
            Ok(_) => {
                println!("\nQR Code generated successfully.");
                println!("Address: {}", address);
            }
            Err(e) => {
                println!("Failed to generate QR code: {}", e);
            }
        }
        
        Ok(())
    }

    // Generate QR code as PNG image
    #[allow(dead_code)]
    pub async fn generate_qr_png(&self, data: &str, filename: &str) -> Result<()> {
        // Generate QR code
        let code = QrCode::new(data)?;
        
        // Convert to image using the correct API for qrcode 0.12
        let image = code.render::<Luma<u8>>().build();
        
        // Save as PNG
        image.save(filename)?;
        
        println!("QR code saved as PNG: {}", filename);
        Ok(())
    }

    // Generate QR code for wallet address as PNG
    #[allow(dead_code)]
    pub async fn generate_address_qr_png(&self, address: &str, filename: &str) -> Result<()> {
        self.generate_qr_png(address, filename).await
    }

    // Generate QR code for private key as PNG
    #[allow(dead_code)]
    pub async fn generate_private_key_qr_png(&self, private_key: &str, filename: &str) -> Result<()> {
        self.generate_qr_png(private_key, filename).await
    }

    // Generate QR code for seed phrase as PNG
    #[allow(dead_code)]
    pub async fn generate_seed_phrase_qr_png(&self, seed_phrase: &str, filename: &str) -> Result<()> {
        self.generate_qr_png(seed_phrase, filename).await
    }

    // Display private key as QR code in terminal
    #[allow(dead_code)]
    pub async fn display_private_key_qr(&self, private_key: &str) -> Result<()> {
        println!("\nQR CODE (Private Key)");
        println!("{}", "=".repeat(35));
        println!("WARNING: Keep this private!");
        
        match print_qr(private_key) {
            Ok(_) => {
                println!("\nPrivate Key QR generated.");
                println!("Store this securely!");
            }
            Err(e) => {
                println!("Failed to generate QR code: {}", e);
            }
        }
        
        Ok(())
    }

    // Display seed phrase as QR code in terminal
    #[allow(dead_code)]
    pub async fn display_seed_phrase_qr(&self, seed_phrase: &str) -> Result<()> {
        println!("\nQR CODE (Seed Phrase)");
        println!("{}", "=".repeat(35));
        println!("WARNING: Keep this private!");
        
        match print_qr(seed_phrase) {
            Ok(_) => {
                println!("\nSeed Phrase QR generated.");
                println!("Store this securely!");
            }
            Err(e) => {
                println!("Failed to generate QR code: {}", e);
            }
        }
        
        Ok(())
    }
}

#[allow(dead_code)]
pub async fn generate_qr_png(data: &str, output: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let qr_gen = QrGenerator::new();
    qr_gen.generate_qr_png(data, &output.to_string_lossy()).await?;
    Ok(())
} 