
fn main() {
    // Test the max u64 that can be stored as an f64 is the theoretical value of
    // 2^53 = 9_007_199_254_740_992
    let mut u_64: u64 = 9_007_199_254_000_000;
    loop {
        let f_64 = u_64 as f64;
        if u_64 != f_64 as u64 {
            println!("Failed at u64: {}, incorrecly stored as f64 {}", u_64, f_64);
            println!("Max u64 is: {}", u_64 - 1);
            break;
        }
        u_64 += 1;
    }
}

