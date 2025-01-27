use std::collections::TryReserveError;
use std::fmt::Display;

use cid::multihash::Multihash;
use cid::{multihash, Cid};
use derive_more::{Deref, From, Into};
use multibase::Base;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_ipld_dagcbor::EncodeError;
use sha2::Digest;
use thiserror::Error;

use crate::plc_operation_ref::codes::{PLC_MULTIBASE_CODEC, PLC_MULTIHASH_CODE};
use crate::SignedPlcOperation;

mod codes {
    use multibase::Base;

    pub const PLC_MULTIBASE_ENCODING: Base = Base::Base32Lower;
    pub const PLC_MULTIBASE_CODEC: u64 = MULTIBASE_TYPE_DAG_CBOR;
    pub const PLC_MULTIHASH_CODE: u64 = MULTIHASH_SHA_256;

    const MULTIBASE_TYPE_DAG_CBOR: u64 = 0x71;
    const MULTIHASH_SHA_256: u64 = 0x12;
}

/// Wrapper for [`cid::Cid`].
///
/// CID parameters for did:plc's `prev` field ([specs](https://web.plc.directory/spec/v0.1/did-plc)) are:
/// - CIDv1
/// - `base32` multibase encoding (prefix: `b`)
/// - `dag-cbor` multibase type (code: `0x71`)
/// - `sha-256` multihash (code: `0x12`)
///
/// `serde` uses string encoding for the CID (as opposed to a binary IPLD "link")
#[derive(Debug, Copy, Clone, Into, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PlcOperationRef(Cid);

#[derive(Debug, Error)]
pub enum Error {
    #[error("PLC operation references must use CIDv1 (was `{0:?}`)")]
    InvalidVersion(cid::Version),
    #[error("Invalid CID codec/multibase type (must be dag-cbor, 0x71; was `{0}`)")]
    InvalidCodec(u64),
    #[error("Invalid CID multihash (must be sha-256, 0x12; was `{0}`)")]
    InvalidMultihash(u64),
    #[error(transparent)]
    CidError(#[from] cid::Error),
    #[error(transparent)]
    EncodeError(#[from] EncodeError<TryReserveError>),
    #[error(transparent)]
    MultihashError(#[from] multihash::Error),
}

fn validate_cid(cid: &Cid) -> Result<(), Error> {
    if cid.version() != cid::Version::V1 {
        // Can't use conversion to v1 here, v0 defaults to the dag-pb codec (0x70)
        // Maybe this can be converted safely though?
        return Err(Error::InvalidVersion(cid.version()));
    }

    if cid.codec() != PLC_MULTIBASE_CODEC {
        return Err(Error::InvalidCodec(cid.codec()));
    }

    if cid.hash().code() != PLC_MULTIHASH_CODE {
        return Err(Error::InvalidMultihash(cid.hash().code()));
    }

    Ok(())
}

impl TryFrom<Cid> for PlcOperationRef {
    type Error = Error;

    fn try_from(value: Cid) -> Result<Self, Self::Error> {
        validate_cid(&value)?;
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
        let cid = Cid::try_from(value)?;
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

impl PlcOperationRef {
    pub fn new(hash: Multihash<64>) -> Result<Self, Error> {
        let cid = Cid::new_v1(PLC_MULTIBASE_CODEC, hash);
        validate_cid(&cid)?;
        Ok(Self(cid))
    }

    pub fn cid(&self) -> &Cid {
        &self.0
    }
    pub fn from_signed_op(plc_op: &SignedPlcOperation) -> Result<PlcOperationRef, Error> {
        // TODO fix
        todo!();
        let bytes = serde_ipld_dagcbor::ser::to_vec(plc_op)?;
        Self::from_dag_cbor(&bytes)
    }

    pub fn from_dag_cbor(bytes: &[u8]) -> Result<PlcOperationRef, Error> {
        let digest = sha2::Sha256::digest(&bytes);
        let hash = Multihash::<64>::wrap(PLC_MULTIHASH_CODE, &digest)?;

        Self::new(hash)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use multihash_codetable::{Code, MultihashDigest};
    use serde_ipld_dagcbor::ser::BufWriter;

    use super::*;
    use crate::plc_operation_ref::codes::PLC_MULTIBASE_CODEC;

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

        let cid = Cid::new_v1(PLC_MULTIBASE_CODEC, multihash);
        let op_ref = PlcOperationRef::try_from(cid);
        assert_matches!(op_ref, Err(Error::InvalidMultihash(0x13)));
    }

    #[test]
    fn valid_cid() {
        let multihash = Code::Sha2_256.digest(b"deadbeef");

        let cid = Cid::new_v1(PLC_MULTIBASE_CODEC, multihash);
        let op_ref = PlcOperationRef::try_from(cid);
        assert_matches!(op_ref, Ok(_));
    }

    #[test]
    fn plc_op_hash() {
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

        // TODO use SignedPlcOperation instead of transcode
        let mut writer = BufWriter::new(Vec::new());

        let mut deser_json = serde_json::Deserializer::from_str(plc_op_json);
        let mut ser_dag_cbor = serde_ipld_dagcbor::ser::Serializer::new(&mut writer);

        serde_transcode::transcode(&mut deser_json, &mut ser_dag_cbor).unwrap();

        // Taken from log/audit
        let expected_ref_cid = PlcOperationRef::try_from(
            "bafyreihb7r2t3qegktlhxzqdr4gca77iov2w3putiiaut5qo33mj2ket2y",
        )
        .unwrap();

        let ref_cid = PlcOperationRef::from_dag_cbor(writer.buffer());

        assert_matches!(ref_cid, Ok(_));
        assert_eq!(ref_cid.unwrap(), expected_ref_cid)
    }
}
