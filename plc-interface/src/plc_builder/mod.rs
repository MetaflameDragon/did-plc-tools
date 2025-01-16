use crate::app::AppSection;
use crate::plc_builder::aka::AlsoKnownAsInterface;
use crate::plc_builder::rotation_keys::RotationKeysInterface;
use crate::plc_builder::services::ServicesInterface;
use crate::plc_builder::verification_methods::VerificationMethodsInterface;
use crate::signing_key::{SigningKey, SigningKeyArray};
use did_key::DidKey;
use did_plc::{AkaUri, PlcService, UnsignedPlcOperation};
use egui::Ui;
use log::info;
use std::collections::HashMap;
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
}

impl AppSection for PlcBuilderInterface {
    fn draw_and_update(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.heading("Also known as:");
            self.also_known_as.draw_and_update(ui);

            ui.heading("Rotation keys:");
            self.rotation_keys.draw_and_update(ui);

            ui.heading("Verification methods:");
            self.verification_methods.draw_and_update(ui);

            ui.heading("Services:");
            self.services.draw_and_update(ui);

            ui.group(|ui| self.draw_action_column(ui));
        });
    }
}

impl PlcBuilderInterface {
    pub fn with_atproto_pds(mut self, pds_endpoint: Url) -> Self {
        self.services.add_atproto_pds(pds_endpoint);
        self
    }

    fn get_unsigned_plc_op_genesis(&self) -> Option<UnsignedPlcOperation> {
        Some(UnsignedPlcOperation::new_genesis(
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
            self.also_known_as.get_aka_uris(),
            self.services.get_map().clone(),
        ))
    }

    fn draw_action_column(&self, ui: &mut Ui) {
        if ui.button("Print unsigned PLC Operation JSON").clicked() {
            let plc_op = self.get_unsigned_plc_op_genesis();
            if let Some(plc_op) = plc_op {
                info!(
                    "{}",
                    serde_json::ser::to_string_pretty(&plc_op)
                        .unwrap_or("Failed to serialize plc operation".to_string())
                );
            }
        }
    }
}
