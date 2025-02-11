use std::collections::HashMap;

use did_key::DidKey;
use ecdsa::signature::Signer;
use ecdsa::{Signature, SignatureEncoding};
use elliptic_curve::{CurveArithmetic, PrimeCurve};
use serde::{Deserialize, Serialize};

use crate::aka_uri::AkaUri;
use crate::operation::signed::SignedPlcOperation;
use crate::plc_operation_ref::PlcOperationRef;
use crate::plc_service::PlcService;
use crate::PlcBlessedKeyCurve;

/// Represents an unsigned PLC operation (all fields except for `sig`).
///
/// Field order matters for `serde_json`, and matches the order
/// used by [plc.directory](https://plc.directory).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnsignedPlcOperation {
    // CID Hash reference to previous operation, null (None) for genesis operations
    prev: Option<PlcOperationRef>,

    // Fixed value "plc_operation"
    r#type: String,

    // Key-value map of services, services must have a type and endpoint.
    // Endpoint must be a valid http(s)-prefixed url
    // Key is currently just "atproto_pds" for type "AtprotoPersonalDataServer"
    // TODO validate max count
    // TODO validate ID length
    // TODO validate service type length
    // TODO validate endpoint length
    // https://github.com/did-method-plc/did-method-plc/blob/main/packages/server/src/constraints.ts
    services: HashMap<String, PlcService>,

    // Array of at:// handles
    // TODO prevent duplicate entries
    #[serde(rename = "alsoKnownAs")]
    also_known_as: Vec<AkaUri>,

    // Array of up to 5 rotation keys
    #[serde(rename = "rotationKeys")]
    rotation_keys: Vec<DidKey>,

    // Key-value map of verification methods (e.g. "atproto" & signing key)
    // TODO validate ID length
    #[serde(rename = "verificationMethods")]
    verification_methods: HashMap<String, DidKey>,
}

impl UnsignedPlcOperation {
    pub fn new_genesis(
        rotation_keys: Vec<DidKey>,
        verification_methods: HashMap<String, DidKey>,
        also_known_as: Vec<AkaUri>,
        services: HashMap<String, PlcService>,
    ) -> Result<Self, !> {
        Self::new(
            rotation_keys,
            verification_methods,
            also_known_as,
            services,
            None,
        )
    }

    pub fn new(
        rotation_keys: Vec<DidKey>,
        verification_methods: HashMap<String, DidKey>,
        also_known_as: Vec<AkaUri>,
        services: HashMap<String, PlcService>,
        prev: Option<PlcOperationRef>,
    ) -> Result<Self, !> {
        Ok(UnsignedPlcOperation {
            r#type: "plc_operation".to_string(),
            rotation_keys,
            verification_methods,
            also_known_as,
            services,
            prev,
        })
    }

    pub fn sign<S, C>(self, signing_key: &S) -> SignedPlcOperation
    where
        C: PlcBlessedKeyCurve,
        C: PrimeCurve + CurveArithmetic,
        S: Signer<Signature<C>>,
        Signature<C>: SignatureEncoding,
    {
        SignedPlcOperation::new(self, signing_key)
    }

    pub fn r#type(&self) -> &str {
        &self.r#type
    }
    pub fn rotation_keys(&self) -> &[DidKey] {
        &self.rotation_keys
    }
    pub fn verification_methods(&self) -> &HashMap<String, DidKey> {
        &self.verification_methods
    }
    pub fn also_known_as(&self) -> &[AkaUri] {
        &self.also_known_as
    }
    pub fn services(&self) -> &HashMap<String, PlcService> {
        &self.services
    }

    pub fn prev(&self) -> Option<PlcOperationRef> {
        self.prev // Copiable
    }

    pub fn is_genesis(&self) -> bool {
        self.prev.is_none()
    }
}

#[cfg(test)]
mod tests {
    #[test_log::test]
    fn test() {}
}
