use crate::traits::*;

impl MulticodecPrefix for p256::PublicKey {
    fn multicodec_prefix_raw() -> u64 {
        0x1200
    }
}

impl MulticodecPrefix for k256::PublicKey {
    fn multicodec_prefix_raw() -> u64 {
        0xe7
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn varint_p256() {
        let varint = p256::PublicKey::multicodec_prefix_unsigned_varint();
        assert_eq!(varint, &[0x80, 0x24])
    }

    #[test]
    fn varint_k256() {
        let varint = k256::PublicKey::multicodec_prefix_unsigned_varint();
        assert_eq!(varint, &[0xe7, 0x01])
    }
}
