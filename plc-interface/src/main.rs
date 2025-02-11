#![feature(never_type)]
#![feature(trait_alias)]
extern crate core;

mod app;
mod plc_builder;
mod ui_helpers;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 600.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    eframe::run_native(
        "PLC Interface",
        native_options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
}
