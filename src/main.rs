use did_key::{
    self, Config, DIDCore, Fingerprint, KeyMaterial, P256KeyPair, PatchedKeyPair, Secp256k1KeyPair,
    CONFIG_LD_PUBLIC,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Pointer, iter};
use url::Url;

fn main() {
    let handle = "scalytooth.metaflame.dev";
    let endpoint = Url::parse("https://scalytooth.metaflame.dev").expect("invalid endpoint URL");

    let seed = None;
    let signing_key = did_key::generate::<Secp256k1KeyPair>(seed);
    let rotation_keys = iter::from_fn(|| Some(did_key::generate::<Secp256k1KeyPair>(seed)))
        .take(2)
        .collect();

    let unsigned_op = UnsignedPlcOperation {
        r#type: "plc_operation".to_string(),
        rotation_keys: rotation_keys.iter().map(format_did_key).collect(),
        verification_methods: HashMap::from([(
            "atproto".to_string(),
            format_did_key(&signing_key),
        )]),
        also_known_as: vec![AkaUri(format!("at://{handle}").to_string())],
        services: HashMap::from([(
            "atproto_pds".to_string(),
            PlcService {
                r#type: "AtprotoPersonalDataServer".to_string(),
                endpoint: endpoint.to_string(),
            },
        )]),
        prev: None,
    };

    dbg!(&unsigned_op);
}

fn format_did_key(did_key: &PatchedKeyPair) -> DidKey {
    DidKey(format!("did:key:{}", did_key.fingerprint()))
}
#[derive(Debug, Serialize, Deserialize)]
struct DidKey(String);

#[derive(Debug, Serialize, Deserialize)]
struct Signature(String);

#[derive(Debug, Serialize, Deserialize)]
struct AkaUri(String);

#[derive(Debug, Serialize, Deserialize)]
struct PlcService {
    r#type: String,
    endpoint: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PlcOperationRef(String);

#[derive(Debug, Serialize, Deserialize)]
struct PlcOperation {
    inner: UnsignedPlcOperation,
    sig: Signature,
}

#[derive(Debug, Serialize, Deserialize)]
struct UnsignedPlcOperation {
    // Fixed value "plc_operation"
    r#type: String,
    // Array of up to 5 rotation keys
    rotation_keys: Vec<DidKey>,
    // Key-value map of verification methods (e.g. "atproto" & signing key)
    verification_methods: HashMap<String, DidKey>,
    // Array of at:// handles
    also_known_as: Vec<AkaUri>,
    // Key-value map of services, services must have a type and endpoint.
    // Endpoint must be a valid http(s)-prefixed url
    // Key is currently just "atproto_pds" for type "AtprotoPersonalDataServer"
    services: HashMap<String, PlcService>,
    // CID Hash reference to previous operation, null (None) for genesis operations
    prev: Option<PlcOperationRef>,
}
