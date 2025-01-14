use ::core::default::Default;
use anyhow::Result;
use derive_more::{Deref, DerefMut};
use did_key::DidKey;
use eframe::Frame;
use egui::{Color32, Context, RichText, Ui};
use std::path::Path;

#[derive(Default)]
pub struct App {
    did_keys: Vec<SigningKeyContainer>,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        App {
            did_keys: vec![Default::default(); 5],
            ..Default::default()
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::panel::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                for key in &mut self.did_keys {
                    key.draw_and_update(ui);
                }
            })
        });
    }
}

pub trait AppSection {
    fn draw_and_update(&mut self, ui: &mut Ui); // TODO: return InnerResponse
}

impl SigningKey {
    fn generate_keypair() -> Result<Self> {
        let (secret, public) = secp256k1::generate_keypair(&mut secp256k1::rand::rngs::OsRng);
        Ok(SigningKey::KeyPair { secret, public })
    }

    fn load_keypair(priv_bytes_path: &Path) -> Result<Self> {
        Ok(todo!())
    }
    pub fn as_did_key(&self) -> DidKey {
        match self {
            SigningKey::KeyPair { public, .. } => public.into(),
        }
    }
}

impl AppSection for SigningKey {
    fn draw_and_update(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let key_str = format!("did:key:{}", self.as_did_key().formatted_value());
            ui.label(RichText::new(key_str).monospace());
        });
    }
}

#[repr(transparent)]
#[derive(Clone, Default, Deref, DerefMut)]
struct SigningKeyContainer(
    #[deref]
    #[deref_mut]
    Option<SigningKey>,
);

impl AppSection for SigningKeyContainer {
    fn draw_and_update(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if let Some(ref mut key) = **self {
                if ui.button("X").clicked() {
                    **self = None;
                } else {
                    key.draw_and_update(ui);
                }
            } else {
                if ui.button("New").clicked() {
                    **self = SigningKey::generate_keypair().ok();
                }
                if ui.button("Load").clicked() {}
            }
        });
    }
}

#[derive(Clone)]
enum SigningKey {
    KeyPair {
        secret: secp256k1::SecretKey,
        public: secp256k1::PublicKey,
    },
}
