use derive_more::Into;
use multibase::Base;
use serde::{Deserialize, Serialize};
use crate::key_algo::{SupportedKeyAlgo};

/// A String newtype representing public key bytes in the did:key:<mb-value>
/// format.
#[derive(Into, Serialize, Deserialize, Debug)]
pub struct DidKey {
    #[into]
    #[serde(flatten)]
    formatted_value: String,
}

impl DidKey {
    pub fn from_public_key<K: SupportedKeyAlgo>(public_key: K) -> DidKey {
        let mb_value = multibase::encode(
            Base::Base58Btc,
            itertools::chain(K::multicodec_bytes(), public_key.pub_key_bytes().iter())
                .copied()
                .collect::<Vec<u8>>(),
        );

        DidKey {
            formatted_value: format!("did:key:{mb_value}"),
        }
    }

    pub fn formatted_value(&self) -> &str {
        &self.formatted_value
    }
}

#[cfg(test)]
mod tests {
    use crate::did_key::DidKey;
    use rand::rngs::mock::StepRng;

    #[test]
    fn encode_key() {
        let mut rng = StepRng::new(2, 1);
        let (_secret_key, public_key) = secp256k1::generate_keypair(&mut rng);

        let did_key = DidKey::from_public_key(public_key);

        assert_eq!(
            did_key.formatted_value(),
            "did:key:z6DuCMU2vmvYpdavvpDwStgrKHsf6h8fiAoLRGkntD8jj37W"
        );
    }
}
