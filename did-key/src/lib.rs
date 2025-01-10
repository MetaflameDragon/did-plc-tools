use derive_more::Into;
use multibase::Base;
use std::io::Read;

/// A String newtype representing public key bytes in the did:key:<mb-value>
/// format.
#[derive(Into)]
pub struct DidKey {
    #[into]
    formatted_value: String,
}

impl DidKey {
    pub fn from_public_key(multicodec_key_bytes: &[u8], public_key_bytes: &[u8]) -> DidKey {
        // TODO: Check key byte count
        let mb_value = multibase::encode(
            Base::Base58Btc,
            itertools::chain(public_key_bytes, multicodec_key_bytes)
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
    use super::*;
    use rand::rngs::mock::StepRng;

    #[test]
    fn encode_key() {
        let mut rng = StepRng::new(2, 1);
        let (_secret_key, public_key) = secp256k1::generate_keypair(&mut rng);

        let did_key = DidKey::from_public_key(&[0xe7], &public_key.serialize());

        assert_eq!(
            did_key.formatted_value(),
            "did:key:z61QAFNa5KZ9vJsJStq3yTWSPZESoPd8yYP1eiy52nwXgut"
        );
    }
}
