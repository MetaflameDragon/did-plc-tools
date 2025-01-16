use crate::app::AppSection;
use derive_more::{Deref, DerefMut, Into};
use egui::{RichText, Ui};
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Clone, Debug, Deref, DerefMut, Into)]
pub struct HashMapRenderer<K, V> {
    #[deref]
    #[deref_mut]
    #[into]
    map: HashMap<K, V>,
}

impl<K, V> Default for HashMapRenderer<K, V> {
    fn default() -> Self {
        Self {
            map: Default::default(),
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

impl<K, V> AppSection for HashMapRenderer<K, V>
where
    K: ToString + Clone + Eq + Hash,
    V: AppSection,
{
    fn draw_and_update(&mut self, ui: &mut Ui) {
        Self::draw_map_items(&mut self.map, ui);
    }
}

impl<K, V> HashMapRenderer<K, V>
where
    K: ToString + Clone + Eq + Hash,
    V: AppSection,
{
    fn draw_map_items(map: &mut HashMap<K, V>, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                if map.is_empty() {
                    ui.label(RichText::new("Empty").weak().italics());
                } else {
                    let mut key_to_remove = None;

                    for (key, mut value) in &mut *map {
                        let should_remove = Self::draw_item(&key.to_string(), &mut value, ui);
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
    fn draw_item(key: &str, value: &mut V, ui: &mut Ui) -> bool {
        let mut should_remove = false;
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let resp = ui.button("X");
                should_remove = resp.clicked();
                ui.label(&*key);
            });
            value.draw_and_update(ui);
        });

        should_remove
    }
}
