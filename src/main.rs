use std::io;

fn main() {
    println!("Calculator in Rust");

    loop {
        println!("Input: ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }
}
