use std::borrow::Borrow;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Context;
use bincode::Options;
use derive_more::From;
use did_key::DidKey;
use ecdsa::SigningKey;
use egui::{RichText, Ui};
use k256::Secp256k1;
use log::{error, info};
use serde::Serialize;

use crate::app::AppSection;

// impl CryptoKey {
//     pub fn new_keypair(keypair: secp256k1::Keypair) -> Self {
//         Self {
//             key: KeyType::Signing(SigningKeyWrapper::Secp256k1(keypair)),
//         }
//     }
//
//     pub fn new_verify_only(public_key: secp256k1::PublicKey) -> Self {
//         Self {
//             key: KeyType::VerifyOnly(VerificationKeyWrapper::Secp256k1(public_key)),
//         }
//     }
//
//     fn get_bincode_options() -> impl Options {
//         bincode::DefaultOptions::new().reject_trailing_bytes()
//     }
//
//     pub fn generate_keypair() -> anyhow::Result<Self> {
//         let rng = &mut rand::rngs::OsRng;
//         let keypair = SigningKey::<Secp256k1>::random(rng);
//         Ok(Self::new_keypair(keypair))
//     }
//
//     pub fn load_keypair(priv_bytes_path: &Path) -> anyhow::Result<Self> {
//         let bytes = std::fs::read(priv_bytes_path)?;
//
//         let keypair = Self::get_bincode_options().deserialize(&bytes)?;
//
//         Ok(Self::new_keypair(keypair))
//     }
//     pub fn as_did_key(&self) -> DidKey {
//         self.public_key().public_key().into()
//     }
//
//     pub fn save_keypair(&self, priv_bytes_path: &Path) -> anyhow::Result<()> {
//         let bytes = Self::get_bincode_options().serialize(
//             &self
//                 .keypair()
//                 .context("Signing key is not owned (no secret part)")?,
//         )?;
//
//         let mut file = std::fs::File::create_new(priv_bytes_path)?;
//         file.write_all(bytes.as_ref())?;
//
//         Ok(())
//     }
//
//     pub fn public_key(&self) -> VerificationKeyWrapper {
//         match &self.key {
//             KeyType::Signing(owned) => owned.keypair().public_key().into(),
//             KeyType::VerifyOnly(verification_key) => verification_key.to_owned(),
//         }
//     }
//
//     pub fn signing_key(&self) -> Option<SigningKeyWrapper> {
//         match &self.key {
//             KeyType::Signing(signing_key) => Some(signing_key.to_owned()),
//             KeyType::VerifyOnly(_) => None,
//         }
//     }
//
//     pub fn keypair(&self) -> Option<secp256k1::Keypair> {
//         Some(self.signing_key()?.keypair().to_owned())
//     }
// }
//
// impl AppSection for CryptoKey {
//     fn draw_and_update(&mut self, _ctx: &egui::Context, ui: &mut Ui) {
//         ui.horizontal(|ui| {
//             let did_key = self.as_did_key();
//
//             ui.label(RichText::new(did_key.formatted_value()).monospace());
//         });
//     }
// }
//
// #[derive(Debug, Clone, From)]
// pub enum SigningKeyWrapper {
//     Secp256k1(#[from] secp256k1::Keypair),
// }
//
// impl SigningKeyWrapper {
//     pub fn keypair(&self) -> &secp256k1::Keypair {
//         match self {
//             SigningKeyWrapper::Secp256k1(keypair) => keypair,
//         }
//     }
// }
//
// #[derive(Debug, Clone, From)]
// pub enum VerificationKeyWrapper {
//     Secp256k1(#[from] secp256k1::PublicKey),
// }
//
// impl VerificationKeyWrapper {
//     pub fn public_key(&self) -> &secp256k1::PublicKey {
//         match self {
//             VerificationKeyWrapper::Secp256k1(public_key) => &public_key,
//         }
//     }
// }
//
// #[derive(Clone, Debug)]
// enum KeyType {
//     Signing(SigningKeyWrapper),
//     VerifyOnly(VerificationKeyWrapper),
// }
//
// #[derive(Clone, Debug)]
// pub struct CryptoKey {
//     key: KeyType,
// }
