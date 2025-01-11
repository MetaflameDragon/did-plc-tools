use crate::key_algo::{Multicodec, SupportedKeyAlgo};
use secp256k1::PublicKey;

impl Multicodec for PublicKey {
    fn multicodec_bytes() -> &'static [u8] {
        &[0xe7]
    }
}

impl SupportedKeyAlgo for PublicKey {
    fn pub_key_bytes(&self) -> Box<[u8]> {
        Box::new(self.serialize())
    }
}
