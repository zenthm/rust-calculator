use std::io::{self, Write};

fn main() {
    println!("Calculator in Rust");

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("Input: ");
        stdout.flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");
    }
}
