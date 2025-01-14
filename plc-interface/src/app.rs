use anyhow::Result;
use did_key::DidKey;
use eframe::Frame;
use egui::{Color32, Context, RichText, Ui};
use std::path::Path;

#[derive(Default)]
pub struct App {
    did_key_generator: Vec<SignatureKeyGenerator>,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        App {
            did_key_generator: vec![SignatureKeyGenerator::default(); 5],
            ..Default::default()
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::panel::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                for gen in &mut self.did_key_generator {
                    gen.draw_and_update(ui);
                }
            })
        });
    }
}

pub trait AppSection {
    fn draw_and_update(&mut self, ui: &mut Ui); // TODO: return InnerResponse
}

#[derive(Default, Clone)]
struct SignatureKeyGenerator {
    signature_key: Option<SignatureKey>,
}

impl SignatureKeyGenerator {
    fn generate_and_load_keypair(&mut self) -> Result<()> {
        self.signature_key = Some(generate_keypair()?);
        Ok(())
    }

    fn load_keypair_from_file(&mut self, path: &Path) {}
}

impl AppSection for SignatureKeyGenerator {
    fn draw_and_update(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if let Some(key) = &self.signature_key {
                if ui.button("X").clicked() {
                    self.signature_key = None;
                } else {
                    let key_str = format!("did:key:{}", key.as_did_key().formatted_value());
                    ui.label(RichText::new(key_str).monospace());
                }
            } else {
                if ui.button("New").clicked() {
                    self.generate_and_load_keypair();
                }
                if ui.button("Load").clicked() {}
            }
        });
    }
}

#[derive(Clone)]
enum SignatureKey {
    KeyPair {
        secret: secp256k1::SecretKey,
        public: secp256k1::PublicKey,
    },
}

impl SignatureKey {
    pub fn as_did_key(&self) -> DidKey {
        match self {
            SignatureKey::KeyPair { public, .. } => public.into(),
        }
    }
}

fn generate_keypair() -> Result<SignatureKey> {
    let (secret, public) = secp256k1::generate_keypair(&mut secp256k1::rand::rngs::OsRng);
    Ok(SignatureKey::KeyPair { secret, public })
}

fn load_keypair(priv_bytes_path: &Path) -> Result<SignatureKey> {
    Ok(todo!())
}
