#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let path_env = std::env::var("PATH").unwrap();

    let builtins = ["exit", "echo", "type"];

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let argv: Vec<&str> = input.trim().split_whitespace().collect();
        if argv.is_empty() {
            continue;
        }

        match argv[0] {
            "exit" => break,
            "echo" => {
                println!("{}", argv[1..].join(" "));
            }
            "type" => {
                if argv.len() != 2 {
                    println!("type: expected 1 argument, got {}", argv.len() - 1);
                    continue;
                }

                let cmd = argv[1];

                if builtins.contains(&cmd) {
                    println!("{} is a shell builtin", cmd);
                } else if let Some(path) = locate_program(&path_env, cmd) {
                    println!("{} is {}", cmd, path);
                } else {
                    println!("{} not found", cmd);
                }
            }
            _ => {
                println!("{}: command not found", argv[0]);
            }
        }
    }
}

/// Locate a program in the directories specified by the PATH environment variable
fn locate_program(path_env: &str, program: &str) -> Option<String> {
    for dir in path_env.split(':') {
        let full_path = format!("{}/{}", dir, program);
        if std::fs::metadata(&full_path).is_ok() {
            return Some(full_path);
        }
    }
    None
}
