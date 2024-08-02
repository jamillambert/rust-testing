use sha2::{Digest, Sha256};

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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_string() {
        let input = "Hello, world!";
        let expected_output = "2ef7bde608ce5404e97d5f042f95f89f1c232871";
        let hashed = hash_string(input);
        assert_eq!(hashed, expected_output);
    }

    #[test]
    fn test_hash_string_empty_input() {
        let input = "";
        let expected_output = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        let hashed = hash_string(input);
        assert_eq!(hashed, expected_output);
    }

    #[test]
    fn test_hash_string_long_input() {
        let input = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed non risus. Suspendisse lectus tortor, dignissim sit amet, adipiscing nec, ultricies sed, dolor.";
        let expected_output = "f7e1a8e5e8f5e4f5e8f5e4f5e8f5e4f5e8f5e4f5e8f5e4f5e8f5e4f5e8f5e4f5";
        let hashed = hash_string(input);
        assert_eq!(hashed, expected_output);
    }
}
