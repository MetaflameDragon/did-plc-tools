use crate::app::AppSection;
use crate::plc_builder::aka::AlsoKnownAsInterface;
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
    rotation_keys: SigningKeyArray<5>,
    verification_methods: HashMap<String, DidKey>,
    services: HashMap<String, PlcService>,
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

            ui.heading("Services:");
        });
    }
}

impl PlcBuilderInterface {
    pub fn with_default_services(mut self, pds_endpoint: Url) -> Self {
        self.services = Self::get_default_services(pds_endpoint);
        self
    }

    fn get_default_services(pds_endpoint: Url) -> HashMap<String, PlcService> {
        HashMap::from([(
            "atproto_pds".to_string(),
            PlcService::new_atproto_pds(pds_endpoint),
        )])
    }
}
