use crate::ui_helpers::emoji;
use crate::{app::AppSection, signing_key::key::SigningKey};
use derive_more::{Deref, DerefMut};
use egui::Ui;
use log::{error, info};
use std::path::PathBuf;

#[repr(transparent)]
#[derive(Clone, Default, Deref, DerefMut, Debug)]
pub struct SigningKeyContainer {
    #[deref]
    #[deref_mut]
    key: Option<SigningKey>,
}

impl AppSection for SigningKeyContainer {
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
                    self.key = SigningKey::generate_keypair().ok();
                }
                if ui.button("Load").clicked() {
                    self.is_load_modal_open = true;
                }
            }
        });
    }
}

enum DrawKeyResponse {
    DeleteKey,
    SaveKey,
}

fn draw_contained_key(key: &mut SigningKey, ctx: &egui::Context, ui: &mut Ui) -> Option<DrawKeyResponse> {
    if ui.button("X").clicked() {
        return Some(DrawKeyResponse::DeleteKey);
    }

    if ui.small_button(emoji::FLOPPY_DISK).clicked() {
        return Some(DrawKeyResponse::SaveKey);
    }

    key.draw_and_update(ctx, ui);

    None
}
