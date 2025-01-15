use crate::plc_builder::PlcBuilderInterface;
use ::core::default::Default;
use eframe::Frame;
use egui::{Context, Ui};
use std::str::FromStr;
use url::Url;

#[derive(Default)]
pub struct App {
    plc_builder: PlcBuilderInterface,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let pds_endpoint = Url::from_str("https://scalytooth.metaflame.dev")
            .expect("Failed to parse PDS endpoint URL");
        App {
            plc_builder: PlcBuilderInterface::default().with_atproto_pds(pds_endpoint),
            ..Default::default()
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::panel::CentralPanel::default().show(ctx, |ui| self.plc_builder.draw_and_update(ui));
    }
}

pub trait AppSection {
    fn draw_and_update(&mut self, ui: &mut Ui); // TODO: return InnerResponse
}
