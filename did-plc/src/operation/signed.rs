use elliptic_curve::{CurveArithmetic, PrimeCurve};
use ecdsa::signature::Signer;
use ecdsa::{Signature, SignatureEncoding};
use base64::engine::general_purpose::URL_SAFE as BASE64_URL_SAFE;
use serde::{Deserialize, Serialize};
use base64::Engine;
use crate::operation::unsigned::UnsignedPlcOperation;
use crate::PlcBlessedKeyType;
use crate::did_plc::DidPlc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignedPlcOperation {
    #[serde(flatten)]
    inner: UnsignedPlcOperation,
    sig: SignatureBase64Url,
}

impl SignedPlcOperation {
    pub fn new<S, C>(unsigned_op: UnsignedPlcOperation, signing_key: &S) -> Self
    where
        // Curve C must be "blessed" (allowed by spec), and Signing key S must sign with curve C
        C: PlcBlessedKeyType,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureBase64Url(String);