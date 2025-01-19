use crate::app::AppSection;
use crate::signing_key::SigningKeyArray;
use egui::Ui;

#[derive(Default, Clone, Debug)]
pub struct RotationKeysInterface {
    rotation_keys: SigningKeyArray<5>,
}

impl AppSection for RotationKeysInterface {
    fn draw_and_update(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        self.rotation_keys.draw_and_update(ctx, ui)
    }
}

impl RotationKeysInterface {
    pub fn keys(&self) -> &SigningKeyArray<5> {
        &self.rotation_keys
    }
}
