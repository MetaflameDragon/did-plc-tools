use did_key::{DIDCore, Fingerprint, Generate, KeyPair, PatchedKeyPair, Secp256k1KeyPair, ECDH};
use eframe::Frame;
use egui::{Color32, Context, RichText, Ui};

trait KeyPairGenerator = Generate + ECDH + DIDCore + Fingerprint + Into<KeyPair>;

#[derive(Default)]
pub struct App {
    did_key_generator: DidKeyGenerator,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::panel::CentralPanel::default().show(ctx, |ui| {
            &self.did_key_generator.draw_and_update(ui);
        });
    }
}

pub trait AppSection {
    fn draw_and_update(&mut self, ui: &mut Ui); // TODO: return InnerResponse
}

#[derive(Default)]
struct DidKeyGenerator {
    loaded_did_key: Option<PatchedKeyPair>,
}

impl DidKeyGenerator {
    fn generate_and_load_did_key(&mut self) {
        self.loaded_did_key = Some(generate_new_did_key::<Secp256k1KeyPair>());
    }
}

impl AppSection for DidKeyGenerator {
    fn draw_and_update(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.heading("did:key generator");

            ui.horizontal(|ui| {
                if ui.button("Generate new did:key").clicked() {
                    self.generate_and_load_did_key();
                }
            });

            ui.horizontal(|ui| {
                ui.label(RichText::new("Loaded key: ").strong());
                if let Some(key) = &self.loaded_did_key {
                    let key_str = format!("did:key:{}", key.fingerprint());
                    ui.label(RichText::new(key_str).monospace());
                } else {
                    ui.label(RichText::new("None").italics().weak());
                }
            });
        });
    }
}

fn generate_new_did_key<T: KeyPairGenerator>() -> PatchedKeyPair {
    did_key::generate::<T>(None)
}
