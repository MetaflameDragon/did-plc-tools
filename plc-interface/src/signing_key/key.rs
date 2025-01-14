use crate::app::AppSection;
use did_key::DidKey;
use egui::{RichText, Ui};
use std::path::Path;

impl SigningKey {
    pub fn generate_keypair() -> anyhow::Result<Self> {
        let (secret, public) = secp256k1::generate_keypair(&mut secp256k1::rand::rngs::OsRng);
        Ok(SigningKey::KeyPair { secret, public })
    }

    pub fn load_keypair(priv_bytes_path: &Path) -> anyhow::Result<Self> {
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
            let did_key = self.as_did_key();
            ui.label(RichText::new(did_key.formatted_value()).monospace());
        });
    }
}

#[derive(Clone)]
pub enum SigningKey {
    KeyPair {
        secret: secp256k1::SecretKey,
        public: secp256k1::PublicKey,
    },
}
