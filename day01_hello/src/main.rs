use std::io;

fn main() {
    println!("Hello...");
    println!("Who are you ?");

    let mut input: String = String::new();

    let stdin = io::stdin();

    stdin.read_line(&mut input)
    .expect("Read fails");

    println!("Hello {}", input)
}
