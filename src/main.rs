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

        let tokens = parse_command(&input);
        
        match tokens.get(0).map(|s| s.as_str()) {
            Some("exit") => break,
            Some("echo") => {
                if tokens.len() > 1 {
                    println!("{}", tokens[1..].join(" "));
                } else {
                    println!();
                }
            }
            Some("cat") => {
                if tokens.len() > 1 {
                    let mut contents = Vec::new();
                    for file_path in &tokens[1..] {
                        match File::open(file_path) {
                            Ok(file) => {
                                let reader = BufReader::new(file);
                                for line in reader.lines() {
                                    if let Ok(content) = line {
                                        contents.push(content.trim().to_string());
                                    }
                                }
                            }
                            Err(e) => eprintln!("Error opening file {}: {}", file_path, e),
                        }
                    }
                    // Print all contents without spaces between them
                    println!("{}", contents.join(""));
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