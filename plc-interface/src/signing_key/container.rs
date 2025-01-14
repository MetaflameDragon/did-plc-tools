use crate::{app::AppSection, signing_key::key::SigningKey};
use derive_more::{Deref, DerefMut};
use egui::Ui;

#[repr(transparent)]
#[derive(Clone, Default, Deref, DerefMut)]
pub struct SigningKeyContainer(
    #[deref]
    #[deref_mut]
    Option<SigningKey>,
);

impl AppSection for SigningKeyContainer {
    fn draw_and_update(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if let Some(ref mut key) = **self {
                if ui.button("X").clicked() {
                    **self = None;
                } else {
                    key.draw_and_update(ui);
                }
            } else {
                if ui.button("New").clicked() {
                    **self = SigningKey::generate_keypair().ok();
                }
                if ui.button("Load").clicked() {}
            }
        });
    }
}
