use crate::app::AppSection;
use crate::signing_key::{SigningKey, SigningKeyContainer};
use crate::ui_helpers::hash_map::HashMapRenderer;
use anyhow::{bail, Result};
use egui::{RichText, TextBuffer, Ui};
use log::error;
use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
pub struct VerificationMethodsInterface {
    map_renderer: HashMapRenderer<String, SigningKey>,
    input_fields: InputFields,
}

#[derive(Default, Clone, Debug)]
struct InputFields {
    name: String,
    key: SigningKeyContainer,
}

impl InputFields {
    fn try_get_verification_method(&mut self) -> Result<(String, SigningKey)> {
        if self.name.is_empty() {
            bail!("Name is empty")
        }
        let Some(key) = self.key.take() else {
            bail!("Key is empty")
        };

        Ok((self.name.take(), key))
    }
}

impl AppSection for VerificationMethodsInterface {
    fn draw_and_update(&mut self, ui: &mut Ui) {
        self.map_renderer.draw_and_update(ui);
        self.draw_input_field(ui);
    }
}

impl VerificationMethodsInterface {
    fn draw_input_field(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.text_edit_singleline(&mut self.input_fields.name);
                    self.input_fields.key.draw_and_update(ui);
                });
            });
            if ui.button("Add").clicked() {
                let res = self.input_fields.try_get_verification_method();
                match res {
                    Ok((name, key)) => {
                        self.map_renderer.inner_mut().insert(name, key);
                    }
                    Err(err) => {
                        error!("{}", err);
                    }
                }
            }
        });
    }
}
