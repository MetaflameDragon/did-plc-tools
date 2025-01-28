use anyhow::{Context, Result};
use derive_new::new;
use did_key::DidKey;
use egui::{Ui, Widget};

use crate::app::key_store::KeyStore;

const ROTATION_KEY_COUNT_MAX: usize = 5;

#[derive(Clone, Debug)]
pub struct RotationKeySetInterface {
    rotation_keys: Vec<RotationKeyInterface>,
    selected_key: Option<DidKey>,
}

impl Default for RotationKeySetInterface {
    fn default() -> Self {
        Self {
            rotation_keys: vec![Default::default(); 5],
            selected_key: None,
        }
    }
}

impl RotationKeySetInterface {
    pub fn ui(&mut self, ui: &mut Ui, keystore: &KeyStore) {
        ui.vertical(|ui| {
            for key_interface in &mut self.rotation_keys {
                key_interface.ui(ui, false);
            }
        });
    }

    pub fn try_get_keys(&self) -> Result<Vec<DidKey>> {
        self.rotation_keys
            .iter()
            .cloned()
            .filter_map(|key_interface| key_interface.try_get_key())
            .map(|res| res.context("Failed to parse did:key"))
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
            rotation_keys: keys_str
                .into_iter()
                .map(|str| RotationKeyInterface::new(str))
                .collect(),
            selected_key: None,
        }
    }
}

#[derive(Debug, Default, Clone, new)]
struct RotationKeyInterface {
    key_str: String,
}

impl RotationKeyInterface {
    pub fn try_get_key(&self) -> Option<Result<DidKey, did_key::Error>> {
        if self.key_str.is_empty() {
            None
        } else {
            Some(DidKey::try_from(self.key_str.clone()))
        }
    }

    pub fn ui(&mut self, ui: &mut Ui, is_selected: bool) -> bool {
        ui.horizontal(|ui| {
            let radio = egui::RadioButton::new(is_selected, "");
            let radio_resp = radio.ui(ui);

            ui.text_edit_singleline(&mut self.key_str);

            radio_resp.clicked()
        })
        .inner
    }
}
