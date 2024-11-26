mod encode;

fn main() {
    let input = "This is string to test the encoding with";
    let bech32_str = encode::bech32_encode(input);
    let base58check_str = encode::base58check_encode(input);
    let base58_str_checked = encode::convert_base58_to_base58check(encode::base58_encode(input).as_str());
    println!("Bech32 string: {}", bech32_str);
    println!("Base58Check string: {}", base58check_str);
    println!("Base58Check string from base58: {}", base58_str_checked);
}
