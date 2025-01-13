use std::fs;
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let path_env = std::env::var("PATH").unwrap();

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
            "type" => handle_type_command(&argv[1..], &path_env),
            _ => {
                println!("{}: command not found", argv[0]);
            }
        }
    }
}

/// Handle the `type` built-in command
fn handle_type_command(args: &[&str], path_env: &str) {
    if args.len() != 1 {
        println!("type: expected 1 argument, got {}", args.len());
        return;
    }

    let cmd = args[0];
    let builtins = ["exit", "echo", "type"];

    if builtins.contains(&cmd) {
        println!("{} is a shell builtin", cmd);
    } else if let Some(full_path) = locate_program(path_env, cmd) {
        println!("{} is {}", cmd, full_path);
    } else {
        println!("{}: not found", cmd);
    }
}

/// Locate a program in the directories specified by the PATH environment variable
fn locate_program(path_env: &str, program: &str) -> Option<String> {
    for dir in path_env.split(':') {
        let full_path = format!("{}/{}", dir, program);
        if fs::metadata(&full_path).is_ok() {
            return Some(full_path);
        }
    }
    None
}
