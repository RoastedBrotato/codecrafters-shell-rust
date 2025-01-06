#[allow(unused_imports)]
use std::io::{self, Write};
fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
        // Wait for user input
        stdin.read_line(&mut input).unwrap();
        match input.trim() {
            "exit 0" => break,
            &_ => {
                print!("{}: not found\n", input.trim());
            }
        }
    }
}
