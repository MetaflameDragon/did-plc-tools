use std::fs;
use std::path::{Path, PathBuf};

use derive_more::Display;
use derive_new::new;
use did_key::DidKey;
use did_plc::{PlcBlessedSigningKey, PlcBlessedSigningKeyBox};
use ecdsa::SigningKey;
use egui::{CollapsingHeader, Color32, Modal, RichText, Ui};
use itertools::Itertools;
use k256::Secp256k1;
use log::{error, info};
use p256::NistP256;

pub struct KeyStoreInterface {
    key_store_dir_str: String,
    store: KeyStore,
    key_gen_interface: KeyGeneratorInterface,
}

impl KeyStoreInterface {
    pub fn keystore(&self) -> &KeyStore {
        &self.store
    }

    pub fn new(key_store_dir_str: String) -> Self {
        let mut store = KeyStore::new(key_store_dir_str.clone());
        store.refresh();

        Self {
            key_store_dir_str,
            store,
            key_gen_interface: KeyGeneratorInterface {
                ..Default::default()
            },
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let refresh_button = ui.small_button(crate::ui_helpers::emoji::COUNTERCLOCKWISE_ARROWS);
            if refresh_button.clicked() {
                self.store.set_dir(&self.key_store_dir_str);
                self.store.refresh();
            };
            ui.text_edit_singleline(&mut self.key_store_dir_str);
        });
        let keys_header = format!("Keys ({})", &self.store.loaded_keys.len());
        CollapsingHeader::new(keys_header)
            .id_salt(egui::Id::from("Keys collapsing header"))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    for key in &self.store.loaded_keys {
                        let formatted_value = key.as_did_key().formatted_value().to_owned();
                        let label = RichText::new(formatted_value);
                        ui.label(label.monospace());
                    }

                    if ui.button("Add Key").clicked() {
                        self.key_gen_interface.set_modal_open_state(true);
                    }

                    self.store.set_dir(&self.key_store_dir_str);

                    let new_key = self
                        .key_gen_interface
                        .ui(ui, Path::new(&self.key_store_dir_str));

                    if let Some(_new_key) = new_key {
                        // TODO: update without a full refresh
                        self.store.refresh();
                    }
                });
            });
    }
}

#[derive(new)]
pub struct KeyStore {
    #[new(into)]
    key_store_path: PathBuf,
    #[new(default)]
    loaded_keys: Vec<PlcBlessedSigningKeyBox>,
}

impl KeyStore {
    pub fn keys(&self) -> &[PlcBlessedSigningKeyBox] {
        &self.loaded_keys
    }

    pub fn try_get_by_did_key(&self, key: &DidKey) -> Option<&PlcBlessedSigningKeyBox> {
        self.loaded_keys
            .iter()
            .find(|key_box| key_box.as_did_key() == *key)
    }

    pub fn set_dir(&mut self, dir_str: impl Into<PathBuf>) {
        self.key_store_path = dir_str.into();
    }

    pub fn refresh(&mut self) {
        let dir_iter = match fs::read_dir(&self.key_store_path) {
            Ok(iterator) => {
                info!(
                    "Found key store path (at \"{}\")",
                    self.key_store_path.display()
                );
                iterator
            }
            Err(err) => {
                error!("Error refreshing keys: {err}");
                error!("Was looking for: \"{}\"", self.key_store_path.display());
                return;
            }
        };

        self.loaded_keys.clear();

        for file in dir_iter {
            let file = match file {
                Ok(file) => file,
                Err(err) => {
                    error!("Unexpected IO error when iterating dir: {}", err);
                    continue;
                }
            };

            let key = match PlcBlessedSigningKeyBox::read_from_file_pem(&file.path()) {
                Ok(key) => key,
                Err(err) => {
                    error!("Error reading file {}: {}", file.path().display(), err);
                    continue;
                }
            };
            self.loaded_keys.push(key);
        }
    }
}

#[derive(Default)]
struct KeyGeneratorInterface {
    modal_open: bool,
    selected_key_type_index: usize,
}

impl KeyGeneratorInterface {
    pub fn set_modal_open_state(&mut self, should_open: bool) {
        self.modal_open = should_open;
    }

    pub fn ui(&mut self, ui: &mut Ui, key_store_path: &Path) -> Option<PlcBlessedSigningKeyBox> {
        if !self.modal_open {
            return None;
        }

        let modal_response = Modal::new(egui::Id::new("Key Store Generator Interface"))
            .show(ui.ctx(), |ui| self.modal_ui(ui, key_store_path));

        if modal_response.should_close() {
            self.modal_open = false;
        }

        modal_response.inner
    }

    fn modal_ui(&mut self, ui: &mut Ui, key_store_path: &Path) -> Option<PlcBlessedSigningKeyBox> {
        #[derive(Display)]
        enum KeyType {
            #[display("Secp256k1")]
            Secp256k1,
            #[display("NistP256 (p256)")]
            NistP256,
        }

        let key_types = [KeyType::Secp256k1, KeyType::NistP256];

        egui::ComboBox::from_id_salt("Key type selector").show_index(
            ui,
            &mut self.selected_key_type_index,
            key_types.len(),
            |selected_key_type_index| key_types[selected_key_type_index].to_string(),
        );

        ui.horizontal(|ui| {
            ui.label(RichText::new("Will be saved to:").weak().italics());
            let canonical_path = key_store_path.canonicalize();
            let path_text = match canonical_path {
                Ok(path) => RichText::new(path.display().to_string()).weak().italics(),
                Err(_) => RichText::new(
                    format!("[Invalid path] {}", key_store_path.display()).to_string(),
                )
                .color(Color32::DARK_RED),
            };
            ui.label(path_text);
        });

        if ui.button("Save new key").clicked() {
            let mut rng = rand::rngs::OsRng;

            let key: PlcBlessedSigningKeyBox = match key_types[self.selected_key_type_index] {
                KeyType::Secp256k1 => SigningKey::<Secp256k1>::new_random(&mut rng).into(),
                KeyType::NistP256 => SigningKey::<NistP256>::new_random(&mut rng).into(),
            };

            match Self::save_key(&key, &key_store_path) {
                Ok(()) => {
                    self.modal_open = false;
                    return Some(key);
                }
                Err(err) => {
                    error!("Failed to save key: {err}")
                }
            }
        };

        None
    }

    fn save_key(key: &PlcBlessedSigningKeyBox, key_store_path: &Path) -> std::io::Result<()> {
        let key_path = key_store_path.join(key.as_did_key().multibase_value());
        info!("Saving key to {}", key_path.display());
        key.write_to_file(&key_path)
    }
}
