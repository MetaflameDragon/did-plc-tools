use std::collections::HashMap;
use std::ops::Deref;

use anyhow::{anyhow, bail, Context, Result};
use did_plc::{PlcService, UnsignedPlcOperation};
use egui::{RichText, Ui, ViewportCommand};
use log::{error, info};
use url::Url;

use crate::app::AppSection;
use crate::plc_builder::aka::AlsoKnownAsInterface;
use crate::plc_builder::rotation_keys::RotationKeysInterface;
use crate::plc_builder::services::ServicesInterface;
use crate::plc_builder::verification_methods::VerificationMethodsInterface;
use crate::signing_key::CryptoKeyContainer;

mod aka;
mod rotation_keys;
mod services;
mod verification_methods;

#[derive(Default, Clone, Debug)]
pub struct PlcBuilderInterface {
    also_known_as: AlsoKnownAsInterface,
    rotation_keys: RotationKeysInterface,
    verification_methods: VerificationMethodsInterface,
    services: ServicesInterface,
    prev: String,

    plc_json_loader: PlcJsonLoader,
    signing_key_selector: SigningKeySelector,
}

impl AppSection for PlcBuilderInterface {
    fn draw_and_update(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        ui.vertical(|ui| {
            self.draw_plc_loader_interface(ctx, ui);

            ui.heading("Also known as:");
            self.also_known_as.draw_and_update(ctx, ui);

            ui.heading("Rotation keys:");
            self.rotation_keys.draw_and_update(ctx, ui);

            ui.heading("Verification methods:");
            self.verification_methods.draw_and_update(ctx, ui);

            ui.heading("Services:");
            self.services.draw_and_update(ctx, ui);

            ui.heading("Previous CID:");
            ui.text_edit_singleline(&mut self.prev);

            ui.group(|ui| self.draw_action_column(ctx, ui));
        });
    }
}

impl PlcBuilderInterface {
    fn draw_plc_loader_interface(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        let plc_op = match self.plc_json_loader.ui(ctx, ui) {
            None => None,
            Some(Ok(plc_op)) => {
                info!("Loading plc json: {:?}", plc_op);
                Some(plc_op)
            }
            Some(Err(e)) => {
                error!("Error loading plc json:");
                for err in e.chain().take(3) {
                    error!(": {}", err);
                }
                None
            }
        };

        if let Some(plc_op) = plc_op {
            match Self::from_plc_op(plc_op) {
                Ok(new) => {
                    *self = new;
                }
                Err(err) => {
                    error!("Failed to load PLC operation: {err}");
                }
            }
        }
    }

    pub fn with_atproto_pds(mut self, pds_endpoint: String) -> Self {
        self.services.add_atproto_pds(pds_endpoint);
        self
    }

    fn get_unsigned_plc_op(&self) -> Result<UnsignedPlcOperation> {
        Ok(UnsignedPlcOperation::new(
            // self.rotation_keys
            //     .keys()
            //     .iter()
            //     .filter_map(|k| k.as_ref().map(|k| k.as_did_key()))
            //     .collect(),
            todo!(),
            // self.verification_methods
            //     .get_map()
            //     .iter()
            //     .map(|(key, value)| (key.clone(), value.as_did_key()))
            //     .collect(),
            todo!(),
            self.also_known_as
                .get_aka_uris()
                .context("Failed to parse AkaUris")?,
            HashMap::from_iter(
                self.services
                    .get_map()
                    .iter()
                    .map(|(key, value)| {
                        value
                            .clone()
                            .try_into()
                            .map(|plc_service: PlcService| (key.clone(), plc_service))
                    })
                    .collect::<Result<Vec<_>>>()?,
            ),
            if !self.prev.is_empty() {
                Some(
                    self.prev
                        .clone()
                        .try_into()
                        .context("Failed to parse prev CID")?,
                )
            } else {
                None
            },
        )?)
    }

