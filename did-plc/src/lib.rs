#![feature(never_type)]
#[macro_use]
extern crate derive_getters;

use base64::prelude::*;
use derive_more::Into;
use did_key::DidKey;
use secp256k1::Message;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::{collections::HashMap, fmt::Display};
use url::Url;
use aka_uri::AkaUri;

mod aka_uri;
mod handle;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlcService {
    pub r#type: String,
    pub endpoint: Url,
}

impl PlcService {
    pub fn new_atproto_pds(pds_endpoint: Url) -> Self {
        PlcService {
            r#type: "AtprotoPersonalDataServer".to_string(),
            endpoint: pds_endpoint,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlcOperationRef(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedPlcOperation {
    #[serde(flatten)]
    inner: UnsignedPlcOperation,
    sig: Signature,
}

impl SignedPlcOperation {
    pub fn new(unsigned_op: UnsignedPlcOperation, signing_key: &secp256k1::SecretKey) -> Self {
        let unsigned_op_serialized = serde_ipld_dagcbor::ser::to_vec(&unsigned_op)
            .expect("Unsigned operation serialization failed");
        let message =
            Message::from_digest(sha2::Sha256::digest(unsigned_op_serialized.as_slice()).0);
        let signature = signing_key.sign_ecdsa(message);
        let signature_base64url = BASE64_URL_SAFE.encode(signature.serialize_compact());

        SignedPlcOperation {
            inner: unsigned_op,
            sig: Signature(signature_base64url),
        }
    }

    pub fn get_did_plc(&self) -> DidPlc {
        // TODO: Limit to genesis op?
        DidPlc::from_signed_op(self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnsignedPlcOperation {
    // Fixed value "plc_operation"
    r#type: String,

    // Array of up to 5 rotation keys
    #[serde(rename = "rotationKeys")]
    rotation_keys: Vec<DidKey>,

    // Key-value map of verification methods (e.g. "atproto" & signing key)
    #[serde(rename = "verificationMethods")]
    verification_methods: HashMap<String, DidKey>,

    // Array of at:// handles
    #[serde(rename = "alsoKnownAs")]
    also_known_as: Vec<AkaUri>,

    // Key-value map of services, services must have a type and endpoint.
    // Endpoint must be a valid http(s)-prefixed url
    // Key is currently just "atproto_pds" for type "AtprotoPersonalDataServer"
    services: HashMap<String, PlcService>,

    // CID Hash reference to previous operation, null (None) for genesis operations
    prev: Option<PlcOperationRef>,
}

impl UnsignedPlcOperation {
    pub fn new_genesis(
        rotation_keys: Vec<DidKey>,
        verification_methods: HashMap<String, DidKey>,
        also_known_as: Vec<AkaUri>,
        services: HashMap<String, PlcService>,
    ) -> Result<Self, !> {
        Ok(UnsignedPlcOperation {
            r#type: "plc_operation".to_string(),
            rotation_keys,
            verification_methods,
            also_known_as,
            services,
            prev: None,
        })
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

    pub fn sign(self, signing_key: &secp256k1::SecretKey) -> SignedPlcOperation {
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

    pub fn prev(&self) -> Option<&PlcOperationRef> {
        self.prev.as_ref()
    }
}

pub struct DidPlc {
    plc_hash: String,
}

impl DidPlc {
    pub fn new(plc_hash: String) -> Self {
        DidPlc { plc_hash }
    }

    pub fn from_signed_op(signed_op: &SignedPlcOperation) -> Self {
        let signed_op_serialized = serde_ipld_dagcbor::ser::to_vec(signed_op)
            .expect("Signed operation serialization failed");

        let signed_op_hash = sha2::Sha256::digest(&signed_op_serialized);

        let plc_hash = &base32::encode(
            base32::Alphabet::Rfc4648Lower { padding: false },
            signed_op_hash.as_slice(),
        )[..24];

        Self::new(plc_hash.to_string())
    }

    pub fn plc_hash(&self) -> &str {
        &self.plc_hash
    }

    pub fn to_did_str(&self) -> String {
        format!("did:plc:{}", self.plc_hash)
    }
}

impl Display for DidPlc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_did_str())
    }
}
