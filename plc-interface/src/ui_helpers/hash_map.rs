use std::collections::HashMap;
use std::hash::Hash;

use derive_more::{Deref, DerefMut, Into};
use did_key::DidKey;
use egui::{Button, RichText, Ui, Widget};

use crate::app::AppSection;

#[derive(Clone, Debug, Deref, DerefMut, Into)]
pub struct HashMapRenderer<K, V> {
    #[deref]
    #[deref_mut]
    #[into]
    map: HashMap<K, V>,
    pub allow_remove: bool,
}

impl<K, V> Default for HashMapRenderer<K, V> {
    fn default() -> Self {
        Self {
            map: Default::default(),
            allow_remove: true,
        }
    }
}

// #[derive(Default, Clone, Debug)]
// struct InputFields<V> {
//     name: String,
//     key: V,
// }

impl<K, V> HashMapRenderer<K, V> {
    pub fn inner(&self) -> &HashMap<K, V> {
        &self.map
    }
    pub fn inner_mut(&mut self) -> &mut HashMap<K, V> {
        &mut self.map
    }

    pub fn into_inner(self) -> HashMap<K, V> {
        self.map
    }
}

impl<K, V> HashMapRenderer<K, V>
where
    K: ToString + Clone + Eq + Hash,
    V: AppSection,
{
    pub fn ui(&mut self, ui: &mut Ui) {
        Self::draw_map_items(&mut self.map, self.allow_remove, ui);
    }

    fn draw_map_items(map: &mut HashMap<K, V>, allow_removing: bool, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                if map.is_empty() {
                    ui.label(RichText::new("Empty").weak().italics());
                } else {
                    let mut key_to_remove = None;

                    for (key, value) in &mut *map {
                        let should_remove =
                            Self::draw_item(&key.to_string(), value, allow_removing, ui);
                        if should_remove {
                            key_to_remove = Some(key.clone());
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
    fn draw_item(key: &str, value: &mut V, allow_removing: bool, ui: &mut Ui) -> bool {
        let mut should_remove = false;
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if allow_removing {
                    let remove_button = Button::new("X");
                    let resp = remove_button.ui(ui);
                    should_remove = resp.clicked();
                }
                ui.label(key);
            });
            value.draw_and_update(ui);
        });

        should_remove
    }
}

impl AppSection for DidKey {
    fn draw_and_update(&mut self, ui: &mut Ui) {
        // TODO: better proper viewer
        ui.monospace(self.formatted_value());
    }
}
