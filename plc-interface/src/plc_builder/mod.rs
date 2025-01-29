use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use did_key::DidKey;
use did_plc::{PlcOperationRef, PlcService, SignedPlcOperation, UnsignedPlcOperation};
use egui::{RichText, Ui, ViewportCommand, Widget};
use log::{error, info};

use crate::app::key_store::KeyStore;
use crate::plc_builder::aka::AlsoKnownAsInterface;
use crate::plc_builder::rotation_keys::RotationKeySetInterface;
use crate::plc_builder::services::ServicesInterface;
use crate::plc_builder::verification_methods::VerificationMethodsInterface;

mod aka;
mod rotation_keys;
mod services;
mod verification_methods;

#[derive(Default, Clone, Debug)]
pub struct PlcBuilderInterface {
    also_known_as: AlsoKnownAsInterface,
    rotation_keys: RotationKeySetInterface,
    verification_methods: VerificationMethodsInterface,
    services: ServicesInterface,
    prev: Option<PlcOperationRef>,
}

impl PlcBuilderInterface {
    pub fn ui(&mut self, ui: &mut Ui, key_store: &KeyStore) {
        ui.vertical(|ui| {
            let plc_op = self.draw_plc_loader_ui_print_errors(
                ui,
                "Load signed operation from clipboard (JSON) & set CID ref",
            );

            if let Some(signed_plc_op) = plc_op {
                match Self::from_signed_plc_op_with_ref(signed_plc_op) {
                    Ok(new) => {
                        *self = new;
                    }
                    Err(err) => {
                        error!("Failed to load PLC operation: {err}");
                    }
                }
            }

            ui.heading("Also known as:");
            self.also_known_as.ui(ui);

            ui.heading("Rotation keys:");
            self.rotation_keys.ui(ui, key_store);

            ui.heading("Verification methods:");
            self.verification_methods.ui(ui);

            ui.heading("Services:");
            self.services.ui(ui);

            ui.heading("Previous CID:");
            {
                let text = {
                    if let Some(value) = self.prev {
                        RichText::new(value.to_string())
                    } else {
                        RichText::new("[none]".to_string()).weak().italics()
                    }
                };
                ui.label(text);
            }
            ui.horizontal(|ui| {
                if ui.button("Clear").clicked() {
                    self.prev = None;
                }
                if let Some(plc_op) = self.draw_plc_loader_ui_print_errors(
                    ui,
                    "Load signed operation from clipboard (JSON) & set CID ref",
                ) {
                    match plc_op.get_cid_reference() {
                        Ok(prev) => {
                            self.prev = Some(prev);
                        }
                        Err(err) => {
                            error!("Failed to get CID to PLC operation: {err}");
                        }
                    };
                }
            });

            ui.add_space(20.0);

            ui.group(|ui| self.draw_action_column(ui, key_store));
        });
    }

    fn draw_plc_loader_ui_print_errors(
        &mut self,
        ui: &mut Ui,
        button_text: &str,
    ) -> Option<SignedPlcOperation> {
        match plc_json_loader_ui(ui, button_text) {
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
        }
    }

    pub fn with_atproto_pds(mut self, pds_endpoint: String) -> Self {
        self.services.add_atproto_pds(pds_endpoint);
        self
    }

    fn get_unsigned_plc_op(&self) -> Result<UnsignedPlcOperation> {
        Ok(UnsignedPlcOperation::new(
            self.rotation_keys.try_get_keys()?,
            self.verification_methods.get_map().clone(),
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
            self.prev,
        )?)
    }

    fn draw_action_column(&mut self, ui: &mut Ui, key_store: &KeyStore) {
        if ui.button("Print unsigned PLC Operation JSON").clicked() {
            let plc_op = self.get_unsigned_plc_op();
            match plc_op {
                Ok(plc_op) => {
                    let json = serde_json::ser::to_string_pretty(&plc_op)
                        .unwrap_or("Failed to serialize plc operation".to_string());
                    info!("{json}");
                }
                Err(err) => {
                    for err in err.chain().take(3) {
                        error!("{}", err);
                    }
                }
            }
        }

        if ui.button("Sign & print JSON").clicked() {
            let key = self.rotation_keys.try_get_selected_key();
            let Some(key) = key else {
                error!("No key selected");
                return;
            };
            if !self.rotation_keys.contains(key) {
                error!("Selected key is not in rotation keys");
                return;
            }
            let Some(signing_key) = key_store.try_get_by_did_key(key) else {
                error!("Selected key is not in key store");
                return;
            };
            let unsigned_op = match self.get_unsigned_plc_op() {
                Ok(plc_op) => plc_op,
                Err(err) => {
                    error!("Error getting PLC operation:");

                    for err in err.chain().take(3) {
                        error!("{}", err);
                    }
                    return;
                }
            };

            let signed_op = signing_key.sign_plc_op(unsigned_op);

            let result = match serde_json::ser::to_string_pretty(&signed_op) {
                Ok(res) => res,
                Err(err) => {
                    error!("Error serializing signed PLC operation: {err}");
                    return;
                }
            };

            info!("Signed PLC operation:\n{result}");
            info!("Identifier: {}", signed_op.get_did_plc());
            if signed_op.prev().is_some() {
                info!("(Note: this is not a genesis operation! You may need the genesis did:plc instead.)");
            }
        }
    }

    fn from_signed_plc_op_with_ref(plc_op: SignedPlcOperation) -> Result<Self> {
        let mut new = Self::from_unsigned_plc_op_direct((*plc_op).clone())?;
        new.prev = Some(PlcOperationRef::from_signed_op(&plc_op)?);
        Ok(new)
    }

    fn from_unsigned_plc_op_direct(plc_op: UnsignedPlcOperation) -> Result<Self> {
        let also_known_as =
            AlsoKnownAsInterface::from_aka_uris(plc_op.also_known_as().iter().cloned());

        let rotation_keys = RotationKeySetInterface::from_keys(plc_op.rotation_keys().to_owned());

        let verification_methods =
            VerificationMethodsInterface::from_map(plc_op.verification_methods().clone());

        let services = ServicesInterface::from_map(plc_op.services().clone());

        let prev = plc_op.prev();

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

/// Displays UI for a PLC JSON loader
///
/// Returns `Some(Result)` when the user attempts to parse a PLC operation.
/// - `Result::Ok(SignedPlcOperation)` if parsing was successful.
/// - `Result::Err` with an `anyhow` error if there was an error while parsing.
fn plc_json_loader_ui(ui: &mut Ui, button_text: &str) -> Option<Result<SignedPlcOperation>> {
    let button = egui::Button::new(button_text);
    let btn_resp = button.ui(ui);

    if btn_resp.clicked() {
        ui.ctx().send_viewport_cmd(ViewportCommand::RequestPaste);
        btn_resp.request_focus();
    }

    if !btn_resp.has_focus() {
        return None;
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

    btn_resp.surrender_focus();
    Some(serde_json::de::from_str(&clipboard).context("Failed to deserialize JSON in clipboard"))
}
