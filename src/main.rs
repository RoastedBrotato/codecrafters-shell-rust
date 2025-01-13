use std::io::{self, Write};
use std::process::Command;

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
            _ => {
                let program = argv[0];
                let args = &argv[1..];

                // Locate the program using PATH
                let program_path = locate_program(&path_env, program);

                match program_path {
                    Some(full_path) => {
                        // Execute the program
                        match Command::new(full_path).args(args).output() {
                            Ok(output) => {
                                // Print stdout
                                if !output.stdout.is_empty() {
                                    print!("{}", String::from_utf8_lossy(&output.stdout));
                                }
                                // Print stderr
                                if !output.stderr.is_empty() {
                                    eprint!("{}", String::from_utf8_lossy(&output.stderr));
                                }
                            }
                            Err(err) => {
                                // Error while executing the program
                                eprintln!("{}: failed to execute: {}", program, err);
                            }
                        }
                    }
                    None => {
                        // Program not found
                        eprintln!("{}: command not found", program);
                    }
                }
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
