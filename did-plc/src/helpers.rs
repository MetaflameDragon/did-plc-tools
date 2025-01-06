use did_key::{KeyMaterial, PatchedKeyPair};
use did_plc::SignedPlcOperation;
use std::path::PathBuf;
use std::fs::File;
use serde::Serialize;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;

pub fn write_results(
    signing_key: &PatchedKeyPair,
    rotation_keys: &[PatchedKeyPair],
    plc_op: &SignedPlcOperation,
    plc_hash: &str,
    file_name: &str,
) {
    let file_path = PathBuf::from(file_name);

    let file = File::options()
        .write(true)
        .create_new(true)
        .open(file_path.clone());
    let file = match file {
        Ok(file) => file,
        Err(err) => panic!("Could not write results to {}: {err}", file_path.display()),
    };

    #[derive(Serialize)]
    struct Output<'a> {
        signing_key_priv_bytes_base64: String,
        rotation_keys_priv_bytes_base64: Vec<String>,
        plc_op: &'a SignedPlcOperation,
        plc_hash: &'a str,
    }
    let output = Output {
        signing_key_priv_bytes_base64: BASE64_STANDARD.encode(signing_key.private_key_bytes()),
        rotation_keys_priv_bytes_base64: rotation_keys
            .iter()
            .map(|k| BASE64_STANDARD.encode(k.private_key_bytes()))
            .collect(),
        plc_op,
        plc_hash,
    };

    serde_json::ser::to_writer_pretty(file, &output).expect("Failed to serialize results");
}