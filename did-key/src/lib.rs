use derive_more::Into;
use multibase::Base;
use p256::PublicKey as P256PublicKey;
use secp256k1::PublicKey as Secp256k1PublicKey;
use serde::Serialize;
use std::io::Read;

/// A String newtype representing public key bytes in the did:key:<mb-value>
/// format.
#[derive(Into)]
pub struct DidKey {
    #[into]
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

pub trait SupportedKeyAlgo: Multicodec {
    fn pub_key_bytes(&self) -> Box<[u8]>;
}

pub trait Multicodec {
    fn multicodec_bytes() -> &'static [u8];
}

impl Multicodec for P256PublicKey {
    fn multicodec_bytes() -> &'static [u8] {
        &[0x12, 0x00]
    }
}

impl Multicodec for Secp256k1PublicKey {
    fn multicodec_bytes() -> &'static [u8] {
        &[0xe7]
    }
}

impl SupportedKeyAlgo for Secp256k1PublicKey {
    fn pub_key_bytes(&self) -> Box<[u8]> {
        Box::new(self.serialize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
