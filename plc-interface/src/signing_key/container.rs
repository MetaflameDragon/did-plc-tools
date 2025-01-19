use crate::ui_helpers::emoji;
use crate::{app::AppSection, signing_key::key::CryptoKey};
use derive_more::{Deref, DerefMut};
use egui::{Color32, Modal, Ui, Widget};
use log::{error, info};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Default, Deref, DerefMut, Debug)]
pub struct CryptoKeyContainer {
    #[deref]
    #[deref_mut]
    key: Option<CryptoKey>,

    is_load_modal_open: bool,
    load_path_buf_str: String,
}

impl AppSection for CryptoKeyContainer {
    fn draw_and_update(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if let Some(ref mut key) = self.key {
                // Draw contained key
                match draw_contained_key(key, ctx, ui) {
                    None => {}
                    Some(DrawKeyResponse::DeleteKey) => {
                        self.key = None;
                    }
                    Some(DrawKeyResponse::SaveKey) => {
                        let did_key = key.as_did_key();
                        let did_key_name = did_key.formatted_value().replace(":", "_");
                        let path = format!("{did_key_name}.secp256k1.priv");
                        let path = PathBuf::from(path);

                        let result = key.save_keypair(&path);
                        match result {
                            Ok(()) => {
                                info!("Key saved to {}", path.display());
                            }
                            Err(err) => {
                                error!("{err}");
                            }
                        }
                    }
                }
            } else {
                // Draw key generation options
                if ui.button("New").clicked() {
                    self.key = CryptoKey::generate_keypair().ok();
                }
                if ui.button("Load").clicked() {
                    self.is_load_modal_open = true;
                }

                // Check for dropped file, set key if the cursor is hovering over this area
                let interact_rect = ui.response().interact_rect;
                if let Some(dropped_file) = ctx.input(|i| {
                    let Some(file) = i.raw.dropped_files.first() else {
                        return None;
                    };

                    let Some(pos) = i.pointer.latest_pos() else {
                        return None;
                    };
                    if interact_rect.contains(pos) {
                        Some(file.clone())
                    } else {
                        None
                    }
                }) {
                    if let Some(path) = &dropped_file.path {
                        match CryptoKey::load_keypair(path) {
                            Ok(key) => {
                                self.key = Some(key);
                            }
                            Err(err) => {
                                error!("{err}");
                            }
                        }
                    } else {
                        error!("File path was not set");
                    }
                }
            }
        });

        if self.is_load_modal_open {
            let modal = Modal::new(egui::Id::new("Key Load Modal")).show(ctx, |ui| {
                ui.set_width(f32::min(ui.available_width(), 250.0));
                ui.vertical_centered(|ui| {
                    ui.heading("Load from file:");
                    // NOTE: single-line TextEdit does not have the correct width despite clipping text
                    // https://github.com/emilk/egui/issues/5500
                    let text_field =
                        egui::TextEdit::singleline(&mut self.load_path_buf_str).clip_text(true);
                    let text_resp = text_field.ui(ui);

                    let confirm_button_resp = ui.button("Load");

                    let user_confirmed_field = text_resp.lost_focus()
                        && text_resp
                            .ctx
                            .input(|state| state.key_down(egui::Key::Enter));
                    if confirm_button_resp.clicked() || user_confirmed_field {
                        let path = Path::new(&self.load_path_buf_str);
                        match CryptoKey::load_keypair(path) {
                            Ok(key) => {
                                self.key = Some(key);
                                self.load_path_buf_str.clear();
                                self.is_load_modal_open = false;
                            }
                            Err(err) => {
                                error!("{err}");
                            }
                        }
                    }
                });
            });

            if modal.should_close() {
                self.load_path_buf_str.clear();
                self.is_load_modal_open = false;
            }
        }
    }
}

enum DrawKeyResponse {
    DeleteKey,
    SaveKey,
}

fn draw_contained_key(
    key: &mut CryptoKey,
    ctx: &egui::Context,
    ui: &mut Ui,
) -> Option<DrawKeyResponse> {
    if ui.button("X").clicked() {
        return Some(DrawKeyResponse::DeleteKey);
    }

    if ui.small_button(emoji::FLOPPY_DISK).clicked() {
        return Some(DrawKeyResponse::SaveKey);
    }

    key.draw_and_update(ctx, ui);

    None
}
