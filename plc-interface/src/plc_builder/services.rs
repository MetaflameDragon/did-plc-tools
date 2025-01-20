use crate::app::AppSection;
use crate::ui_helpers::hash_map::HashMapRenderer;
use did_plc::PlcService;
use egui::{RichText, Ui, Widget};
use std::collections::HashMap;
use url::Url;

#[derive(Clone, Debug)]
pub struct ServicesInterface {
    services: HashMapRenderer<String, PlcServiceInterface>,
}

impl Default for ServicesInterface {
    fn default() -> Self {
        let mut services: HashMapRenderer<_, _> = Default::default();
        services.allow_remove = false;
        Self { services }
    }
}

impl AppSection for ServicesInterface {
    fn draw_and_update(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        self.services.draw_and_update(ctx, ui);
    }
}

impl ServicesInterface {
    pub fn add_atproto_pds(&mut self, pds_endpoint: Url) {
        self.services.insert(
            "atproto_pds".to_string(),
            PlcService::new_atproto_pds(pds_endpoint).into(),
        );
    }

    pub fn get_map(&self) -> &HashMap<String, PlcServiceInterface> {
        self.services.inner()
    }
}

#[derive(Clone, Debug)]
pub struct PlcServiceInterface {
    r#type: String,
    endpoint_buffer: String,
}

impl From<PlcService> for PlcServiceInterface {
    fn from(plc_service: PlcService) -> Self {
        Self {
            r#type: plc_service.r#type,
            endpoint_buffer: plc_service.endpoint.to_string(),
        }
    }
}

impl TryInto<PlcService> for PlcServiceInterface {
    type Error = anyhow::Error;

    fn try_into(self) -> std::result::Result<PlcService, Self::Error> {
        Ok(PlcService {
            r#type: self.r#type,
            endpoint: Url::parse(self.endpoint_buffer.as_str())?,
        })
    }
}

impl AppSection for PlcServiceInterface {
    fn draw_and_update(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.label(RichText::new(&self.r#type).italics().weak());

            egui::TextEdit::singleline(&mut self.endpoint_buffer)
                .frame(true)
                .interactive(true)
                .ui(ui);
        });
    }
}
