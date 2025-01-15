use std::collections::HashMap;
use egui::Ui;
use crate::app::AppSection;
use crate::signing_key::SigningKey;

#[derive(Default, Clone, Debug)]
pub struct VerificationMethodsInterface {
    verification_methods: HashMap<String, SigningKey>,
}

impl AppSection for VerificationMethodsInterface {
    fn draw_and_update(&mut self, ui: &mut Ui) {

    }
}