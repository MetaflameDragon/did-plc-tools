use anyhow::{Context, Result};
use did_key::DidKey;
use egui::Ui;

use crate::app::AppSection;

const ROTATION_KEY_COUNT_MAX: usize = 5;

#[derive(Clone, Debug)]
pub struct RotationKeysInterface {
    rotation_keys_str: Vec<String>,
}

impl Default for RotationKeysInterface {
    fn default() -> Self {
        Self {
            rotation_keys_str: vec![String::new(); 5],
        }
    }
}

impl AppSection for RotationKeysInterface {
    fn draw_and_update(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        ui.vertical(|ui| {
            for key in &mut self.rotation_keys_str {
                ui.text_edit_singleline(key);
            }
        });
    }
}

impl RotationKeysInterface {
    pub fn try_get_keys(&self) -> Result<Vec<DidKey>> {
        self.rotation_keys_str
            .iter()
            .cloned()
            .filter(|k| !k.is_empty())
            .map(|str| DidKey::try_from(str).context("Failed to parse did:key"))
            .collect::<Result<Vec<_>>>()
    }

    pub fn from_keys(keys: impl IntoIterator<Item = DidKey>) -> Self {
        let mut keys_str = keys
            .into_iter()
            .take(5)
            .map(|k| k.formatted_value().to_owned())
            .collect::<Vec<_>>();
        while keys_str.len() < ROTATION_KEY_COUNT_MAX {
            keys_str.push(String::new());
        }

        Self {
            rotation_keys_str: keys_str,
        }
    }
}
