#[allow(unused_imports)]
use std::io::{self, Write};
fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let trimmed_input = input.trim();
        let command = trimmed_input.split_whitespace().next().unwrap();
        // let args = trimmed_input.split_whitespace().collect::<Vec<&str>>();
        match command {
            "echo" => println!("{}", trimmed_input.replacen("echo ", "", 1)),
            "exit" => break,
            _ => println!("{}: command not found", trimmed_input),
        }
    }
}