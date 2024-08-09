use bitcoin::absolute::LockTime;
use bitcoin::blockdata::script::Script;
use bitcoin::blockdata::transaction::{OutPoint, Transaction, TxIn, TxOut, Version};
use bitcoin::secp256k1::{rand, Secp256k1};
use bitcoin::{Address, Amount, Network, PublicKey, Txid, Witness};
use bitcoin_hashes::sha256d::Hash;
use std::str::FromStr;

/// Creates a Bitcoin transaction.
///
/// This function creates a new Bitcoin transaction by adding inputs and outputs to the transaction
/// structure. It generates a random key pair, creates a pay-to-pubkey-hash address, and sets the
/// script signature.
/// Finally, it prints the transaction.
fn create_bitcoin_transaction() {
    // Create a new transaction
    let mut transaction = Transaction {
        version: Version(1),
        lock_time: LockTime::from_time(1653195600).expect("valid time"),
        input: vec![],
        output: vec![],
    };

    // Add inputs to the transaction
    let prev_txid = [
        0x1b, 0x3f, 0x4d, 0x6f, 0x7f, 0x8f, 0x9f, 0xaf, 0xbf, 0xcf, 0xdf, 0xef, 0x0f, 0x1f, 0x2f,
        0x3f, 0x4f, 0x5f, 0x6f, 0x7f, 0x8f, 0x9f, 0xaf, 0xbf, 0xcf, 0xdf, 0xef, 0x0f, 0x1f, 0x2f,
        0x3f, 0x4f,
    ];
    let prev_txid = Hash::from_bytes_ref(&prev_txid);
    let prev_outpoint = OutPoint {
        txid: Txid::from_raw_hash(*prev_txid),
        vout: 0,
    };
    use bitcoin::blockdata::transaction::Sequence;

    let input = TxIn {
        previous_output: prev_outpoint,
        script_sig: Script::new().into(),
        sequence: Sequence(0xFFFFFFFF),
        witness: Witness::default(),
    };
    transaction.input.push(input);

    // Add outputs to the transaction
    let recipient_address: Address = Address::from_str("32iVBEu4dxkUQk9dJbZUiBiQdmypcEyJRf")
        .unwrap()
        .require_network(Network::Bitcoin)
        .unwrap();
    let output = TxOut {
        value: Amount::from_btc(0.1).unwrap(),
        script_pubkey: recipient_address.script_pubkey(),
    };
    transaction.output.push(output);

    // Set the script signature
    let script_sig = Script::new();
    transaction.input[0].script_sig = script_sig.into();

    // Print the transaction
    println!("Bitcoin Transaction: {:?}", transaction);
}

fn main() {
    // Generate random key pair.
    let s = Secp256k1::new();
    let public_key = PublicKey::new(s.generate_keypair(&mut rand::thread_rng()).1);
    // Generate pay-to-pubkey-hash address.
    let address = Address::p2pkh(&public_key, Network::Bitcoin);
    println!(
        "Address: {} s: {:?}, public_key: {}",
        address, s, public_key
    );
    create_bitcoin_transaction();
}
