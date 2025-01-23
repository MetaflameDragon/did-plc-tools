use std::fmt::Display;
use sha2::Digest;
use crate::SignedPlcOperation;

pub struct DidPlc {
    plc_hash: String,
}

impl DidPlc {
    pub fn new(plc_hash: String) -> Self {
        DidPlc { plc_hash }
    }

    pub fn from_signed_op(signed_op: &SignedPlcOperation) -> Self {
        let signed_op_serialized = serde_ipld_dagcbor::ser::to_vec(signed_op)
            .expect("Signed operation serialization failed");

        let signed_op_hash = sha2::Sha256::digest(&signed_op_serialized);

        let plc_hash = &base32::encode(
            base32::Alphabet::Rfc4648Lower { padding: false },
            signed_op_hash.as_slice(),
        )[..24];

        Self::new(plc_hash.to_string())
    }

    pub fn plc_hash(&self) -> &str {
        &self.plc_hash
    }

    pub fn to_did_str(&self) -> String {
        format!("did:plc:{}", self.plc_hash)
    }
}

impl Display for DidPlc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_did_str())
    }
}