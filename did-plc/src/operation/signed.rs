use base64::engine::general_purpose::URL_SAFE as BASE64_URL_SAFE;
use base64::Engine;
use cid::Cid;
use ecdsa::signature::Signer;
use ecdsa::{Signature, SignatureEncoding};
use elliptic_curve::{CurveArithmetic, PrimeCurve};
use serde::{Deserialize, Serialize};

use crate::did_plc::DidPlc;
use crate::operation::unsigned::UnsignedPlcOperation;
use crate::PlcBlessedKeyCurve;

/// Represents a signed PLC operation (unsigned operation + `sig`).
///
/// Field order matters for `serde_json`, and matches the order
/// used by [plc.directory](https://plc.directory).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignedPlcOperation {
    sig: SignatureBase64Url,
    #[serde(flatten)]
    inner: UnsignedPlcOperation,
}

impl SignedPlcOperation {
    pub fn new<S, C>(unsigned_op: UnsignedPlcOperation, signing_key: &S) -> Self
    where
        // Curve C must be "blessed" (allowed by spec), and Signing key S must sign with curve C
        C: PlcBlessedKeyCurve,
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

    pub fn get_cid_pointer(&self) -> Cid {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureBase64Url(String);

#[cfg(test)]
mod tests {
    use std::io::BufWriter;

    use super::*;

    #[test]
    pub fn json_serde_matches() {
        let plc_op_json = r#"
        {
            "sig": "MDnVsVKDj-X2iHDtt9bX4xN8yIFruMexTHGFeLczgJZv-RNErz_Kg0mQDhEjezX158cP5-XBHPZ1nQ1K44OEFQ",
            "prev": "bafyreieg4qrrfepem7fpnsurihrenghjjqn7ebx5kansmdizmcxsdvtfku",
            "type": "plc_operation",
            "services": {
                "atproto_pds": {
                    "type": "AtprotoPersonalDataServer",
                    "endpoint": "https://magic.us-west.host.bsky.network"
                }
            },
            "alsoKnownAs": [
                "at://test.metaflame.dev",
                "at://alt.test.metaflame.dev"
            ],
            "rotationKeys": [
                "did:key:zQ3shhCGUqDKjStzuDxPkTxN6ujddP4RkEKJJouJGRRkaLGbg",
                "did:key:zQ3shpKnbdPx3g3CmPf5cRVTPe1HtSwVn5ish3wSnDPQCbLJK",
                "did:key:zQ3shb9nQ22CdsmTCKoeHnwTXXB9i12Uh2XT3vyCHhgaJWBUL"
            ],
            "verificationMethods": {
                "atproto": "did:key:zQ3shTuHbPL5uNPWmz5Tf6W1EWrhjWnxsCxNx9C7SdKqL1JXe"
            }
        }
        "#;

        let mut reserialized = BufWriter::new(Vec::new());

        let mut de_json = serde_json::Deserializer::from_str(plc_op_json);
        let mut ser_json = serde_json::Serializer::new(&mut reserialized);

        serde_transcode::transcode(&mut de_json, &mut ser_json).unwrap();

        let plc_op: SignedPlcOperation = serde_json::de::from_str(&plc_op_json).unwrap();
        let plc_ser = serde_json::to_string(&plc_op).unwrap();

        // TODO: fails on extra forward slash in PDS endpoint
        assert_eq!(
            plc_ser,
            String::from_utf8(reserialized.buffer().to_vec()).unwrap()
        );
    }
}
