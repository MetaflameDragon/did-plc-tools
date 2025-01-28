use std::collections::HashMap;

use anyhow::{bail, Result};
use did_key::DidKey;
use egui::{TextBuffer, Ui};
use log::error;

use crate::ui_helpers::hash_map::HashMapRenderer;

#[derive(Default, Clone, Debug)]
pub struct VerificationMethodsInterface {
    map_renderer: HashMapRenderer<String, DidKey>,
    input_fields: InputFields,
}

#[derive(Default, Clone, Debug)]
struct InputFields {
    name: String,
    key_str: String,
}

impl InputFields {
    fn try_get_verification_method(&mut self) -> Result<(String, DidKey)> {
        if self.name.is_empty() {
            bail!("Name is empty")
        }

        Ok((self.name.take(), DidKey::try_from(self.key_str.to_owned())?))
    }
}

impl VerificationMethodsInterface {
    pub fn ui(&mut self, ui: &mut Ui) {
        self.map_renderer.ui(ui);
        self.draw_input_field(ui);
    }

    pub fn get_map(&self) -> &HashMap<String, DidKey> {
        self.map_renderer.inner()
    }

    fn draw_input_field(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.text_edit_singleline(&mut self.input_fields.name);
                    ui.text_edit_singleline(&mut self.input_fields.key_str);
                });
            });
            if ui.button("Add").clicked() {
                let res = self.input_fields.try_get_verification_method();
                match res {
                    Ok((name, key)) => {
                        self.map_renderer.insert(name, key);
                    }
                    Err(err) => {
                        error!("{}", err);
                    }
                }
            }
        });
    }

    pub fn set_map(&mut self, map: HashMap<String, DidKey>) {
        *self.map_renderer.inner_mut() = map;
    }

    pub fn from_map(map: HashMap<String, DidKey>) -> Self {
        let mut map_renderer = HashMapRenderer::default();
        *map_renderer.inner_mut() = map;
        Self {
            map_renderer,
            input_fields: InputFields::default(),
        }
    }
}
