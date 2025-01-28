use anyhow::{Context, Result};
use did_plc::AkaUri;
use egui::Ui;
use itertools::Itertools;

#[derive(Debug, Default, Clone)]
pub struct AlsoKnownAsInterface {
    entries_multiline: String,
}

impl AlsoKnownAsInterface {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.text_edit_multiline(&mut self.entries_multiline);
    }

    pub fn get_aka_uris(&self) -> Result<Vec<AkaUri>> {
        self.entries_multiline
            .lines()
            .map(|line| TryInto::<AkaUri>::try_into(line).context("Failed to parse line as AkaUri"))
            .collect()
    }

    pub fn from_aka_uris(aka_uris: impl IntoIterator<Item = AkaUri>) -> Self {
        AlsoKnownAsInterface {
            entries_multiline: aka_uris.into_iter().map(String::from).join("\n").to_owned(),
        }
    }
}
