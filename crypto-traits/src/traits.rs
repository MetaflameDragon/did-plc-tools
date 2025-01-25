pub trait MulticodecPrefix {
    /// Returns the Multicodec prefix for a type.
    /// The prefix is returned exactly as described [here](https://github.com/multiformats/multicodec/blob/86f0e145b1c52a40395ec07609e989f525bf86e6/table.csv).
    ///
    /// See [multicodec_prefix_unsigned_varint](MulticodecPrefix::multicodec_prefix_unsigned_varint)
    /// for the variant used with did:key
    fn multicodec_prefix_raw() -> u64;

    /// Returns the Multicodec prefix in the unsigned varint format.
    ///
    /// This is the format used by did:key.
    fn multicodec_prefix_unsigned_varint() -> Vec<u8> {
        let mut buf = unsigned_varint::encode::u64_buffer();
        let out_buf = unsigned_varint::encode::u64(Self::multicodec_prefix_raw(), &mut buf);
        out_buf.to_vec()
    }
}
