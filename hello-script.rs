use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut first_arg = String::new();

    if args.len() > 1 {
        first_arg = args[1].clone();
    }

    println!("Hello, {}", first_arg);
}
