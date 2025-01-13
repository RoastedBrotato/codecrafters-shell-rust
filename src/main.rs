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
                // Attempt to locate and execute the command
                if let Some(full_path) = locate_program(&path_env, argv[0]) {
                    execute_program(&full_path, argv[0], &argv[1..]);
                } else {
                    println!("{}: command not found", argv[0]);
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

/// Execute the program with its arguments
fn execute_program(program_path: &str, program_name: &str, args: &[&str]) {
    let output = Command::new(program_path)
        .args(args)
        .output()
        .expect("Failed to execute program");

    // Print program output, modifying Arg #0 to show only the program name
    let stdout = String::from_utf8_lossy(&output.stdout);
    let modified_output = stdout
        .replace(program_path, program_name); // Replace full path with program name
    print!("{}", modified_output);
}
