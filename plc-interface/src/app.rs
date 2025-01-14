use crate::signing_key::SigningKeyContainer;
use ::core::default::Default;
use eframe::Frame;
use egui::{Context, Ui};
#[derive(Default)]
pub struct App {
    did_keys: Vec<SigningKeyContainer>,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        App {
            did_keys: vec![Default::default(); 5],
            ..Default::default()
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::panel::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                for key in &mut self.did_keys {
                    key.draw_and_update(ui);
                }
            })
        });
    }
}

pub trait AppSection {
    fn draw_and_update(&mut self, ui: &mut Ui); // TODO: return InnerResponse
}
