use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process;

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

        // Parse the command and arguments while respecting quotes
        let tokens = parse_command(&input);
        
        match tokens.get(0).map(|s| s.as_str()) {
            Some("exit") => break,
            Some("echo") => {
                // Join all arguments after "echo" with a single space
                if tokens.len() > 1 {
                    println!("{}", tokens[1..].join(" "));
                } else {
                    println!();
                }
            }
            Some("cat") => {
                if tokens.len() > 1 {
                    for file_path in &tokens[1..] {
                        match File::open(file_path) {
                            Ok(file) => {
                                let reader = BufReader::new(file);
                                for line in reader.lines() {
                                    match line {
                                        Ok(content) => print!("{} ", content),
                                        Err(e) => eprintln!("Error reading file: {}", e),
                                    }
                                }
                                println!();
                            }
                            Err(e) => eprintln!("Error opening file {}: {}", file_path, e),
                        }
                    }
                }
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
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\'' if !in_quotes => {
                in_quotes = true;
            }
            '\'' if in_quotes => {
                in_quotes = false;
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            ' ' if !in_quotes => {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            _ => {
                current_token.push(c);
            }
        }
    }

    // Add the last token if there is one
    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    // Filter out empty tokens
    tokens.into_iter().filter(|s| !s.is_empty()).collect()
}