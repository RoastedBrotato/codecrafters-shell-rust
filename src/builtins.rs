use std::env;
use std::path::Path;


pub fn echo(args: &[&str]) {
    println!("{}", args.join(" "));
}

pub fn exit(args: &[&str]) {
    let code = args.get(0).and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
    std::process::exit(code);
}

pub fn cmd_type(cmd: &str, _args: &[&str]) {
    // Check if the command is a shell builtin
    if super::BUILD_INS.contains(&cmd) {
        println!("{} is a shell builtin", cmd);
        return;
    }

    // Check if the command is an executable in the PATH
    if let Ok(paths) = env::var("PATH") {
        for path in env::split_paths(&paths) {
            let exe_path = path.join(cmd);
            if exe_path.is_file() && is_executable(&exe_path) {
                println!("{} is {}", cmd, exe_path.display());
                return;
            }
        }
    }

    // If no match is found, print a not found message
    println!("{}: not found", cmd);
}

// Helper function to check if a file is executable
fn is_executable(path: &Path) -> bool {
    path.metadata()
        .map(|m| m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

pub fn pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(err) => eprintln!("pwd: error retrieving current directory: {}", err),
    }
}

pub const BUILD_INS: &[&str] = &["exit", "echo", "type", "pwd"];
