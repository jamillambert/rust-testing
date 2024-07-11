fn main() {
    let foo = |s: &mut String| -> String {s.push('.'); s.clone()};
    let mut bar = String::from("Hmm");
    bar = foo(&mut bar.clone());
    bar = foo(&mut bar.clone());
    bar.push('.');
    println!("bar = {}", bar);
}