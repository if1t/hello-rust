use std::env;

fn main() {
    let name = env::args().nth(1).unwrap_or("stranger".into());
    println!("Hello, {name}");
}
