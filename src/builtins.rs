use std::env;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;


pub fn echo(args: &[&str]) {
    println!("{}", args.join(" "));
}


pub fn exit(args: &[&str]) {
    let code = args.get(0).and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
    std::process::exit(code);
}

pub fn find_exe(name: &str) -> Option<PathBuf> {
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

pub fn cmd_type(_cmd: &str, args: &[&str]) {
    if args.is_empty() {
        eprintln!("type: missing operand");
        return;
    }

    let command = args[0];

    // Check if it's a shell builtin
    if BUILD_INS.contains(&command) {
        println!("{} is a shell builtin", command);
    }
    // Check if it's an executable in the PATH
    else if let Some(path) = find_exe(command) {
        println!("{} is {}", command, path.display());
    } else {
        println!("{}: not found", command);
    }
}

pub fn is_executable(path: &Path) -> bool {
    path.metadata()
        .map(|m| m.permissions().mode() & 0o111 != 0) // Check if the file is executable
        .unwrap_or(false)
}

pub fn pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(err) => eprintln!("pwd: error retrieving current directory: {}", err),
    }
}

pub const BUILD_INS: &[&str] = &["exit", "echo", "type", "pwd"];
