use did_key::{
    self, Config, DIDCore, Fingerprint, KeyMaterial, P256KeyPair, PatchedKeyPair, Secp256k1KeyPair,
    CONFIG_LD_PUBLIC,
};
use serde::Deserialize;
use std::{collections::HashMap, fmt::Pointer};
use url::Url;

fn main() {
    let handle = "scalytooth.metaflame.dev";
    let endpoint = Url::parse("https://scalytooth.metaflame.dev").expect("invalid endpoint URL");

    let seed = None;
    let signing_key = did_key::generate::<Secp256k1KeyPair>(seed);
    let rotation_keys = vec![did_key::generate::<Secp256k1KeyPair>(seed)];

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
                endpoint,
            },
        )]),
        prev: None,
    };

    dbg!(&unsigned_op);
}

fn format_did_key(did_key: &PatchedKeyPair) -> DidKey {
    DidKey(format!("did:key:{}", did_key.fingerprint()))
}
#[derive(Debug)]
struct DidKey(String);

#[derive(Debug)]
struct Signature(String);

#[derive(Debug)]
struct AkaUri(String);

#[derive(Debug)]
struct PlcService {
    r#type: String,
    endpoint: Url,
}

#[derive(Debug)]
struct PlcOperation<'a> {
    inner: &'a UnsignedPlcOperation<'a>,
    sig: Signature,
}

#[derive(Debug)]
struct UnsignedPlcOperation<'a> {
    r#type: String,
    rotation_keys: Vec<DidKey>,
    verification_methods: HashMap<String, DidKey>,
    also_known_as: Vec<AkaUri>,
    services: HashMap<String, PlcService>,
    prev: Option<&'a PlcOperation<'a>>,
}
