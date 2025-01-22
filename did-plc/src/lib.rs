#![feature(never_type)]
use ecdsa::{Signature, SignatureEncoding, SigningKey};
#[macro_use]
extern crate derive_getters;

use aka_uri::AkaUri;
use base64::prelude::*;
use crypto_traits::MulticodecPrefix;
use derive_more::Into;
use did_key::DidKey;
use ecdsa::signature::Signer;
use elliptic_curve::{CurveArithmetic, PrimeCurve, PublicKey, SecretKey};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::{collections::HashMap, fmt::Display};
use url::Url;

pub mod aka_uri;
pub mod handle;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureBase64Url(String);

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

// TODO: Use CID
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlcOperationRef(pub String);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignedPlcOperation {
    #[serde(flatten)]
    inner: UnsignedPlcOperation,
    sig: SignatureBase64Url,
}

pub trait PlcBlessedKeyType {}

impl PlcBlessedKeyType for p256::NistP256 {}
impl PlcBlessedKeyType for k256::Secp256k1 {}

impl SignedPlcOperation {
    pub fn new<S, C>(unsigned_op: UnsignedPlcOperation, signing_key: &S) -> Self
    where
        // Curve C must be "blessed" (allowed by spec), and Signing key S must sign with curve C
        C: PlcBlessedKeyType,
        C: PrimeCurve + CurveArithmetic,
        S: Signer<Signature<C>>,
        Signature<C>: SignatureEncoding,
    {
        let unsigned_op_serialized = serde_ipld_dagcbor::ser::to_vec(&unsigned_op)
            .expect("Unsigned operation serialization failed");

        let signature: Signature<_> = Signer::sign(signing_key, &unsigned_op_serialized);
        let signature_base64url = BASE64_URL_SAFE.encode(signature.to_bytes().as_ref());

        SignedPlcOperation {
            inner: unsigned_op,
            sig: SignatureBase64Url(signature_base64url),
        }
    }

    pub fn get_did_plc(&self) -> DidPlc {
        // TODO: Limit to genesis op?
        DidPlc::from_signed_op(self)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
        C: PlcBlessedKeyType,
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
