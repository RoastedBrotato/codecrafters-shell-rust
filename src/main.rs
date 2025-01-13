mod builtins;
use crate::builtins::{cmd_type, echo, exit, BUILD_INS};
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

/// Find an executable in the PATH environment variable
fn find_exe(name: &str) -> Option<PathBuf> {
    if let Ok(paths) = env::var("PATH") {
        for path in env::split_paths(&paths) {
            let exe_path = path.join(name);
            if exe_path.is_file() {
                return Some(exe_path);
            }
        }
    }
    None
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let cmds: Vec<_> = input.split_whitespace().collect();
        if cmds.is_empty() {
            continue;
        }

        let cmd = cmds[0];
        let args = &cmds[1..];

        if BUILD_INS.contains(&cmd) {
            match cmd {
                "exit" => exit(args),
                "echo" => echo(args),
                "type" => cmd_type(cmd, args),
                _ => unreachable!(),
            };
        } else if let Some(path) = find_exe(cmd) {
            Command::new(path)
                .args(args)
                .status()
                .expect("failed to execute process");
        } else {
            println!("{}: command not found", cmd)
        }
    }
}

// Implementation of `cmd_type`
pub fn cmd_type(cmd: &str, args: &[&str]) {
    if args.len() != 1 {
        println!("type: expected 1 argument, got {}", args.len());
        return;
    }

    let query = args[0];
    let builtins = ["exit", "echo", "type"];

    if builtins.contains(&query) {
        println!("{} is a shell builtin", query);
    } else if let Some(path) = find_exe(query) {
        // Strip directory path for expected output
        if let Some(name) = path.file_name() {
            println!("{} is {}", query, name.to_string_lossy());
        } else {
            println!("{}: not found", query);
        }
    } else {
        println!("{}: not found", query);
    }
}
