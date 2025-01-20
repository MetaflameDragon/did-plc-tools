use crate::key_algo::{Multicodec, SupportedKeyAlgo};
use secp256k1::PublicKey;

impl Multicodec for PublicKey {
    fn multicodec_bytes() -> &'static [u8] {
        // https://github.com/bluesky-social/atproto/blob/5417476622ff5a97daaa00e2f57bae90dea2a22e/packages/crypto/src/const.ts#L2
        // Not the standard byte prefix (?)
        &[0xe7, 0x01]
    }
}

impl SupportedKeyAlgo for PublicKey {
    fn pub_key_bytes(&self) -> Box<[u8]> {
        Box::new(self.serialize())
    }
}
