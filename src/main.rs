#[allow(unused_imports)]
use std::io::{self, Write};
fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input)?;
        let tokens: Vec<&str> = input.trim().split(' ').filter(|s| !s.is_empty()).collect();
        match tokens[..] {
            ["exit", code] => std::process::exit(code.parse::<u8>().unwrap().into()),
            ["echo", ..] => println!("{}", tokens[1..].join(" ")),
            ["type", cmd] => match cmd {
                "exit" | "echo" | "type" => println!("{cmd} is a shell builtin"),
                _ => println!("{cmd}: not found"),
            },
            _ => println!("{}: command not found", input.trim()),
        }
    }
}