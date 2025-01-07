#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let path_env = std::env::var("PATH").unwrap();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let argv = input.split_whitespace().collect::<Vec<&str>>();
        if argv.is_empty() {
            continue;
        }

        let builtins = ["exit", "echo", "type"];
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
                } else {
                    // Search for the command in PATH directories
                    let split = path_env.split(':');
                    if let Some(path) = split
                        .find(|&dir| std::fs::metadata(format!("{}/{}", dir, cmd)).is_ok())
                    {
                        println!("{cmd} is {path}/{cmd}");
                    } else {
                        println!("{cmd}: not found");
                    }
                }
            }
            _ => {
                println!("{}: command not found", input.trim());
            }
        }
    }
}
