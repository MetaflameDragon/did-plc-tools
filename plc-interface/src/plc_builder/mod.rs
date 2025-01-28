use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use did_plc::{PlcOperationRef, PlcService, SignedPlcOperation, UnsignedPlcOperation};
use egui::{Ui, ViewportCommand};
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

    plc_json_loader: PlcJsonLoader,
}

impl PlcBuilderInterface {
    pub fn ui(&mut self, ui: &mut Ui, keystore: &KeyStore) {
        ui.vertical(|ui| {
            self.draw_plc_loader_interface(ui);

            ui.heading("Also known as:");
            self.also_known_as.ui(ui);

            ui.heading("Rotation keys:");
            self.rotation_keys.ui(ui, keystore);

            ui.heading("Verification methods:");
            self.verification_methods.ui(ui);

            ui.heading("Services:");
            self.services.ui(ui);

            ui.heading("Previous CID:");
            ui.label(
                self.prev
                    .map(|value| value.to_string())
                    .unwrap_or("".to_owned()),
            );

            ui.group(|ui| self.draw_action_column(ui));
        });
    }

    fn draw_plc_loader_interface(&mut self, ui: &mut Ui) {
        let plc_op = match self.plc_json_loader.ui(ui) {
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

    fn draw_action_column(&mut self, ui: &mut Ui) {
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

#[derive(Default, Clone, Debug)]
struct PlcJsonLoader {}

impl PlcJsonLoader {
    /// Displays the UI
    ///
    /// Returns `Some(Result)` when the user attempts to parse a PLC operation.
    /// - `Result::Ok(SignedPlcOperation)` if parsing was successful.
    /// - `Result::Err` with an `anyhow` error if there was an error while parsing.
    fn ui(&mut self, ui: &mut Ui) -> Option<Result<SignedPlcOperation>> {
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
