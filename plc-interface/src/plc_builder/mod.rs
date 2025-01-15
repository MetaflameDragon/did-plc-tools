use crate::app::AppSection;
use crate::signing_key::SigningKeyArray;
use did_key::DidKey;
use did_plc::{AkaUri, PlcService};
use egui::Ui;
use std::collections::HashMap;
use derive_new::new;

#[derive(Default, Clone, Debug)]
pub struct PlcBuilderInterface {
    also_known_as: Vec<AkaUri>,
    rotation_keys: SigningKeyArray<5>,
    verification_methods: HashMap<String, DidKey>,
    services: HashMap<String, PlcService>,
    prev: Option<String>,
}

impl AppSection for PlcBuilderInterface {
    fn draw_and_update(&mut self, ui: &mut Ui) {
        ui.heading("Rotation Keys");
        ui.vertical(|ui| {
            self.rotation_keys.draw_and_update(ui);
        });
    }
}

impl PlcBuilderInterface {
    pub fn with_default_services(mut self) -> Self {
        self.services = Self::get_default_services();
        self
    }

    fn get_default_services() -> HashMap<String, PlcService> {
        HashMap::from([(
            "atproto_pds".to_string(),
            PlcService {
                r#type: "AtprotoPersonalDataServer".to_string(),
                endpoint: "https://scalytooth.metaflame.dev".to_string(),
            },
        )])
    }
}
