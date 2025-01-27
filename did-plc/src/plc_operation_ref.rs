use std::fmt::Display;

use cid::Cid;
use derive_more::{Deref, Into};
use multibase::Base;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

/// Wrapper for [`cid::Cid`].
///
/// CID parameters for did:plc's `prev` field ([specs](https://web.plc.directory/spec/v0.1/did-plc)) are:
/// - CIDv1
/// - `base32` multibase encoding (prefix: `b`)
/// - `dag-cbor` multibase type (code: `0x71`)
/// - `sha-256` multihash (code: `0x12`)
///
/// `serde` uses string encoding for the CID (as opposed to a binary IPLD "link")
#[derive(Debug, Copy, Clone, Deref, Into, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PlcOperationRef(#[deref] Cid);

#[derive(Debug, Error)]
pub enum Error {
    #[error("PLC operation references must use CIDv1 (was `{0:?}`)")]
    InvalidVersion(cid::Version),
    #[error("Invalid CID codec/multibase type (must be dag-cbor, 0x71; was `{0}`)")]
    InvalidCodec(u64),
    #[error("Invalid CID multihash (must be sha-256, 0x12; was `{0}`)")]
    InvalidMultihash(u64),
    #[error(transparent)]
    CidError(cid::Error),
}

impl TryFrom<Cid> for PlcOperationRef {
    type Error = Error;

    fn try_from(value: Cid) -> Result<Self, Self::Error> {
        if value.version() != cid::Version::V1 {
            // Can't use conversion to v1 here, v0 defaults to the dag-pb codec (0x70)
            // Maybe this can be converted safely though?
            return Err(Error::InvalidVersion(value.version()));
        }

        if value.codec() != 0x71 {
            return Err(Error::InvalidCodec(value.codec()));
        }

        if value.hash().code() != 0x12 {
            return Err(Error::InvalidMultihash(value.hash().code()));
        }

        Ok(Self(value))
    }
}

impl Display for PlcOperationRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Can unwrap here, because CID is always v1
        // The string conversion fails for CIDv0 if the base isn't the only supported Base58Btc
        let str = &self
            .0
            .to_string_of_base(Base::Base32Lower)
            .expect("Unexpected serialization error");
        f.write_str(str)
    }
}

impl TryFrom<&str> for PlcOperationRef {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let cid = Cid::try_from(value).map_err(|err| Error::CidError(err))?;
        PlcOperationRef::try_from(cid)
    }
}

impl TryFrom<String> for PlcOperationRef {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        PlcOperationRef::try_from(value.as_str())
    }
}

impl Serialize for PlcOperationRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for PlcOperationRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = String::deserialize(deserializer).map_err(serde::de::Error::custom)?;
        PlcOperationRef::try_from(str).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use multihash_codetable::{Code, MultihashDigest};

    use super::*;

    #[test]
    fn from_str() {
        let prev_cid_str = "bafyreieg4qrrfepem7fpnsurihrenghjjqn7ebx5kansmdizmcxsdvtfku";
        let op_ref = PlcOperationRef::try_from(prev_cid_str);

        assert_matches!(op_ref, Ok(_));
    }

    #[test]
    fn from_str_err() {
        // Same as in [test_from_str] without the first 'b' char
        let prev_cid_str = "afyreieg4qrrfepem7fpnsurihrenghjjqn7ebx5kansmdizmcxsdvtfku";
        let op_ref = PlcOperationRef::try_from(prev_cid_str);

        assert_matches!(op_ref, Err(Error::CidError(cid::Error::ParsingError)));
    }

    #[test]
    fn from_str_non_base32() {
        // Same as in [test_from_str], 2nd char is 8 (not base32)
        let prev_cid_str = "b8fyreieg4qrrfepem7fpnsurihrenghjjqn7ebx5kansmdizmcxsdvtfku";
        let op_ref = PlcOperationRef::try_from(prev_cid_str);

        assert_matches!(op_ref, Err(Error::CidError(cid::Error::ParsingError)));
    }

    #[test]
    fn invalid_version() {
        let multihash = Code::Sha2_256.digest(b"deadbeef");

        let cid = Cid::new_v0(multihash).unwrap();
        let op_ref = PlcOperationRef::try_from(cid);
        assert_matches!(op_ref, Err(Error::InvalidVersion(cid::Version::V0)));
    }

    #[test]
    fn invalid_codec() {
        let multihash = Code::Sha2_256.digest(b"deadbeef");

        let cid = Cid::new_v1(0x70, multihash);
        let op_ref = PlcOperationRef::try_from(cid);
        assert_matches!(op_ref, Err(Error::InvalidCodec(0x70)));
    }

    #[test]
    fn invalid_multihash() {
        let multihash = Code::Sha2_512.digest(b"deadbeef");

        let cid = Cid::new_v1(0x71, multihash);
        let op_ref = PlcOperationRef::try_from(cid);
        assert_matches!(op_ref, Err(Error::InvalidMultihash(0x13)));
    }

    #[test]
    fn valid_cid() {
        let multihash = Code::Sha2_256.digest(b"deadbeef");

        let cid = Cid::new_v1(0x71, multihash);
        let op_ref = PlcOperationRef::try_from(cid);
        assert_matches!(op_ref, Ok(_));
    }
}
