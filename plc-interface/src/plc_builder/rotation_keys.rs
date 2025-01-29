use anyhow::{Context, Result};
use did_key::DidKey;
use egui::{Color32, TextEdit, Ui, Widget};
use itertools::Itertools;

use crate::app::key_store::KeyStore;

const ROTATION_KEY_COUNT_MAX: usize = 5;

#[derive(Clone, Debug)]
pub struct RotationKeySetInterface {
    rotation_keys: Vec<String>,
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
            let loaded_keys: Vec<_> = keystore.keys().iter().map(|k| k.as_did_key()).collect();

            enum RotKey {
                Invalid,
                NotOwned(DidKey),
                Owned(DidKey),
            }

            for rot_key_str in &mut self.rotation_keys {
                ui.horizontal(|ui| {
                    let rot_key = {
                        if let Ok(key) = DidKey::try_from(rot_key_str.clone()) {
                            if loaded_keys.contains(&key) {
                                RotKey::Owned(key)
                            } else {
                                RotKey::NotOwned(key)
                            }
                        } else {
                            RotKey::Invalid
                        }
                    };

                    if let RotKey::Owned(key) = &rot_key {
                        ui.radio_value(&mut self.selected_key, Some(key.clone()), "")
                    } else {
                        ui.add_enabled_ui(false, |ui| ui.radio(false, "")).inner
                    };

                    let mut key_field = TextEdit::singleline(rot_key_str);
                    key_field = match &rot_key {
                        RotKey::Invalid => key_field.text_color(Color32::DARK_RED),
                        RotKey::NotOwned(_) => key_field.text_color(Color32::LIGHT_GRAY),
                        RotKey::Owned(_) => key_field.text_color(Color32::DARK_GREEN),
                    };

                    key_field.ui(ui);
                });
            }
        });
    }

    pub fn try_get_keys(&self) -> Result<Vec<DidKey>> {
        self.rotation_keys
            .iter()
            .cloned()
            .filter(|k| !k.is_empty())
            .map(DidKey::try_from)
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
            rotation_keys: keys_str.into_iter().collect(),
            selected_key: None,
        }
    }

    pub fn contains(&self, key: &DidKey) -> bool {
        self.rotation_keys
            .iter()
            .cloned()
            .contains(key.formatted_value())
    }

    /// Returns a reference to the selected signing key
    ///
    /// The selected key may have a stale value (referencing a removed or no-longer-owned key),
    /// always make sure to validate against owned and included rotation keys
    pub fn try_get_selected_key(&self) -> Option<&DidKey> {
        self.selected_key.as_ref()
    }
}
