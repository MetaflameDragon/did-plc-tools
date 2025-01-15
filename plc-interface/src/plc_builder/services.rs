use crate::app::AppSection;
use did_plc::PlcService;
use egui::Ui;
use std::collections::HashMap;
use url::Url;

#[derive(Default, Clone, Debug)]
pub struct ServicesInterface {
    services: HashMap<String, PlcService>,
}

impl AppSection for ServicesInterface {
    fn draw_and_update(&mut self, ui: &mut Ui) {}
}

impl ServicesInterface {
    pub fn add_atproto_pds(&mut self, pds_endpoint: Url) {
        self.services.insert(
            "atproto_pds".to_string(),
            PlcService::new_atproto_pds(pds_endpoint),
        );
    }
}
