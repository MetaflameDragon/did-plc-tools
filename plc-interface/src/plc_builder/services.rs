use std::collections::HashMap;
use crate::app::AppSection;
use crate::ui_helpers::hash_map::HashMapRenderer;
use did_plc::PlcService;
use egui::{RichText, Ui};
use url::Url;

#[derive(Clone, Debug)]
pub struct ServicesInterface {
    services: HashMapRenderer<String, PlcService>,
}

impl Default for ServicesInterface {
    fn default() -> Self {
        let mut services: HashMapRenderer<_, _> = Default::default();
        services.allow_remove = false;
        Self { services }
    }
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

    pub fn get_map(&self) -> &HashMap<String, PlcService> {
        self.services.inner()
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
