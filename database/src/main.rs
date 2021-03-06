fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("Key is required");
    let value = arguments.next().unwrap();
    println!("The key is {} and the value is {}", key, value);
    let contents = format!("{}\t{}\n", key, value);
    std::fs::write("database.rs", contents);
}
