use sha2::{Sha256, Digest};

fn main() {
    let input = "Hello, world!";
    let hashed = hash_string(input);
    println!("Hashed value: {:?}", hashed);
}

fn hash_string(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}", result)
}