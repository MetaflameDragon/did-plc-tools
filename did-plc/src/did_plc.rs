use std::fmt::Display;

use sha2::Digest;
use thiserror::Error;

use crate::SignedPlcOperation;

const DID_PLC_PREFIX: &'static str = "did:plc:";
const PLC_HASH_BASE32_LENGTH: usize = 24;

// Rounded-up division isn't necessary for the current 24 characters - but just in case
const PLC_HASH_BYTE_COUNT: usize = usize::div_ceil(PLC_HASH_BASE32_LENGTH * 5, 8);

const PLC_HASH_ALPHABET: base32::Alphabet = base32::Alphabet::Rfc4648Lower { padding: false };

// TODO custom debug
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DidPlc {
    hash_bytes: [u8; PLC_HASH_BYTE_COUNT],
}

impl DidPlc {
    pub fn from_signed_op(signed_op: &SignedPlcOperation) -> Self {
        let signed_op_serialized = serde_ipld_dagcbor::ser::to_vec(signed_op)
            .expect("Signed operation serialization failed");

        let signed_op_hash = sha2::Sha256::digest(&signed_op_serialized);
        debug_assert!(signed_op_hash.len() >= PLC_HASH_BYTE_COUNT); // Sanity check
        let signed_op_hash_trunc = &signed_op_hash[..PLC_HASH_BYTE_COUNT];

        Self {
            // Should never fail, since slice length = hash_bytes length
            hash_bytes: signed_op_hash_trunc.try_into().unwrap(),
        }
    }

    pub fn hash_bytes(&self) -> &[u8; PLC_HASH_BYTE_COUNT] {
        &self.hash_bytes
    }

    pub fn hash_encoded(&self) -> String {
        base32::encode(PLC_HASH_ALPHABET, &self.hash_bytes)
    }

    pub fn formatted_did(&self) -> String {
        format!("did:plc:{}", self.hash_encoded())
    }
}

impl TryFrom<&str> for DidPlc {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        try_parse_formatted(value)
    }
}

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Missing did:plc: prefix")]
    MissingPrefix,
    #[error("Formatted value was an invalid length")]
    InvalidLength,
    #[error("Unexpected hash format (must be base32 RFC4648, lowercase)")]
    InvalidHash,
}

fn try_parse_formatted(formatted_value: &str) -> Result<DidPlc, Error> {
    if !formatted_value.starts_with(DID_PLC_PREFIX) {
        return Err(Error::MissingPrefix);
    }

    try_parse_hash(&formatted_value[DID_PLC_PREFIX.len()..])
}

fn try_parse_hash(hash: &str) -> Result<DidPlc, Error> {
    if hash.len() != PLC_HASH_BASE32_LENGTH {
        return Err(Error::InvalidLength);
    }

    let bytes = base32::decode(PLC_HASH_ALPHABET, hash).ok_or(Error::InvalidHash)?;
    debug_assert_eq!(bytes.len(), PLC_HASH_BYTE_COUNT); // Should always hold true

    // Force-convert to a fixed byte array (as of currently, 24 base32 chars = 15 bytes)
    Ok(DidPlc {
        hash_bytes: bytes.try_into().unwrap(),
    })
}

impl Display for DidPlc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_did())
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;
    #[test]
    fn valid_plc() {
        let plc = "did:plc:c6te24qg5hx54qgegqylpqkx";
        assert_matches!(try_parse_formatted(plc), Ok(_));
    }

    #[test]
    fn plc_hash_matches() {
        let hash_value = sha2::Sha256::digest(b"rawr uwu");
        // base32 = 5 bits per char
        // This means 15 bytes for a 24-char encoded hash
        let hash_bytes_trunc = &hash_value.as_slice()[..PLC_HASH_BYTE_COUNT];
        let encoded_hash = base32::encode(PLC_HASH_ALPHABET, &hash_bytes_trunc);

        let plc = format!("did:plc:{}", encoded_hash);
        let parsed_bytes = try_parse_formatted(&plc);
        assert_matches!(parsed_bytes, Ok(_));
        assert_eq!(parsed_bytes.unwrap().hash_bytes(), hash_bytes_trunc);
    }

    #[test]
    fn formatted_matches() {
        let plc = "did:plc:c6te24qg5hx54qgegqylpqkx";
        let parsed: DidPlc = plc.try_into().unwrap();
        assert_eq!(parsed.formatted_did(), plc)
    }

    #[test]
    fn missing_prefix() {
        let plc = "c6te24qg5hx54qgegqylpqkx";
        assert_matches!(try_parse_formatted(plc), Err(Error::MissingPrefix));
    }

    #[test]
    fn invalid_length() {
        let plc = "did:plc:c6te24qg5hx54qgegqylpqkxabcd";
        assert_matches!(try_parse_formatted(plc), Err(Error::InvalidLength));
    }

    #[test]
    fn invalid_hash_chars() {
        let plc = "did:plc:c6te24qg5hx54qgegqyl0189";
        assert_matches!(try_parse_formatted(plc), Err(Error::InvalidHash));
    }
}
