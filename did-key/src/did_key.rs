use crypto_traits::MulticodecPrefix;
use derive_more::Into;
use elliptic_curve::PublicKey;
use k256::Secp256k1;
use multibase::Base;
use p256::NistP256;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const DID_KEY_PREFIX: &str = "did:key:";

/// A String newtype representing public key bytes in the did:key:<mb-value>
/// format.
#[derive(Into, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(into = "String", try_from = "String")]
pub struct DidKey {
    #[into]
    formatted_value: String,
}

impl From<PublicKey<Secp256k1>> for DidKey {
    fn from(public_key: PublicKey<Secp256k1>) -> Self {
        let multicodec_prefix = PublicKey::<Secp256k1>::multicodec_prefix_unsigned_varint();
        let key_bytes = public_key.to_sec1_bytes();

        make_did_key(&multicodec_prefix, &key_bytes)
    }
}

impl From<PublicKey<NistP256>> for DidKey {
    fn from(public_key: PublicKey<NistP256>) -> Self {
        let multicodec_prefix = PublicKey::<NistP256>::multicodec_prefix_unsigned_varint();
        let key_bytes = public_key.to_sec1_bytes();

        make_did_key(&multicodec_prefix, &key_bytes)
    }
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum Error {
    #[error(r#"Missing "did:key:" prefix"#)]
    MissingPrefix,
    #[error("Invalid did:key multibase value")]
    InvalidValue,
}

impl TryFrom<String> for DidKey {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !value.starts_with(DID_KEY_PREFIX) {
            return Err(Error::MissingPrefix);
        }
        // TODO: Verify value
        Ok(DidKey {
            formatted_value: value,
        })
    }
}

/// Creates a did:key using a multicodec prefix + key bytes.
/// The key is composed of a prefix (did:key:) and the Base58Btc-encoded value of those bytes.
///
/// Described [here](https://w3c-ccg.github.io/did-method-key/#format).
fn make_did_key(multicodec_prefix: &[u8], key_bytes: &[u8]) -> DidKey {
    let mb_value = multibase::encode(
        Base::Base58Btc,
        itertools::chain(multicodec_prefix, key_bytes)
            .copied()
            .collect::<Vec<u8>>(),
    );

    DidKey {
        formatted_value: format!("{DID_KEY_PREFIX}{mb_value}"),
    }
}

impl DidKey {
    /// The identifier part of the key (after "did:key:")
    ///
    /// Example: `zQ3shtkBf66Yd4GgTkAdgJ7Tge3Wj1w7Xbi6q1TiUG6BXmKVr`
    pub fn multibase_value(&self) -> &str {
        let prefix_len = DID_KEY_PREFIX.len();
        &self.formatted_value[prefix_len..]
    }

    /// The whole "did:key:\[...\]" representation
    ///
    /// Example: `did:key:zQ3shtkBf66Yd4GgTkAdgJ7Tge3Wj1w7Xbi6q1TiUG6BXmKVr`
    pub fn formatted_value(&self) -> &str {
        &self.formatted_value
    }
}

#[cfg(test)]
mod tests {
    use crate::did_key::DidKey;
    use elliptic_curve::ScalarPrimitive;

    fn gen_did_key() -> DidKey {
        let secret_key = k256::SecretKey::new(ScalarPrimitive::from(1));
        let public_key = secret_key.public_key();

        public_key.into()
    }

    #[test]
    fn encode_key() {
        let did_key = gen_did_key();

        assert_eq!(
            did_key.formatted_value(),
            "did:key:zQ3shVc2UkAfJCdc1TR8E66J85h48P43r93q8jGPkPpjF9Ef9"
        );
    }

    #[test]
    fn serialize_key() {
        let did_key = gen_did_key();

        assert_eq!(
            serde_json::to_string_pretty(&did_key).unwrap(),
            r#""did:key:zQ3shVc2UkAfJCdc1TR8E66J85h48P43r93q8jGPkPpjF9Ef9""#
        );
    }

    #[test]
    fn deserialize_key() {
        let did_key = gen_did_key();
        let did_key_str = serde_json::to_string(&did_key).unwrap();

        let did_key_deser =
            serde_json::de::from_str::<DidKey>(&did_key_str).expect("Failed to deserialize");

        assert_eq!(did_key_deser, did_key);
    }

    #[test]
    fn deserialize_invalid_prefix() {
        let invalid_str = r#""did:other:abcd""#;

        let err = serde_json::de::from_str::<DidKey>(invalid_str)
            .expect_err("Failed to reject invalid format");

        dbg!(&err);
        println!("{err:#?}");

        assert!(err.is_data());
        // TODO: does not forward internal error enum!
    }
}
