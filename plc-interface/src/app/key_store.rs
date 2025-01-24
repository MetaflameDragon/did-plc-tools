use derive_new::new;
use egui::Ui;
use std::path::PathBuf;

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
        ui.vertical(|ui| {
            for key in &self.store.loaded_keys {
                ui.label(key);
            }
        });
    }
}

#[derive(new)]
struct KeyStore {
    #[new(into)]
    key_store_path: PathBuf,
    #[new(default)]
    loaded_keys: Vec<String>,
}

impl KeyStore {
    pub fn set_dir(&mut self, dir_str: impl Into<PathBuf>) {
        self.key_store_path = dir_str.into();
    }

    pub fn refresh(&mut self) {
        self.loaded_keys = vec!["abc", "def", "ghi"]
            .iter()
            .map(|s| s.to_string())
            .collect();
    }
}
