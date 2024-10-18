use bech32::{self, ToBase32, Variant};
use sha2::{Sha256, Digest};
use bs58;

fn main() {
    let input = "This is string to test the encoding with";

    let bech32_str = bech32_encode(input);
    let base58check_str = base58check_encode(input);
    let base58_str_checked = convert_base58_to_base58check(base58_encode(input).as_str());

    println!("Bech32 string: {}", bech32_str);
    println!("Base58Check string: {}", base58check_str);
    println!("Base58Check string from base58: {}", base58_str_checked);

}

fn bech32_encode(input: &str) -> String {
    let bytes = input.as_bytes();
    bech32::encode("bc", bytes.to_base32(), Variant::Bech32m).unwrap()
}

fn base58_encode(input: &str) -> String {
    bs58::encode(input.as_bytes()).into_string()
}

fn base58check_encode(input: &str) -> String {
    let mut bytes = vec![0];  // Start with 1 (in Base58 it's represented by leading zeros)
    bytes.extend_from_slice(input.as_bytes());

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

fn convert_base58_to_base58check(input: &str) -> String {
    // Decode the Base58 string to bytes
    let mut bytes = bs58::decode(input).into_vec().unwrap();

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