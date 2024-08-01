use crate::internal_macros::define_extension_trait;

define_extension_trait! {
    /// An extension trait for the `String` type that provides additional methods for working with script buffers.
    pub trait ScriptBufExt impl for String {
        /// Creates a new script buffer for a pay-to-public-key (P2PK) script.
        ///
        /// # Arguments
        ///
        /// * `pubkey` - The public key to be used in the script.
        ///
        /// # Returns
        ///
        /// The generated script buffer.
        fn new_p2pk(pubkey: u64) -> Self {
            format!("OP_PUSHBYTES_{}", pubkey)
        }

        /// Creates a new script buffer for a pay-to-public-key-hash (P2PKH) script.
        ///
        /// # Arguments
        ///
        /// * `pubkey_hash` - The public key hash to be used in the script.
        ///
        /// # Returns
        ///
        /// The generated script buffer.
        fn new_p2pkh(pubkey_hash: String) -> Self {
            format!("Hash_{}", pubkey_hash)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_p2pk() {
        let pubkey = 123;
        let expected = "OP_PUSHBYTES_123".to_string();
        assert_eq!(String::new_p2pk(pubkey), expected);
    }

    #[test]
    fn test_new_p2pkh() {
        let pubkey_hash = "abc".to_string();
        let expected = "Hash_abc".to_string();
        assert_eq!(String::new_p2pkh(pubkey_hash), expected);
    }
}
