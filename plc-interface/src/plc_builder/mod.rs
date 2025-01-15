use crate::app::AppSection;
use crate::plc_builder::aka::AlsoKnownAsInterface;
use crate::plc_builder::rotation_keys::RotationKeysInterface;
use crate::plc_builder::services::ServicesInterface;
use crate::plc_builder::verification_methods::VerificationMethodsInterface;
use crate::signing_key::SigningKeyArray;
use did_key::DidKey;
use did_plc::{AkaUri, PlcService};
use egui::Ui;
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
        });
    }
}

impl PlcBuilderInterface {
    pub fn with_atproto_pds(mut self, pds_endpoint: Url) -> Self {
        self.services.add_atproto_pds(pds_endpoint);
        self
    }
}
