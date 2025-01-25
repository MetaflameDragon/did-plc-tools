use crate::app::AppSection;
use anyhow::Context;
use anyhow::Result;
use did_plc::AkaUri;
use egui::Ui;

#[derive(Debug, Default, Clone)]
pub struct AlsoKnownAsInterface {
    entries_multiline: String,
}

impl AppSection for AlsoKnownAsInterface {
    fn draw_and_update(&mut self, _ctx: &egui::Context, ui: &mut Ui) {
        ui.text_edit_multiline(&mut self.entries_multiline);
    }
}

impl AlsoKnownAsInterface {
    pub fn get_aka_uris(&self) -> Result<Vec<AkaUri>> {
        self.entries_multiline
            .lines()
            .map(|line| TryInto::<AkaUri>::try_into(line).context("Failed to parse line as AkaUri"))
            .collect()
    }

    pub fn set_aka_uris(&mut self, aka_uris: &[AkaUri]) -> Result<()> {
        // TODO
        // self.entries_multiline = aka_uris.iter().map(|uri| uri.into()).join("\n");

        Ok(())
    }
}
