use crate::app::AppSection;
use crate::plc_builder::aka::AlsoKnownAsInterface;
use crate::plc_builder::rotation_keys::RotationKeysInterface;
use crate::plc_builder::services::ServicesInterface;
use crate::plc_builder::verification_methods::VerificationMethodsInterface;
use crate::signing_key::{CryptoKey, CryptoKeyContainer, SigningKeyArray};
use anyhow::{anyhow, Context, Result};
use did_key::DidKey;
use did_plc::{AkaUri, PlcService, SignedPlcOperation, UnsignedPlcOperation};
use egui::Ui;
use log::{error, info};
use secp256k1::Keypair;
use std::collections::HashMap;
use std::ops::Deref;
use url::Url;

mod aka;
mod rotation_keys;
mod services;
mod verification_methods;

#[derive(Default, Clone, Debug)]
pub struct PlcBuilderInterface {
    also_known_as: AlsoKnownAsInterface,
    rotation_keys: RotationKeysInterface,
    verification_methods: VerificationMethodsInterface,
    services: ServicesInterface,
    prev: Option<String>,

    signing_key_selector: SigningKeySelector,
}

impl AppSection for PlcBuilderInterface {
    fn draw_and_update(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.heading("Also known as:");
            self.also_known_as.draw_and_update(ctx, ui);

            ui.heading("Rotation keys:");
            self.rotation_keys.draw_and_update(ctx, ui);

            ui.heading("Verification methods:");
            self.verification_methods.draw_and_update(ctx, ui);

            ui.heading("Services:");
            self.services.draw_and_update(ctx, ui);

            ui.group(|ui| self.draw_action_column(ctx, ui));
        });
    }
}

impl PlcBuilderInterface {
    pub fn with_atproto_pds(mut self, pds_endpoint: Url) -> Self {
        self.services.add_atproto_pds(pds_endpoint);
        self
    }

    fn get_unsigned_plc_op_genesis(&self) -> Result<UnsignedPlcOperation> {
        Ok(UnsignedPlcOperation::new_genesis(
            self.rotation_keys
                .keys()
                .iter()
                .filter_map(|k| k.as_ref().map(|k| k.as_did_key()))
                .collect(),
            self.verification_methods
                .get_map()
                .iter()
                .map(|(key, value)| (key.clone(), value.as_did_key()))
                .collect(),
            self.also_known_as
                .get_aka_uris()
                .context("Failed to parse AkaUris")?,
            self.services.get_map().clone(),
        ))
    }

    fn draw_action_column(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        if ui.button("Print unsigned PLC Operation JSON").clicked() {
            let plc_op = self.get_unsigned_plc_op_genesis();
            match plc_op {
                Ok(plc_op) => {
                    let json = serde_json::ser::to_string_pretty(&plc_op)
                        .unwrap_or("Failed to serialize plc operation".to_string());
                    info!("{json}");
                }
                Err(err) => {
                    error!("{err}")
                }
            }
        }

        self.signing_key_selector
            .ui(ctx, ui, self.rotation_keys.keys().deref().deref());

        if ui.button("Sign operation").clicked() {
            let plc_op = self.get_unsigned_plc_op_genesis();
            let signing_key = self
                .rotation_keys
                .keys()
                .iter()
                .filter_map(|cont| cont.deref().as_ref())
                .next();

            let signed_op = match plc_op {
                Ok(plc_op) => match signing_key {
                    None => Err(anyhow!("Missing rotation keys")),
                    Some(signing_key) => match signing_key.keypair() {
                        None => Err(anyhow!("Signing key is not owned (no private part)")),
                        Some(keypair) => Ok(plc_op.sign(&keypair.secret_key())),
                    },
                },
                Err(err) => Err(err),
            };

            match signed_op {
                Ok(signed_op) => {
                    let did_plc = signed_op.get_did_plc();
                    let json = serde_json::ser::to_string_pretty(&signed_op)
                        .unwrap_or("Failed to serialize signed plc operation".to_string());
                    info!("{json}");
                    info!("{did_plc}");
                }
                Err(err) => {
                    error!("{err}");
                }
            }
        }
    }
}

#[derive(Default, Clone, Debug)]
struct SigningKeySelector {
    key_index: usize,
}

impl SigningKeySelector {
    fn ui(&mut self, _ctx: &egui::Context, ui: &mut Ui, rotation_keys: &[CryptoKeyContainer]) {
        egui::ComboBox::from_label("Signing key").show_index(
            ui,
            &mut self.key_index,
            rotation_keys.len(),
            |i| {
                rotation_keys
                    .get(i)
                    .map(|k| k.try_get_did_key())
                    .flatten()
                    .map_or("[none]".to_string(), |k| k.multibase_value().to_string())
            },
        );
    }
}