    fn draw_action_column(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        if ui.button("Print unsigned PLC Operation JSON").clicked() {
            let plc_op = self.get_unsigned_plc_op();
            match plc_op {
                Ok(plc_op) => {
                    let json = serde_json::ser::to_string_pretty(&plc_op)
                        .unwrap_or("Failed to serialize plc operation".to_string());
                    info!("{json}");
                }
                Err(err) => {
                    error!("{err}")
                }
            }
        }

        // TODO
        // self.signing_key_selector
        //     .ui(ctx, ui, self.rotation_keys.keys().deref());
        //
        // if ui.button("Sign operation").clicked() {
        //     let plc_op = self.get_unsigned_plc_op();
        //     let signing_key = self
        //         .rotation_keys
        //         .keys()
        //         .get(self.signing_key_selector.key_index)
        //         .and_then(|k| k.as_ref());
        //
        //     let signed_op: Result<(), _> = match plc_op {
        //         Ok(plc_op) => match signing_key {
        //             None => Err(anyhow!("No signing key selected")),
        //             Some(signing_key) => {
        //                 todo!()
        //                 // match signing_key.keypair() {
        //                 //     None => Err(anyhow!("Signing key is not owned (no private part)")),
        //                 //     Some(keypair) => Ok(plc_op.sign(&keypair.secret_key())),
        //                 // }
        //             }
        //         },
        //         Err(err) => Err(err),
        //     };
        //
        //     match signed_op {
        //         Ok(signed_op) => {
        //             todo!()
        //             // let did_plc = signed_op.get_did_plc();
        //             // let json = serde_json::ser::to_string_pretty(&signed_op)
        //             //     .unwrap_or("Failed to serialize signed plc operation".to_string());
        //             // info!("{json}");
        //             // info!("{did_plc}");
        //         }
        //         Err(err) => {
        //             error!("{err}");
        //         }
        //     }
        // }
    }

    fn from_plc_op(plc_op: UnsignedPlcOperation) -> Result<Self> {
        let also_known_as =
            AlsoKnownAsInterface::from_aka_uris(plc_op.also_known_as().iter().cloned());

        let rotation_keys = RotationKeysInterface::from_keys(plc_op.rotation_keys().to_owned());

        let verification_methods =
            VerificationMethodsInterface::from_map(plc_op.verification_methods().clone());

        let services = ServicesInterface::from_map(plc_op.services().clone());

        let prev = plc_op
            .prev()
            .map(|plc_op_ref| plc_op_ref.to_string())
            .unwrap_or(String::new());

        Ok(Self {
            also_known_as,
            rotation_keys,
            verification_methods,
            services,
            prev,
            ..Default::default()
        })
    }
}

#[derive(Default, Clone, Debug)]
struct SigningKeySelector {
    pub key_index: usize,
}

impl SigningKeySelector {
    fn ui(&mut self, _ctx: &egui::Context, ui: &mut Ui, rotation_keys: &[CryptoKeyContainer]) {
        let selected_key = rotation_keys.get(self.key_index);

        let empty_key_text = RichText::new("[empty]").weak().italics();

        // TODO
        // let selected_text = {
        //     match selected_key.map(|k| k.try_get_did_key()).flatten() {
        //         None => empty_key_text.clone(),
        //         Some(k) => {
        //             let val = k.multibase_value();
        //             let lead_char_count = 6;
        //             let tail_char_count = 3;
        //             // Show only a truncated version of the value
        //             RichText::new(format!(
        //                 "{}...{}",
        //                 val[..lead_char_count].to_string(),
        //                 val[val.len() - tail_char_count..].to_string()
        //             ))
        //         }
        //     }
        // };
        //
        // egui::ComboBox::from_label("Signing key")
        //     .selected_text(selected_text)
        //     .show_ui(ui, |ui| {
        //         for (i, key_container) in rotation_keys.iter().enumerate() {
        //             let label = key_container
        //                 .try_get_did_key()
        //                 .map_or(empty_key_text.clone(), |k| {
        //                     RichText::new(k.multibase_value())
        //                 });
        //             ui.selectable_value(&mut self.key_index, i, label);
        //         }
        //     });
    }
}

#[derive(Default, Clone, Debug)]
struct PlcJsonLoader {}

impl PlcJsonLoader {
    /// Displays the UI
    ///
    /// Returns `Some(Result)` when the user attempts to parse a PLC operation.
    /// - `Result::Ok(UnsignedPlcOperation)` if parsing was successful.
    /// - `Result::Err` with an `anyhow` error if there was an error while parsing.
    fn ui(&mut self, ctx: &egui::Context, ui: &mut Ui) -> Option<Result<UnsignedPlcOperation>> {
        if ui.button("Load from clipboard (JSON)").clicked() {
            ui.ctx().send_viewport_cmd(ViewportCommand::RequestPaste);
            ui.response().request_focus();
        }

        let clipboard = ui.input(|i| {
            i.events.iter().find_map(|e| {
                if let egui::Event::Paste(paste) = e {
                    Some(paste.to_owned())
                } else {
                    None
                }
            })
        })?;

        if clipboard.is_empty() {
            return Some(Err(anyhow!("Clipboard is empty")));
        }

        Some(
            serde_json::de::from_str(&clipboard).context("Failed to deserialize JSON in clipboard"),
        )
    }
}
