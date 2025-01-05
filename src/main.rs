use base64::prelude::*;
use did_key::{self, CoreSign, Fingerprint, PatchedKeyPair, Secp256k1KeyPair};
use serde::{Deserialize, Serialize, Serializer};
use sha2::Digest;
use std::{collections::HashMap, fmt::Pointer, iter};
use url::Url;

fn main() {
    let handle = "scalytooth.metaflame.dev";
    let endpoint = Url::parse("https://scalytooth.metaflame.dev").expect("invalid endpoint URL");

    let seed = None;
    let signing_key = did_key::generate::<Secp256k1KeyPair>(seed);
    let rotation_keys: Vec<_> = iter::from_fn(|| Some(did_key::generate::<Secp256k1KeyPair>(seed)))
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

    // Serialization test - JSON should look the same as in the docs/examples
    //println!("{}", serde_json::ser::to_string_pretty(&unsigned_op).unwrap());

    let unsigned_op_serialized = serde_ipld_dagcbor::ser::to_vec(&unsigned_op)
        .expect("Unsigned operation serialization failed");
    let signature = rotation_keys
        .first()
        .expect("Expected at least one rotation key")
        .sign(unsigned_op_serialized.as_slice());
    let signature_base64url = BASE64_URL_SAFE.encode(signature.as_slice());

    let signed_op = SignedPlcOperation {
        inner: unsigned_op,
        sig: Signature(signature_base64url),
    };

    let signed_op_serialized =
        serde_ipld_dagcbor::ser::to_vec(&signed_op).expect("Signed operation serialization failed");

    let signed_op_hash = sha2::Sha256::digest(&signed_op_serialized);

    println!("{}", serde_json::ser::to_string_pretty(&signed_op).unwrap());

    let plc_hash = &base32::encode(
        base32::Alphabet::Rfc4648Lower { padding: false },
        signed_op_hash.as_slice(),
    )[..24];

    println!("did:plc:{}", plc_hash)
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
struct SignedPlcOperation {
    #[serde(flatten)]
    inner: UnsignedPlcOperation,
    sig: Signature,
}

#[derive(Debug, Serialize, Deserialize)]
struct UnsignedPlcOperation {
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
