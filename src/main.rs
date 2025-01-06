use base64::prelude::*;
use did_key::{self, CoreSign, KeyMaterial, Secp256k1KeyPair};
use did_plc::{AkaUri, DidPlc, PlcService, SignedPlcOperation, UnsignedPlcOperation};
use serde::{Serialize, Serializer};
use sha2::Digest;
use std::{collections::HashMap, fmt::Pointer, io::Write, iter};
use url::Url;

mod helpers;

fn main() {
    let handle = "scalytooth.metaflame.dev";
    let endpoint = Url::parse("https://scalytooth.metaflame.dev").expect("invalid endpoint URL");

    let seed = None;
    let signing_key = did_key::generate::<Secp256k1KeyPair>(seed);
    let rotation_keys: Vec<_> = iter::from_fn(|| Some(did_key::generate::<Secp256k1KeyPair>(seed)))
        .take(2)
        .collect();

    let unsigned_op = UnsignedPlcOperation::new_genesis(
        rotation_keys.iter().map(did_plc::format_did_key).collect(),
        HashMap::from([("atproto".to_string(), did_plc::format_did_key(&signing_key))]),
        vec![AkaUri::new_at(handle)],
        HashMap::from([("atproto_pds".to_string(), PlcService::new_at_pds(&endpoint))]),
    );

    dbg!(&unsigned_op);

    // Serialization test - JSON should look the same as in the docs/examples
    //println!("{}", serde_json::ser::to_string_pretty(&unsigned_op).unwrap());

    let signing_key = rotation_keys
        .first()
        .expect("Expected at least one rotation key");
    let signed_op = SignedPlcOperation::new(unsigned_op, signing_key);

    // println!("{}", serde_json::ser::to_string_pretty(&signed_op).unwrap());

    let did_plc = DidPlc::from_signed_op(&signed_op);
    println!("{did_plc}");

    // Store generated keys
    let file_name = format!(
        "./did_plc_creds_{}.secret",
        chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S")
    );
    helpers::write_results(
        &signing_key,
        &rotation_keys,
        &signed_op,
        did_plc.plc_hash(),
        file_name.as_str(),
    );
    println!("Saved credentials to {file_name}");
}
