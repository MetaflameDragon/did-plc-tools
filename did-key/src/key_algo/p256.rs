use crate::key_algo::{Multicodec, SupportedKeyAlgo};
use p256::PublicKey;

impl Multicodec for PublicKey {
    fn multicodec_bytes() -> &'static [u8] {
        &[0x12, 0x00]
    }
}

impl SupportedKeyAlgo for PublicKey {
    fn pub_key_bytes(&self) -> Box<[u8]> {
        #[allow(unreachable_code)]
        Box::new([todo!()])
    }
}
