use bitcoin::absolute::LockTime;
use bitcoin::blockdata::script::Script;
use bitcoin::blockdata::transaction::{OutPoint, Sequence, Transaction, TxIn, TxOut, Version};
use bitcoin::opcodes::all::*;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::{Address, Amount, Network, Opcode, PublicKey, Txid, Witness};
use bitcoin::consensus::deserialize;
use bitcoin_hashes::sha256d::Hash;

/// Creates a Bitcoin transaction.
///
/// This function is a template for creating a new Bitcoin transaction by adding inputs and outputs
/// to the transaction structure.
fn create_bitcoin_transaction(recipient_address: &Address) -> Transaction {
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

    let input = TxIn {
        previous_output: prev_outpoint,
        script_sig: Script::new().into(),
        sequence: Sequence(0xFFFFFFFF),
        witness: Witness::default(),
    };
    transaction.input.push(input);

    let output = TxOut {
        value: Amount::from_btc(0.1).unwrap(),
        script_pubkey: recipient_address.script_pubkey(),
    };
    transaction.output.push(output);

    // Set the script signature
    let script_sig = Script::new();
    transaction.input[0].script_sig = script_sig.into();

    transaction
}

/// Verifies a Bitcoin transaction.
///
/// This function is a template for verifying a Bitcoin transaction by checking the input scripts
/// against the output scripts.
fn verify_transaction(transaction: &Transaction, prev_tx: &Transaction) -> bool {
    // Verify the transaction
    let mut input_scripts = vec![];
    for input in &transaction.input {
        let _prev_txid = input.previous_output.txid;
        let prev_vout = input.previous_output.vout;
        let prev_output = prev_tx.output[prev_vout as usize].clone();
        let script_pubkey = prev_output.script_pubkey.clone();
        input_scripts.push(script_pubkey);
    }

    for (input, _script_pubkey) in transaction.input.iter().zip(input_scripts) {
        let script_sig = input.script_sig.clone();
        //let witness = input.witness.clone();
        let script = script_sig.to_bytes();
        //script.extend(witness.iter().flat_map(|w| w.to_bytes()));
        let mut stack = vec![];
        let mut pc = 0;
        while pc < script.len() {
            let opcode: Opcode = Opcode::from(script[pc]);
            pc += 1;
            match opcode {
                OP_PUSHDATA1 => {
                    let len = script[pc] as usize;
                    pc += 1;
                    stack.push(script[pc..pc + len].to_vec());
                    pc += len;
                }
                OP_PUSHDATA2 => {
                    let len = u16::from_le_bytes([script[pc], script[pc + 1]]) as usize;
                    pc += 2;
                    stack.push(script[pc..pc + len].to_vec());
                    pc += len;
                }
                OP_PUSHDATA4 => {
                    let len = u32::from_le_bytes([
                        script[pc],
                        script[pc + 1],
                        script[pc + 2],
                        script[pc + 3],
                    ]) as usize;
                    pc += 4;
                    stack.push(script[pc..pc + len].to_vec());
                    pc += len;
                }
                OP_DUP => {
                    if let Some(item) = stack.last().cloned() {
                        stack.push(item);
                    }
                }
                OP_EQUALVERIFY => {
                    if let (Some(item1), Some(item2)) = (stack.pop(), stack.pop()) {
                        if item1 == item2 {
                            continue;
                        } else {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => {
                    return false;
                }
            }
        }
    }

    true
}

fn sign_transaction(transaction: &Transaction) {
    let mut input_scripts = vec![];
    for input in &transaction.input {
        let _prev_txid = input.previous_output.txid;
        let prev_vout = input.previous_output.vout;
        let prev_output = transaction.output[prev_vout as usize].clone();
        let script_pubkey = prev_output.script_pubkey.clone();
        input_scripts.push(script_pubkey);
    }

    for (input, _script_pubkey) in transaction.input.iter().zip(input_scripts) {
        let script_sig = input.script_sig.clone();
        let script = script_sig.to_bytes();
        let mut stack = vec![];
        let mut pc = 0;
        while pc < script.len() {
            let opcode: Opcode = Opcode::from(script[pc]);
            pc += 1;
            match opcode {
                OP_PUSHDATA1 => {
                    let len = script[pc] as usize;
                    pc += 1;
                    stack.push(script[pc..pc + len].to_vec());
                    pc += len;
                }
                OP_PUSHDATA2 => {
                    let len = u16::from_le_bytes([script[pc], script[pc + 1]]) as usize;
                    pc += 2;
                    stack.push(script[pc..pc + len].to_vec());
                    pc += len;
                }
                OP_PUSHDATA4 => {
                    let len = u32::from_le_bytes([
                        script[pc],
                        script[pc + 1],
                        script[pc + 2],
                        script[pc + 3],
                    ]) as usize;
                    pc += 4;
                    stack.push(script[pc..pc + len].to_vec());
                    pc += len;
                }
                OP_DUP => {
                    if let Some(item) = stack.last().cloned() {
                        stack.push(item);
                    }
                }
                OP_EQUALVERIFY => {
                    if let (Some(item1), Some(item2)) = (stack.pop(), stack.pop()) {
                        if item1 == item2 {
                            continue;
                        } else {
                            return;
                        }
                    } else {
                        return;
                    }
                }
                _ => {
                    return;
                }
            }
        }
    }
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
    let transaction = create_bitcoin_transaction(&address);
    sign_transaction(&transaction);
    let prev_tx: Transaction = deserialize(&hex::decode("0200000000010166c3d39490dc827a2594c7b17b7d37445e1f4b372179649cd2ce4475e3641bbb0100000017160014e69aa750e9bff1aca1e32e57328b641b611fc817fdffffff01e87c5d010000000017a914f3890da1b99e44cd3d52f7bcea6a1351658ea7be87024830450221009eb97597953dc288de30060ba02d4e91b2bde1af2ecf679c7f5ab5989549aa8002202a98f8c3bd1a5a31c0d72950dd6e2e3870c6c5819a6c3db740e91ebbbc5ef4800121023f3d3b8e74b807e32217dea2c75c8d0bd46b8665b3a2d9b3cb310959de52a09bc9d20700").unwrap()).unwrap();
    let is_valid = verify_transaction(&transaction, &prev_tx);
    println!("Transaction is valid: {}", is_valid);
}
