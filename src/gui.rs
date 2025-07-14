use iced::widget::{button, checkbox, column, container, pick_list, row, text, text_input, horizontal_space, vertical_space};
use iced::{Element, Length, Application, Settings, Command, Color};
use iced::theme;
use iced::Background;
use crate::wallet::{Wallet, WalletType, generate_wallet_async};
use arboard::Clipboard;
use qrcode::QrCode;


#[derive(Debug, Clone)]
pub enum Message {
    GenerateWallet,
    WalletGenerated(Result<Wallet, String>),
    WalletTypeChanged(WalletType),
    EncryptChanged(bool),
    PasswordChanged(String),
    ClearWallet,
    ShowAddressQR,
    ShowPrivateKeyQR,
    ShowSeedPhraseQR,
    CopyAddress,
    CopyPrivateKey,
    CopySeedPhrase,
    CloseQRWindow,
    // Core features
    GenerateSeedPhrase,
    SaveWallet,
    ImportWallet,
    BatchGenerate,
    ImportSeedPhraseChanged(String),
    BatchCountChanged(String),
    SaveFormatChanged(String),
    BatchStatusUpdated(String),
    CopySupportWork,
}



// Custom style structs with higher contrast colors
struct ControlsStyle;
struct WalletDisplayStyle;
struct BatchStatusStyle;



impl iced::widget::container::StyleSheet for ControlsStyle {
    type Style = iced::Theme;
    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.9, 0.9, 0.9))),
            ..Default::default()
        }
    }
}

impl iced::widget::container::StyleSheet for WalletDisplayStyle {
    type Style = iced::Theme;
    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(Background::Color(Color::from_rgb(1.0, 1.0, 1.0))),
            ..Default::default()
        }
    }
}

