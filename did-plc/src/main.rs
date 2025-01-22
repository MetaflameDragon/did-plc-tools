use base64::prelude::*;
use did_key::{self, DidKey};
use did_plc::aka_uri::AkaUri;
use did_plc::{DidPlc, PlcService, SignedPlcOperation, UnsignedPlcOperation};
use ecdsa::signature::Signer;
use ecdsa::SigningKey;
use k256::Secp256k1;
use serde::{Serialize, Serializer};
use sha2::Digest;
use std::{collections::HashMap, fmt::Pointer, io::Write, iter};
use url::Url;

mod helpers;

fn main() {
    let handle = "scalytooth.metaflame.dev";
    let endpoint = Url::parse("https://scalytooth.metaflame.dev").expect("invalid endpoint URL");
    let aka_uri = AkaUri::new_at(handle).expect("Invalid URI");

    let mut rng = rand::rngs::OsRng;

    let signing_key = SigningKey::<Secp256k1>::random(&mut rng);

    let rotation_keys: Vec<_> = iter::from_fn(|| Some(SigningKey::<Secp256k1>::random(&mut rng)))
        .take(2)
        .collect();

    let unsigned_op = UnsignedPlcOperation::new_genesis(
        rotation_keys
            .iter()
            .map(|key| elliptic_curve::PublicKey::from(key.verifying_key()))
            .map(|pub_key| pub_key.into())
            .collect(),
        HashMap::from([(
            "atproto".to_string(),
            elliptic_curve::PublicKey::from(signing_key.verifying_key()).into(),
        )]),
        vec![aka_uri],
        HashMap::from([(
            "atproto_pds".to_string(),
            PlcService::new_atproto_pds(endpoint),
        )]),
    ).unwrap();

    dbg!(&unsigned_op);

    // Serialization test - JSON should look the same as in the docs/examples
    //println!("{}", serde_json::ser::to_string_pretty(&unsigned_op).unwrap());

    let operation_signing_key = rotation_keys
        .first()
        .expect("Expected at least one rotation key");
    let signed_op = SignedPlcOperation::new(unsigned_op, operation_signing_key);

    println!("{}", serde_json::ser::to_string_pretty(&signed_op).unwrap());

    let did_plc = DidPlc::from_signed_op(&signed_op);
    println!("{did_plc}");

    // // Store generated keys
    // let file_name = format!(
    //     "./did_plc_creds_{}.secret",
    //     chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S")
    // );
    // helpers::write_results(
    //     &signing_key,
    //     &rotation_keys,
    //     &signed_op,
    //     did_plc.plc_hash(),
    //     &file_name,
    // );
    // println!("Saved credentials to {file_name}");
}
