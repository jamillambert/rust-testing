use bech32::{self, ToBase32, Variant};
use bs58;
use sha2::{Digest, Sha256};

fn main() {
    let input = "This is string to test the encoding with";
    let bech32_str = bech32_encode(input);
    let base58check_str = base58check_encode(input);
    let base58_str_checked = convert_base58_to_base58check(base58_encode(input).as_str());
    println!("Bech32 string: {}", bech32_str);
    println!("Base58Check string: {}", base58check_str);
    println!("Base58Check string from base58: {}", base58_str_checked);
}

/// Encodes the given input string into Bech32 format.
///
/// # Arguments
///
/// * `input` - A string slice that holds the input string to be encoded.
///
/// # Returns
///
/// A `String` containing the Bech32 encoded representation of the input.
///
/// # Example
///
/// ```
/// let input = "This is string to test the encoding with";
/// let bech32_str = bech32_encode(input);
/// println!("Bech32 string: {}", bech32_str);
/// ```
fn bech32_encode(input: &str) -> String {
    let bytes = input.as_bytes();
    bech32::encode("bc", bytes.to_base32(), Variant::Bech32m).unwrap()
}

/// Encodes the given input string into Base58 format.
///
/// # Arguments
///
/// * `input` - A string slice that holds the input string to be encoded.
///
/// # Returns
///
/// A `String` containing the Base58 encoded representation of the input.
///
/// # Example
///
/// ```
/// let input = "This is string to test the encoding with";
/// let base58_str = base58_encode(input);
/// println!("Base58 string: {}", base58_str);
/// ```
fn base58_encode(input: &str) -> String {
    bs58::encode(input.as_bytes()).into_string()
}

/// Encodes the given input string into Base58Check format.
///
/// # Arguments
///
/// * `input` - A string slice that holds the input string to be encoded.
///
/// # Returns
///
/// A `String` containing the Base58Check encoded representation of the input.
///
/// # Example
///
/// ```
/// let input = "This is string to test the encoding with";
/// let base58check_str = base58check_encode(input);
/// println!("Base58Check string: {}", base58check_str);
/// ```
fn base58check_encode(input: &str) -> String {
    let mut bytes = vec![0];
    bytes.extend_from_slice(input.as_bytes());
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let hash1 = hasher.finalize_reset();
    hasher.update(&hash1);
    let hash2 = hasher.finalize();
    let checksum = &hash2[..4];
    bytes.extend_from_slice(checksum);
    bs58::encode(bytes).into_string()
}

/// Converts a Base58 encoded string into Base58Check format.
///
/// # Arguments
///
/// * `input` - A string slice that holds the Base58 encoded string to be converted.
///
/// # Returns
///
/// A `String` containing the Base58Check encoded representation of the input.
///
/// # Example
///
/// ```
/// let base58_str = base58_encode("This is string to test the encoding with");
/// let base58check_str = convert_base58_to_base58check(base58_str.as_str());
/// println!("Base58Check string from base58: {}", base58check_str);
/// ```
fn convert_base58_to_base58check(input: &str) -> String {
    let mut bytes = vec![0]; // Start with 1 (in Base58 it's represented by leading zeros)
    let original_bytes = bs58::decode(input).into_vec().unwrap();
    bytes.extend_from_slice(&original_bytes);

    // Calculate SHA-256 hash
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let hash1 = hasher.finalize_reset();

    // Calculate SHA-256 hash of the hash
    hasher.update(&hash1);
    let hash2 = hasher.finalize();

    // Take first 4 bytes of the second hash as checksum
    let checksum = &hash2[..4];

    // Append checksum to the original bytes
    bytes.extend_from_slice(checksum);

    // Encode the resulting bytes in Base58
    bs58::encode(bytes).into_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bech32_encode() {
        let input = "This is a string to test the encoding with";
        let bech32_str = bech32_encode(input);
        assert_eq!(bech32_str, "bc1235xjueqd9ejqcfqwd68y6twvus8gmeqw3jhxapqw35x2gr9de3k7erfdenjqamfw35q8xqgc2");
    }

    #[test]
    fn test_base58_encode() {
        let input = "This is a string to test the encoding with";
        let base58_str = base58_encode(input);
        assert_eq!(base58_str, "2QhwuRSWNgihbi1b3haxQ4CqKi5hhFqAEDNzX3KJjzJAM3Dd3biazcq2F9");
    }

    #[test]
    fn test_base58check_encode() {
        let input = "This is a string to test the encoding with";
        let base58check_str = base58check_encode(input);
        assert_eq!(base58check_str, "1ADezmVSZjqL4pmEshyAxukb4zqdPFpPbG86bqbigKqUrBjmEEuPJg1NAwehX39z");
    }
}