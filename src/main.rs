// main.rs
mod builtins;

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::os::unix::process::CommandExt; // Import CommandExt trait

// Add the find_exe function that builtins.rs needs
pub fn find_exe(command: &str) -> Option<PathBuf> {
    // If the command contains a path separator, check if it's directly executable
    if command.contains('/') {
        let path = PathBuf::from(command);
        if path.exists() && builtins::is_executable(&path) {
            return Some(path);
        }
        return None;
    }

    // Otherwise search PATH for the command
    if let Ok(path) = env::var("PATH") {
        for dir in path.split(':') {
            let full_path = PathBuf::from(dir).join(command);
            if full_path.exists() && builtins::is_executable(&full_path) {
                return Some(full_path);
            }
        }
    }
    None
}

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
            Some(cmd) => {
                if let Some(path) = find_exe(cmd) {
                    Command::new(&path)
                        .arg0(cmd)  // Use original command name
                        .args(&args)
                        .spawn()?
                        .wait()?;
                } else {
                    eprintln!("{}: command not found", cmd);
                }
            }
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
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\'' => {
                // Read until the closing quote
                while let Some(c) = chars.next() {
                    if c == '\'' {
                        // Check if the next character is also a quote
                        if chars.peek() == Some(&'\'') {
                            chars.next(); // Skip the next quote
                            continue;     // Continue reading without adding the quote
                        } else {
                            // End of quoted section
                            break;
                        }
                    } else {
                        current_token.push(c);
                    }
                }
            }
            ' ' => {
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