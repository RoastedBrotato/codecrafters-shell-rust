use std::env;

pub fn echo(args: &[&str]) {
    println!("{}", args.join(" "));
}

pub fn exit(args: &[&str]) {
    let code = args.get(0).and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
    std::process::exit(code);
}

pub fn cmd_type(_cmd: &str, _args: &[&str]) {
    println!("type command not fully implemented");
}

pub fn pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(err) => eprintln!("pwd: error retrieving current directory: {}", err),
    }
}

pub const BUILD_INS: &[&str] = &["exit", "echo", "type", "pwd"];
