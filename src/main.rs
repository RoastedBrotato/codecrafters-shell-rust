#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();

    loop {
        // Print the prompt
        print!("$ ");
        io::stdout().flush().expect("Failed to flush stdout"); // Ensure the prompt is flushed immediately

        // Read user input
        let mut input = String::new();
        let bytes_read = stdin.read_line(&mut input).expect("Failed to read line");

        // Exit the loop if EOF or no input
        if bytes_read == 0 {
            break;
        }

        let trimmed_input = input.trim();

        // If input is empty, continue to the next iteration
        if trimmed_input.is_empty() {
            continue;
        }

        // Print the "command not found" message
        println!("{}: command not found", trimmed_input);
    }
}
