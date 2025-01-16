mod builtins;

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process;
use std::env;
use std::path::Path;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    
    print!("$ ");
    stdout.flush()?;

    for line in stdin.lock().lines() {
        let input = line?;
        if input.is_empty() {
            print!("$ ");
            stdout.flush()?;
            continue;
        }

        let tokens = parse_command(&input);
        let command = tokens.get(0).map(String::as_str);
        let args: Vec<&str> = tokens.iter().skip(1).map(String::as_str).collect();
        
        match command {
            Some("exit") => builtins::exit(&args),
            Some("echo") => builtins::echo(&args),
            Some("cd") => builtins::cd(&args),
            Some("pwd") => builtins::pwd(),
            Some("type") => builtins::cmd_type(command.unwrap_or(""), &args),
            Some("cat") => {
                // ...existing code...
            }
            Some(cmd) => eprintln!("Unknown command: {}", cmd),
            None => {}
        }

        print!("$ ");
        stdout.flush()?;
    }

    Ok(())
}

fn parse_command(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_quotes = false;

    for c in input.chars() {
        match c {
            '\'' => {
                in_quotes = !in_quotes;
                current_token.push(c);
            }
            ' ' if !in_quotes => {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            _ => current_token.push(c),
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens.into_iter().filter(|s| !s.is_empty()).collect()
}