use base64::prelude::*;
use did_key::{self, DidKey};
use did_plc::{AkaUri, DidPlc, PlcService, SignedPlcOperation, UnsignedPlcOperation};
use serde::{Serialize, Serializer};
use sha2::Digest;
use std::{collections::HashMap, fmt::Pointer, io::Write, iter};
use url::Url;

mod helpers;

fn main() {
    let handle = "scalytooth.metaflame.dev";
    let endpoint = Url::parse("https://scalytooth.metaflame.dev").expect("invalid endpoint URL");

    let mut rng = secp256k1::rand::rngs::OsRng;

    let (signing_key_priv, signing_key_pub) = secp256k1::generate_keypair(&mut rng);
    let rotation_keys: Vec<_> = iter::from_fn(|| Some(secp256k1::generate_keypair(&mut rng)))
        .take(2)
        .collect();

    let unsigned_op = UnsignedPlcOperation::new_genesis(
        rotation_keys
            .iter()
            .map(|pair| pair.1.into())
            .collect(),
        HashMap::from([(
            "atproto".to_string(),
            DidKey::from_public_key(signing_key_pub),
        )]),
        vec![AkaUri::new_at(handle)],
        HashMap::from([("atproto_pds".to_string(), PlcService::new_at_pds(&endpoint))]),
    );

    dbg!(&unsigned_op);

    // Serialization test - JSON should look the same as in the docs/examples
    //println!("{}", serde_json::ser::to_string_pretty(&unsigned_op).unwrap());

    let operation_signing_key = rotation_keys
        .first()
        .expect("Expected at least one rotation key");
    let signed_op = SignedPlcOperation::new(unsigned_op, &operation_signing_key.0);

    // println!("{}", serde_json::ser::to_string_pretty(&signed_op).unwrap());

    let did_plc = DidPlc::from_signed_op(&signed_op);
    println!("{did_plc}");

    // Store generated keys
    let file_name = format!(
        "./did_plc_creds_{}.secret",
        chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S")
    );
    helpers::write_results(
        &signing_key_priv,
        &rotation_keys.iter().map(|pair| pair.0).collect::<Vec<_>>(),
        &signed_op,
        did_plc.plc_hash(),
        &file_name,
    );
    println!("Saved credentials to {file_name}");
}
