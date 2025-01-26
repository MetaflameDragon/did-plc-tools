use std::iter;
use std::path::PathBuf;

use derive_new::new;
use did_plc::{PlcBlessedSigningKey, PlcBlessedSigningKeyBox};
use ecdsa::SigningKey;
use egui::{Button, CollapsingHeader, RichText, Ui};
use k256::Secp256k1;

pub struct KeyStoreInterface {
    key_store_dir_str: String,
    store: KeyStore,
}

impl KeyStoreInterface {
    pub fn new(key_store_dir_str: String) -> Self {
        let mut store = KeyStore::new(key_store_dir_str.clone());
        store.refresh();

        Self {
            key_store_dir_str,
            store,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let refresh_button = ui.small_button(crate::ui_helpers::emoji::COUNTERCLOCKWISE_ARROWS);
            if refresh_button.clicked() {
                self.store.refresh();
            };
            ui.text_edit_singleline(&mut self.key_store_dir_str);
        });
        let keys_header = format!("Keys ({})", &self.store.loaded_keys.len());
        CollapsingHeader::new(keys_header).show(ui, |ui| {
            ui.vertical(|ui| {
                for key in &self.store.loaded_keys {
                    let formatted_value = key.as_did_key().formatted_value().to_owned();
                    let label = RichText::new(formatted_value);
                    ui.label(label.monospace());
                }
            });
        });
    }
}

#[derive(new)]
struct KeyStore {
    #[new(into)]
    key_store_path: PathBuf,
    #[new(default)]
    loaded_keys: Vec<PlcBlessedSigningKeyBox>,
}

impl KeyStore {
    pub fn set_dir(&mut self, dir_str: impl Into<PathBuf>) {
        self.key_store_path = dir_str.into();
    }

    pub fn refresh(&mut self) {
        let mut rng = rand::rngs::OsRng;
        self.loaded_keys = iter::repeat_with(|| {
            let key: SigningKey<Secp256k1> = PlcBlessedSigningKey::new_random(&mut rng);
            key.into()
        })
        .take(3)
        .collect();
    }
}
