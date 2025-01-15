use crate::app::AppSection;
use crate::signing_key::{SigningKey, SigningKeyContainer};
use anyhow::{bail, Result};
use egui::{RichText, TextBuffer, Ui};
use log::error;
use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
pub struct VerificationMethodsInterface {
    verification_methods: HashMap<String, SigningKey>,
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
        draw_verification_method_set(&mut self.verification_methods, ui);
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
                        self.verification_methods.insert(name, key);
                    }
                    Err(err) => {
                        error!("{}", err);
                    }
                }
            }
        });
    }
}

fn draw_verification_method_set(map: &mut HashMap<String, SigningKey>, ui: &mut Ui) {
    ui.group(|ui| {
        ui.vertical(|ui| {
            if map.is_empty() {
                ui.label(RichText::new("No verification methods").weak().italics());
            } else {
                let mut key_to_remove = None;

                for (name, mut key) in &mut *map {
                    let should_remove = draw_verification_method(&name, &mut key, ui);
                    if should_remove {
                        key_to_remove = Some(name.clone());
                    }
                }

                if let Some(name) = key_to_remove {
                    map.remove(&name);
                }
            }
        })
    });
}

/// Returns true if the X (remove) button is clicked
fn draw_verification_method(name: &str, key: &mut SigningKey, ui: &mut Ui) -> bool {
    let mut should_remove = false;
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            let resp = ui.button("X");
            should_remove = resp.clicked();
            ui.label(&*name);
        });
        key.draw_and_update(ui);
    });

    should_remove
}
