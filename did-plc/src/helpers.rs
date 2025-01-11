use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use did_plc::SignedPlcOperation;
use serde::Serialize;
use std::{fs::File, path::PathBuf};

pub fn write_results(
    signing_key_priv: &secp256k1::SecretKey,
    rotation_keys_priv: &[secp256k1::SecretKey],
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
        signing_key_priv_bytes_base64: BASE64_STANDARD.encode(signing_key_priv.secret_bytes()),
        rotation_keys_priv_bytes_base64: rotation_keys_priv
            .iter()
            .map(|k| BASE64_STANDARD.encode(k.secret_bytes()))
            .collect(),
        plc_op,
        plc_hash,
    };

    serde_json::ser::to_writer_pretty(file, &output).expect("Failed to serialize results");
}
