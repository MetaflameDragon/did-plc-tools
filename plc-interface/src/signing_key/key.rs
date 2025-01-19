use crate::app::AppSection;
use did_key::DidKey;
use egui::{RichText, Ui};
use log::{error, info};
use serde::Serialize;
use std::io::Write;
use std::path::{Path, PathBuf};

impl SigningKey {
    pub fn generate_keypair() -> anyhow::Result<Self> {
        let global_context = secp256k1::global::SECP256K1;
        let rng = &mut secp256k1::rand::rngs::OsRng;
        let keypair = secp256k1::Keypair::new(global_context, rng);
        Ok(SigningKey::KeyPair { keypair })
    }

    pub fn load_keypair(priv_bytes_path: &Path) -> anyhow::Result<Self> {
        let bytes = std::fs::read(priv_bytes_path)?;
        let keypair = bincode::deserialize(&bytes)?;

        Ok(SigningKey::KeyPair { keypair })
    }
    pub fn as_did_key(&self) -> DidKey {
        match self {
            SigningKey::KeyPair { keypair } => (&keypair.public_key()).into(),
        }
    }

    pub fn save_keypair(&self, priv_bytes_path: &Path) -> anyhow::Result<()> {
        let bytes = match self {
            SigningKey::KeyPair { keypair } => bincode::serialize(&keypair)?,
        };

        let mut file = std::fs::File::create_new(priv_bytes_path)?;
        file.write_all(bytes.as_ref())?;

        Ok(())
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

#[derive(Clone, Debug)]
pub enum SigningKey {
    KeyPair { keypair: secp256k1::Keypair },
}