impl iced::widget::container::StyleSheet for BatchStatusStyle {
    type Style = iced::Theme;
    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.9, 1.0, 0.9))),
            border: iced::Border {
                color: Color::from_rgb(0.2, 0.6, 0.2),
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct WalletGenerator {
    wallet: Option<Wallet>,
    wallet_type: WalletType,
    encrypt: bool,
    password: String,
    generating: bool,
    qr_window_open: bool,
    qr_data: Option<String>,
    qr_title: Option<String>,
    // Core features
    include_seed_phrase: bool,
    import_seed_phrase: String,
    batch_count: String,
    save_format: String,
    saving: bool,
    batch_status_message: Option<String>,
}

impl Application for WalletGenerator {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            WalletGenerator {
                wallet: None,
                wallet_type: WalletType::Eth,
                encrypt: false,
                password: String::new(),
                generating: false,
                qr_window_open: false,
                qr_data: None,
                qr_title: None,
                // Core features
                include_seed_phrase: false,
                import_seed_phrase: String::new(),
                batch_count: "1".to_string(),
                save_format: "json".to_string(),
                saving: false,
                batch_status_message: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        if self.qr_window_open {
            self.qr_title.clone().unwrap_or_else(|| "QR Code".to_string())
        } else {
            String::from("Rust Wallet Generator")
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        println!("Processing message: {:?}", message); // Debug line
        match message {
            Message::GenerateWallet => {
                println!("Starting wallet generation..."); // Debug line
                self.generating = true;
                let wallet_type = self.wallet_type.clone();
                let encrypt = self.encrypt;
                let password = if self.encrypt && !self.password.is_empty() {
                    self.password.clone()
                } else {
                    String::new()
                };
                let include_seed_phrase = self.include_seed_phrase;
                Command::perform(
                    async move {
                        let password_ref = if !password.is_empty() {
                            Some(password.as_str())
                        } else {
                            None
                        };
                        generate_wallet_async(wallet_type, encrypt, password_ref, include_seed_phrase).await
                    },
                    Message::WalletGenerated,
                )
            }
            Message::WalletGenerated(result) => {
                println!("Wallet generated: {:?}", result.is_ok()); // Debug line
                self.generating = false;
                self.wallet = result.ok();
                Command::none()
            }
            Message::WalletTypeChanged(wallet_type) => {
                self.wallet_type = wallet_type;
                Command::none()
            }
            Message::EncryptChanged(encrypt) => {
                self.encrypt = encrypt;
                Command::none()
            }
            Message::PasswordChanged(password) => {
                self.password = password;
                Command::none()
            }
            Message::ClearWallet => {
                self.wallet = None;
                // Don't reset batch_status_message to keep the message visible
                Command::none()
            }
            Message::ShowAddressQR => {
                if let Some(wallet) = &self.wallet {
                    println!("Showing Address QR Code");
                    self.qr_window_open = true;
                    self.qr_data = Some(wallet.address.clone());
                    self.qr_title = Some("Address QR Code".to_string());
                }
                Command::none()
            }
            Message::ShowPrivateKeyQR => {
                if let Some(wallet) = &self.wallet {
                    println!("Showing Private Key QR Code");
                    self.qr_window_open = true;
                    self.qr_data = Some(wallet.private_key.clone());
                    self.qr_title = Some("Private Key QR Code".to_string());
                }
                Command::none()
            }
            Message::ShowSeedPhraseQR => {
                if let Some(wallet) = &self.wallet {
                    if let Some(phrase) = &wallet.seed_phrase {
                        println!("Showing Seed Phrase QR Code");
                        self.qr_window_open = true;
                        self.qr_data = Some(phrase.clone());
                        self.qr_title = Some("Seed Phrase QR Code".to_string());
                    }
                }
                Command::none()
            }
            Message::CopyAddress => {
                if let Some(wallet) = &self.wallet {
                    println!("Copying address to clipboard");
                    if let Ok(mut clipboard) = Clipboard::new() {
                        if let Err(e) = clipboard.set_text(&wallet.address) {
                            eprintln!("Failed to copy address: {}", e);
                        } else {
                            println!("Address copied successfully!");
                        }
                    } else {
                        eprintln!("Failed to initialize clipboard");
                    }
                }
                Command::none()
            }
            Message::CopyPrivateKey => {
                if let Some(wallet) = &self.wallet {
                    println!("Copying private key to clipboard");
                    if let Ok(mut clipboard) = Clipboard::new() {
                        if let Err(e) = clipboard.set_text(&wallet.private_key) {
                            eprintln!("Failed to copy private key: {}", e);
                        } else {
                            println!("Private key copied successfully!");
                        }
                    } else {
                        eprintln!("Failed to initialize clipboard");
                    }
                }
                Command::none()
            }
            Message::CopySeedPhrase => {
                if let Some(wallet) = &self.wallet {
                    if let Some(phrase) = &wallet.seed_phrase {
                        println!("Copying seed phrase to clipboard");
                        if let Ok(mut clipboard) = Clipboard::new() {
                            if let Err(e) = clipboard.set_text(phrase) {
                                eprintln!("Failed to copy seed phrase: {}", e);
                            } else {
                                println!("Seed phrase copied successfully!");
                            }
                        } else {
                            eprintln!("Failed to initialize clipboard");
                        }
                    }
                }
                Command::none()
            }
            Message::CloseQRWindow => {
                self.qr_window_open = false;
                self.qr_data = None;
                self.qr_title = None;
                Command::none()
            }
            // Core features
            Message::GenerateSeedPhrase => {
                self.include_seed_phrase = !self.include_seed_phrase;
                Command::none()
            }
            Message::SaveWallet => {
                if let Some(wallet) = &self.wallet {
                    self.saving = true;
                    let wallet_clone = wallet.clone();
                    let format = self.save_format.clone();
                    let _password = if self.encrypt { self.password.clone() } else { String::new() };
                    Command::perform(
                        async move {
                            wallet_clone.save_to_file(&format).await
                        },
                        |result| {
                            match result {
                                Ok(_) => Message::ClearWallet, // Refresh after save
                                Err(_e) => Message::ClearWallet, // Handle error silently for now
                            }
                        },
                    )
                } else {
                    Command::none()
                }
            }
            Message::ImportWallet => {
                if !self.import_seed_phrase.is_empty() {
                    let phrase = self.import_seed_phrase.clone();
                    let wallet_type = self.wallet_type.clone();
                    Command::perform(
                        async move {
                            crate::wallet::import_wallet(&wallet_type.to_string().to_lowercase(), &phrase, false, "json").await
                        },
                        |result| {
                            match result {
                                Ok(_) => Message::ClearWallet,
                                Err(_e) => Message::ClearWallet,
                            }
                        },
                    )
                } else {
                    Command::none()
                }
            }
            Message::BatchGenerate => {
                self.batch_status_message = None; // Reset old message
                let count: u32 = self.batch_count.parse().unwrap_or(1);
                let wallet_type = self.wallet_type.clone();
                let format = self.save_format.clone();
                let include_seed_phrase = self.include_seed_phrase; // Get value from checkbox
                Command::perform(
                    async move {
                        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
                        let filename = format!("batch_{}_{}_{}.{}", wallet_type.to_string().to_lowercase(), count, timestamp, format);
                        crate::wallet::generate_batch_wallets(&wallet_type.to_string().to_lowercase(), count, include_seed_phrase, true, &format, false, None).await
                            .map(|_| filename)
                    },
                    |result| {
                        match result {
                            Ok(filename) => Message::BatchStatusUpdated(format!("Batch generation successful! File saved: {}", filename)),
                            Err(_e) => Message::BatchStatusUpdated("Batch generation failed!".to_string()),
                        }
                    },
                )
            }
            Message::ImportSeedPhraseChanged(phrase) => {
                self.import_seed_phrase = phrase;
                Command::none()
            }
            Message::BatchCountChanged(count) => {
                self.batch_count = count;
                Command::none()
            }
            Message::SaveFormatChanged(format) => {
                self.save_format = format;
                Command::none()
            }
            Message::BatchStatusUpdated(message) => {
                self.batch_status_message = Some(message);
                Command::none()
            }
            Message::CopySupportWork => {
                if let Some(wallet) = &self.wallet {
                    println!("Copying support my work to clipboard");
                    if let Ok(mut clipboard) = Clipboard::new() {
                        if let Err(e) = clipboard.set_text(&wallet.support_my_work) {
                            eprintln!("Failed to copy support my work: {}", e);
                        } else {
                            println!("Support my work copied successfully!");
                        }
                    } else {
                        eprintln!("Failed to initialize clipboard");
                    }
                }
                Command::none()
            }

        }
    }

    fn view(&self) -> Element<Message> {
        // If QR window is open, display QR code
        if self.qr_window_open {
            if let Some(qr_data) = &self.qr_data {
                let qr_code = QrCode::new(qr_data.as_bytes()).unwrap();
                let width = qr_code.width();
                let colors = qr_code.to_colors();
                let mut ascii_qr = String::new();
                for y in 0..width {
                    for x in 0..width {
                        let idx = y * width + x;
                        ascii_qr.push(if colors[idx] == qrcode::Color::Dark { '█' } else { ' ' });
                    }
                    ascii_qr.push('\n');
                }
                container(
                    column![
                        text(self.qr_title.as_ref().unwrap_or(&"QR Code".to_string())).size(20),
                        vertical_space().height(20),
                        text(ascii_qr).size(8).font(iced::Font::MONOSPACE),
                        vertical_space().height(20),
                        button("Close").on_press(Message::CloseQRWindow),
                    ]
                    .spacing(10)
                    .padding(20),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
            } else {
                container(
                    column![
                        text("No QR data available").size(16),
                        button("Close").on_press(Message::CloseQRWindow),
                    ]
                    .spacing(20)
                    .padding(20),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
            }
        } else {
            // Main GUI layout
            let wallet_types = vec![
                WalletType::Eth, WalletType::Btc, WalletType::Sol,
                WalletType::Polygon, WalletType::Avax, WalletType::Bitcoin,
                WalletType::Ethereum, WalletType::Litecoin, WalletType::Dogecoin,
            ];

            // Control panel (left side) - display all core features simply
            let controls = {
                let wallet_type_picker = pick_list(
                    wallet_types,
                    Some(self.wallet_type.clone()),
                    Message::WalletTypeChanged,
                )
                .placeholder("Select wallet type");

                let encrypt_checkbox = checkbox("Encrypt wallet", self.encrypt)
                    .on_toggle(|_| Message::EncryptChanged(!self.encrypt));

                let password_input = if self.encrypt {
                    text_input("Enter password", &self.password)
                        .on_input(Message::PasswordChanged)
                        .secure(true)
                        .into()
                } else {
                    Element::from(vertical_space())
                };

                let seed_phrase_checkbox = checkbox("Include Seed Phrase", self.include_seed_phrase)
                    .on_toggle(|_| Message::GenerateSeedPhrase);

                let save_format_picker = pick_list(
                    vec!["json".to_string(), "txt".to_string()],
                    Some(self.save_format.clone()),
                    Message::SaveFormatChanged,
                )
                .placeholder("Save format");

                let batch_count_input = text_input("Batch count", &self.batch_count)
                    .on_input(Message::BatchCountChanged)
                    .width(Length::Fill);

                let import_phrase_input = text_input("Import seed phrase", &self.import_seed_phrase)
                    .on_input(Message::ImportSeedPhraseChanged)
                    .width(Length::Fill);

                let generate_button = if self.generating {
                    button("Generating...").width(Length::Fill)
                } else {
                    button("Generate Wallet").on_press(Message::GenerateWallet).width(Length::Fill)
                };

                let clear_button = button("Clear").on_press(Message::ClearWallet).width(Length::Fill);

                // Move batch status message to center of control panel:
                let batch_status_message_element = if let Some(status_msg) = &self.batch_status_message {
                    Element::from(
                        container(
                            text(status_msg).size(8).style(theme::Text::Color(Color::from_rgb(0.2, 0.6, 0.2))).width(Length::Fill)
                        )
                        .padding(6)
                        .style(theme::Container::Custom(Box::new(BatchStatusStyle)))
                    )
                } else {
                    Element::from(vertical_space())
                };

                // Remove scrollable and reduce font size:
                container(
                    column![
                        text("Wallet Type:").size(10),
                        wallet_type_picker,
                        vertical_space().height(5),
                        encrypt_checkbox,
                        password_input,
                        vertical_space().height(5),
                        seed_phrase_checkbox,
                        vertical_space().height(5),
                        text("Save Format:").size(10),
                        save_format_picker,
                        vertical_space().height(5),
                        row![generate_button, clear_button].spacing(10),
                        vertical_space().height(5),
                        text("Batch Generation:").size(8),
                        batch_count_input,
                        button("Generate Batch").on_press(Message::BatchGenerate).width(Length::Fill),
                        batch_status_message_element,
                        vertical_space().height(5),
                        text("Import Wallet:").size(8),
                        import_phrase_input,
                        button("Import").on_press(Message::ImportWallet).width(Length::Fill),
                    ]
                    .spacing(5)
                    .padding(12),
                )
                .width(Length::Fixed(320.0))
                .height(Length::Fill)
                .style(theme::Container::Custom(Box::new(ControlsStyle)))
            };

            // --- Display Panel (Right Side) ---
            let wallet_display = {
                let content: Element<Message> = if let Some(wallet) = &self.wallet {
                    let wallet_info = column![
                        text("Generated Wallet").size(20),
                        vertical_space().height(10),
                        text(format!("Type: {}", wallet.wallet_type)).size(14),
                        vertical_space().height(15),
                        // Address Section with Copy Button
                        text("Address:").size(16),
                        row![
                            text(&wallet.address).size(12).font(iced::Font::MONOSPACE).width(Length::Fill),
                            horizontal_space(),
                            button("Copy").on_press(Message::CopyAddress),
                        ].spacing(10),
                        vertical_space().height(10),
                        // Private Key Section with Copy Button
                        text("Private Key:").size(16),
                        row![
                            text(&wallet.private_key).size(12).font(iced::Font::MONOSPACE).width(Length::Fill),
                            horizontal_space(),
                            button("Copy").on_press(Message::CopyPrivateKey),
                        ].spacing(10),
                        vertical_space().height(10),
                        // Seed Phrase Section (if exists) with Copy Button
                        if let Some(phrase) = &wallet.seed_phrase {
                            column![
                                text("Seed Phrase:").size(16),
                                row![
                                    text(phrase).size(12).font(iced::Font::MONOSPACE).width(Length::Fill),
                                    horizontal_space(),
                                    button("Copy").on_press(Message::CopySeedPhrase),
                                ].spacing(10),
                                vertical_space().height(10),
                            ].into()
                        } else {
                            Element::from(vertical_space())
                        },
                        // QR Code Section (only 1 button per data)
                        text("QR Codes:").size(16),
                        row![
                            column![
                                text("Address QR").size(12),
                                button("View QR").on_press(Message::ShowAddressQR)
                            ],
                            horizontal_space(),
                            column![
                                text("Private Key QR").size(12),
                                button("View QR").on_press(Message::ShowPrivateKeyQR)
                            ],
                            if wallet.seed_phrase.is_some() {
                                column![
                                    text("Seed Phrase QR").size(12),
                                    button("View QR").on_press(Message::ShowSeedPhraseQR)
                                ]
                            } else {
                                column![vertical_space()]
                            }
                        ].spacing(20),
                        vertical_space().height(15),
                        text(format!("Created: {}", wallet.created_at.format("%Y-%m-%d %H:%M:%S UTC"))).size(10),
                        text(format!("Support: {}", wallet.support_my_work)).size(10),
                        button("Copy Support").on_press(Message::CopySupportWork),
                    ]
                    .spacing(5)
                    .padding(20);
                    container(wallet_info)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .into()
                } else {
                    // This section is unchanged
                    container(
                        text("Generate a wallet to see details here.")
                            .size(16)
                            .style(theme::Text::Color(Color::from_rgb(0.5, 0.5, 0.5)))
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .into()
                };

                // This section is unchanged
                container(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(theme::Container::Custom(Box::new(WalletDisplayStyle)))
            };

            // --- Main Layout ---
            let main_layout = column![
                text("Rust Wallet Generator").size(32).width(Length::Fill).horizontal_alignment(iced::alignment::Horizontal::Center),
                text("A secure, offline cryptocurrency wallet generator.").width(Length::Fill).horizontal_alignment(iced::alignment::Horizontal::Center),
                vertical_space().height(20),
                row![
                    controls,
                    wallet_display,
                ].spacing(20).height(Length::Fill),
            ]
            .padding(20)
            .spacing(10)
            .width(Length::Fill)
            .height(Length::Fill);

            container(main_layout)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
        }
    }
}

pub fn run_gui() -> iced::Result {
    WalletGenerator::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(900.0, 700.0),
            ..Default::default()
        },
        ..Default::default()
    })
} 