fn main() {
    use bitcoin::secp256k1::{rand, Secp256k1};
    use bitcoin::{Address, Network, PublicKey};
    // Generate random key pair.
    let s = Secp256k1::new();
    let public_key = PublicKey::new(s.generate_keypair(&mut rand::thread_rng()).1);
    // Generate pay-to-pubkey-hash address.
    let address = Address::p2pkh(&public_key, Network::Bitcoin);
    println!("Address: {} s: {:?}, public_key: {}", address, s, public_key);
}
