use std::env::{self};
#[allow(unused_imports)]
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{self, Command};
fn main() -> ! {
    let stdin = io::stdin();
    let path_env = env::var("PATH").unwrap_or(String::new());
    let builtins = ["type", "pwd", "cd", "echo", "exit"];
    let mut input = String::new();
    loop {
        // Create stdout and stderr file handles
        let mut stdout_file: Box<dyn Write> = Box::new(io::stdout());
        let mut stderr_file: Box<dyn Write> = Box::new(io::stderr());
        tprint("$ ", &mut stdout_file);
        // Wait for user input
        input.clear();
        stdin.read_line(&mut input).unwrap();
        // Remove trailing newline
        input = input.trim_end().to_string();
        // Handle empty input
        if input.is_empty() {
            continue;
        }
        // Parse input into tokens
        let tokens_owned = parse_tokens(input.as_str());
        let mut tokens: Vec<&str> = tokens_owned.iter().map(|s| s.as_str()).collect();
        // Handle redirection
        match handle_redir(&mut tokens, &mut stdout_file, &mut &mut stderr_file) {
            Ok(()) => {}
            Err(err) => {
                tprintln(
                    &format!("Failed to handle redirection: {}", err),
                    &mut &mut stderr_file,
                );
                continue;
            }
        }
        // Match tokens to commands
        match tokens.as_slice() {
            &["type", cmd] => {
                // Check if the command is a built-in
                if builtins.contains(&cmd) {
                    tprintln(&format!("{} is a shell builtin", cmd), &mut stdout_file);
                    continue;
                }
                // Check if the command is in the PATH
                let (found, found_path) = find_cmd_path(&path_env, cmd);
                if found {
                    tprintln(&format!("{} is {}", cmd, found_path), &mut stdout_file);
                } else {
                    tprintln(&format!("{}: not found", cmd), &mut stdout_file);
                }
            }
            &["cd", dir] => {
                let mut new_dir = dir;
                let mut new_path = PathBuf::new();
                // Handle HOME directory
                if dir.starts_with("~") {
                    let home_dir = env::var("HOME").unwrap();
                    new_path.push(home_dir);
                    new_dir = &new_dir[1..];
                }
                new_path.push(new_dir);
                if new_path.exists() {
                    env::set_current_dir(&new_path).unwrap();
                } else {
                    tprintln(
                        &format!("cd: {}: No such file or directory", new_dir),
                        &mut stdout_file,
                    );
                }
            }
            &["pwd"] => {
                let current_dir = env::current_dir().unwrap();
                tprintln(current_dir.to_str().unwrap(), &mut stdout_file);
            }
            &["echo", ..] => tprintln(&format!("{}", tokens[1..].join(" ")), &mut stdout_file),
            &["exit", code] => {
                let exit_code = code.parse().unwrap_or(0);
                process::exit(exit_code);
            }
            _ => {
                // Check if the command exists in the PATH
                let cmd = tokens[0];
                let (found, _found_path) = find_cmd_path(&path_env, cmd);
                if found {
                    let output = Command::new(cmd)
                        .args(&tokens[1..])
                        .output()
                        .expect("failed to execute command");
                    // Directly write stdout and stderr to their respective targets
                    tprint(&String::from_utf8_lossy(&output.stdout), &mut stdout_file);
                    tprint(&String::from_utf8_lossy(&output.stderr), &mut stderr_file);
                } else {
                    tprintln(
                        &format!("{}: command not found", tokens[0]),
                        &mut stdout_file,
                    );
                }
            }
        }
    }
}
fn handle_redir<'a>(
    tokens: &mut Vec<&str>,
    stdout_file: &mut Box<dyn Write>,
    stderr_file: &mut Box<dyn Write>,
) -> Result<(), &'a str> {
    for i in 0..tokens.len() {
        if tokens[i] == ">" || tokens[i] == "1>" || tokens[i] == "2>" || tokens[i] == ">>" || tokens[i] == "1>>" || tokens[i] == "2>>" {
            // Ensure the redirection file is specified
            if i + 1 < tokens.len() {
                let redir_path = Path::new(tokens[i + 1]);
                // Ensure the parent directory exists
                if let Some(parent) = redir_path.parent() {
                    if !parent.exists() {
                        return Err("Redirection file parent directory does not exist");
                    }
                }
                let redir_file = if tokens[i].ends_with(">>") {
                    File::options().append(true).create(true).open(redir_path).map_err(|_| "Failed to open redirection file")?
                } else {
                    File::create(redir_path).map_err(|_| "Failed to create redirection file")?
                };
                if tokens[i] == ">" || tokens[i] == "1>" || tokens[i] == ">>" || tokens[i] == "1>>" {
                    // Redirect stdout
                    *stdout_file = Box::new(redir_file);
                } else if tokens[i] == "2>" || tokens[i] == "2>>" {
                    // Redirect stderr
                    *stderr_file = Box::new(redir_file);
                }
                // Remove redirection tokens and the file name
                tokens.drain(i..=i + 1);
                break;
            } else {
                return Err("Missing redirection file");
            }
        }
    }
    Ok(())
}
fn parse_tokens(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut chars = input.chars().peekable();
    let mut in_escape = false;
    let mut quote_char = '\0';
    while let Some(ch) = chars.next() {
        // Handle escape characters
        if in_escape {
            in_escape = false;
            current_token.push(ch);
            continue;
        }
        match ch {
            '\\' => {
                in_escape = true;
                // Add the last token if anye;
                // Keep escape characters within quotes
                if quote_char == '\'' {
                    current_token.push(ch);
                } else if quote_char == '"' {
                    // Handle escape characters within double quotes
                    let next_ch = chars.peek();
                    if next_ch == Some(&&'\\') || next_ch == Some(&&'$') || next_ch == Some(&&'"') {
                        // Omit the escape character
                    } else {
                        current_token.push(ch);
                    }
                }
            }
            '\'' | '"' => {
                // Handle quoted strings
                if quote_char == '\0' {
                    quote_char = ch; // Start a quoted string
                } else if ch == quote_char {
                    quote_char = '\0'; // Close quote
                } else {
                    current_token.push(ch); // Part of the quoted string
                }
            }
            ' ' => {
                // Handle token separation
                if quote_char != '\0' {
                    current_token.push(ch); // Space within quotes
                } else if !current_token.is_empty() {
                    tokens.push(current_token);
                    current_token = String::new();
                }
            }
            _ => {
                // Regular characters
                current_token.push(ch);
            }
        };
    }
    // Add the last token if any
    if !current_token.is_empty() {
        tokens.push(current_token);
    }
    tokens
}
fn find_cmd_path(path_env: &String, cmd: &str) -> (bool, String) {
    let mut found = false;
    let mut found_path = String::new();
    // Check if the command is in the PATH
    for dir in path_env.split(":") {
        let path = Path::new(dir).join(cmd);
        if path.exists() {
            found = true;
            found_path = path.to_str().unwrap().to_string();
            break;
        }
    }
    (found, found_path)
}
fn tprint(message: &str, output: &mut Box<dyn Write>) {
    write!(output, "{}", message).unwrap();
    output.flush().unwrap();
}
fn tprintln(message: &str, output: &mut Box<dyn Write>) {
    writeln!(output, "{}", message).unwrap();
    output.flush().unwrap();
}