use crate::key_algo::SupportedKeyAlgo;
use derive_more::Into;
use multibase::Base;
use serde::{Deserialize, Serialize};

const DID_KEY_PREFIX: &str = "did:key:";

/// A String newtype representing public key bytes in the did:key:<mb-value>
/// format.
#[derive(Into, Serialize, Deserialize, Clone, Debug)] // TODO Fix deserialize
#[serde(into = "String")]
pub struct DidKey {
    #[into]
    formatted_value: String,
}

impl DidKey {
    pub fn from_public_key<K: SupportedKeyAlgo>(public_key: &K) -> DidKey {
        let mb_value = multibase::encode(
            Base::Base58Btc,
            itertools::chain(K::multicodec_bytes(), public_key.pub_key_bytes().iter())
                .copied()
                .collect::<Vec<u8>>(),
        );

        DidKey {
            formatted_value: format!("{DID_KEY_PREFIX}{mb_value}"),
        }
    }

    /// The identifier part of the key (after "did:key:")
    pub fn multibase_value(&self) -> &str {
        let prefix_len = DID_KEY_PREFIX.len();
        &self.formatted_value[prefix_len..]
    }

    /// The whole "did:key:\[...\]" representation
    pub fn formatted_value(&self) -> &str {
        &self.formatted_value
    }
}

impl<T: SupportedKeyAlgo> From<&T> for DidKey {
    fn from(value: &T) -> Self {
        Self::from_public_key(value)
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
            "did:key:z6DuCMU2vmvYpdavvpDwStgrKHsf6h8fiAoLRGkntD8jj37W"
        );
    }

    #[test]
    fn serialize_key() {
        let did_key = gen_did_key();

        assert_eq!(
            serde_json::to_string_pretty(&did_key).unwrap(),
            r#""did:key:z6DuCMU2vmvYpdavvpDwStgrKHsf6h8fiAoLRGkntD8jj37W""#
        );
    }
}
