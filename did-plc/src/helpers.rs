// use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
// use did_plc::operation::signed::SignedPlcOperation;
// use serde::Serialize;
// use std::{fs::File, path::PathBuf};
// use elliptic_curve::pkcs8::EncodePrivateKey;

// pub fn write_results<S>(
//     signing_key_priv: &S,
//     rotation_keys_priv: &[S],
//     plc_op: &SignedPlcOperation,
//     plc_hash: &str,
//     file_name: &str,
// ) where
//     S: EncodePrivateKey
// {
//     let file_path = PathBuf::from(file_name);
//
//     let file = File::options()
//         .write(true)
//         .create_new(true)
//         .open(file_path.clone());
//     let file = match file {
//         Ok(file) => file,
//         Err(err) => panic!("Could not write results to {}: {err}", file_path.display()),
//     };
//
//     #[derive(Serialize)]
//     struct Output<'a> {
//         signing_key_priv_bytes_base64: String,
//         rotation_keys_priv_bytes_base64: Vec<String>,
//         plc_op: &'a SignedPlcOperation,
//         plc_hash: &'a str,
//     }
//     let output = Output {
//         signing_key_priv_bytes_base64: BASE64_STANDARD.encode(signing_key_priv.to_bytes().as_ref()),
//         rotation_keys_priv_bytes_base64: rotation_keys_priv
//             .iter()
//             .map(|k| BASE64_STANDARD.encode(k.to_bytes().as_ref()))
//             .collect(),
//         plc_op,
//         plc_hash,
//     };
//
//     serde_json::ser::to_writer_pretty(file, &output).expect("Failed to serialize results");
// }
