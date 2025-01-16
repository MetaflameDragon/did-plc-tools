use crate::app::AppSection;
use did_plc::PlcService;
use egui::{RichText, Ui};
use std::collections::HashMap;
use url::Url;
use crate::ui_helpers::hash_map::HashMapRenderer;

#[derive(Default, Clone, Debug)]
pub struct ServicesInterface {
    services: HashMapRenderer<String, PlcService>,
}

impl AppSection for ServicesInterface {
    fn draw_and_update(&mut self, ui: &mut Ui) {
        self.services.draw_and_update(ui);
    }
}

impl ServicesInterface {
    pub fn add_atproto_pds(&mut self, pds_endpoint: Url) {
        self.services.insert(
            "atproto_pds".to_string(),
            PlcService::new_atproto_pds(pds_endpoint),
        );
    }
}

impl AppSection for PlcService {
    fn draw_and_update(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.label(RichText::new(&self.r#type).italics().weak());
            ui.label(RichText::new(&self.endpoint.to_string()).underline());
        });
    }
}