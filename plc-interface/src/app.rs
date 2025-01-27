use std::str::FromStr;

use ::core::default::Default;
use eframe::{Frame, Storage};
use egui::{Context, Ui};
use url::Url;

use crate::app::key_store::KeyStoreInterface;
use crate::plc_builder::PlcBuilderInterface;

mod key_store;

pub struct App {
    key_store: KeyStoreInterface,
    plc_builder: PlcBuilderInterface,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let storage = cc.storage;

        App {
            key_store: init_key_store(storage),
            plc_builder: PlcBuilderInterface::default()
                .with_atproto_pds(Url::parse("https://pds.domain.placeholder").unwrap()),
        }
    }
}

fn init_key_store(storage: Option<&dyn Storage>) -> KeyStoreInterface {
    fn get_key_store_dir(storage: Option<&dyn Storage>) -> Option<String> {
        Some(match storage?.get_string("key_store_dir") {
            Some(value) => value,
            None => {
                let mut path = std::env::current_dir().ok()?;
                path.push(".key_store");
                path.to_str()?.to_owned()
            }
        })
    }

    KeyStoreInterface::new(get_key_store_dir(storage).unwrap_or("".to_owned()))
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::panel::CentralPanel::default().show(ctx, |ui| {
            ui.group(|ui| {
                ui.heading("Key Store");
                self.key_store.ui(ui);
            });
            self.plc_builder.draw_and_update(ctx, ui)
        });
    }
}

pub trait AppSection {
    fn draw_and_update(&mut self, ctx: &Context, ui: &mut Ui); // TODO: return InnerResponse
}
