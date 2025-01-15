use crate::app::AppSection;
use did_plc::AkaUri;
use egui::Ui;

#[derive(Debug, Default, Clone)]
pub struct AlsoKnownAsInterface {
    entries_multiline: String,
}

impl AppSection for AlsoKnownAsInterface {
    fn draw_and_update(&mut self, ui: &mut Ui) {
        ui.text_edit_multiline(&mut self.entries_multiline);
    }
}

impl AlsoKnownAsInterface {
    pub fn get_aka_uris(&self) -> Vec<AkaUri> {
        todo!()
    }
}
