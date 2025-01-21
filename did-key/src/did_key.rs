use crypto_traits::MulticodecPrefix;
use derive_more::Into;
use k256::PublicKey as Secp256k1PublicKey;
use multibase::Base;
use p256::PublicKey as P256PublicKey;
use serde::{Deserialize, Serialize};

// prevent accidental ambiguous use of PublicKey
type PublicKey = !;

const DID_KEY_PREFIX: &str = "did:key:";

/// A String newtype representing public key bytes in the did:key:<mb-value>
/// format.
#[derive(Into, Serialize, Deserialize, Clone, Debug)] // TODO Fix deserialize
#[serde(into = "String")]
pub struct DidKey {
    #[into]
    formatted_value: String,
}

impl From<Secp256k1PublicKey> for DidKey {
    fn from(public_key: Secp256k1PublicKey) -> Self {
        let multicodec_prefix = Secp256k1PublicKey::multicodec_prefix_unsigned_varint();
        let key_bytes = public_key.to_sec1_bytes();

        make_did_key(&multicodec_prefix, &key_bytes)
    }
}

impl From<P256PublicKey> for DidKey {
    fn from(public_key: P256PublicKey) -> Self {
        let multicodec_prefix = P256PublicKey::multicodec_prefix_unsigned_varint();
        let key_bytes = public_key.to_sec1_bytes();

        make_did_key(&multicodec_prefix, &key_bytes)
    }
}

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
    use rand::rngs::mock::StepRng;

    fn gen_did_key() -> DidKey {
        let mut rng = StepRng::new(2, 1);
        let (_secret_key, public_key) = secp256k1::generate_keypair(&mut rng);

        let did_key = DidKey::from_public_key(&public_key);
        did_key
    }

    #[test]
    fn encode_key() {
        let did_key = gen_did_key();

        assert_eq!(
            did_key.formatted_value(),
            "did:key:zQ3shtkBf66Yd4GgTkAdgJ7Tge3Wj1w7Xbi6q1TiUG6BXmKVr"
        );
    }

    #[test]
    fn serialize_key() {
        let did_key = gen_did_key();

        assert_eq!(
            serde_json::to_string_pretty(&did_key).unwrap(),
            r#""did:key:zQ3shtkBf66Yd4GgTkAdgJ7Tge3Wj1w7Xbi6q1TiUG6BXmKVr""#
        );
    }
}
