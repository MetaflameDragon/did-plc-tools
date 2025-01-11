pub mod p256;
pub mod secp256k1;

pub trait SupportedKeyAlgo: Multicodec {
    fn pub_key_bytes(&self) -> Box<[u8]>;
}

pub trait Multicodec {
    fn multicodec_bytes() -> &'static [u8];
}
