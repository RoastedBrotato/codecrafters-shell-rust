use std::env;
use std::io::{self, Write};
use std::process::{Command, exit};

fn echo(args: &[&str]) {
    let args_with_quotes: Vec<String> = args
        .iter()
        .map(|arg| {
            // Check if the argument starts and ends with single quotes
            if arg.starts_with("'") && arg.ends_with("'") && arg.len() > 1 {
                arg[1..arg.len()-1].to_string() // Remove the surrounding single quotes
            } else {
                arg.to_string()
            }
        })
        .collect();
        
    println!("{}", args_with_quotes.join(" "));
}

fn cat(args: &[&str]) {
    // Handle single quotes similarly for cat command
    for arg in args {
        let arg_without_quotes = if arg.starts_with("'") && arg.ends_with("'") {
            &arg[1..arg.len()-1] // Remove the single quotes from file names
        } else {
            arg
        };
        
        // Simulate reading the file (in reality, you would open and read the file here)
        println!("Reading file: {}", arg_without_quotes);
        // You can add actual file reading code here, for example:
        // let content = std::fs::read_to_string(arg_without_quotes).unwrap();
        // println!("{}", content);
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let cmds: Vec<_> = input.split_whitespace().collect();
        if cmds.is_empty() {
            continue;
        }

        let cmd = cmds[0];
        let args = &cmds[1..];

        match cmd {
            "echo" => echo(args),
            "cat" => cat(args),
            _ => println!("Command not found: {}", cmd),
        }
    }
}
