use crate::signing_key::SigningKeyArray;
use ::core::default::Default;
use eframe::Frame;
use egui::{Context, Ui};
#[derive(Default)]
pub struct App {
    did_keys: SigningKeyArray<5>,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        App {
            ..Default::default()
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::panel::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                self.did_keys.draw_and_update(ui);
            })
        });
    }
}

pub trait AppSection {
    fn draw_and_update(&mut self, ui: &mut Ui); // TODO: return InnerResponse
}
