use lazy_static::lazy_static;
use std::collections::HashMap;
use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
fn not_found(cmd: &[&str]) {
    println!("{}: command not found", cmd.first().unwrap())
}
fn exit(args: &[&str]) {
    let code = args.first().map(|code| code.parse::<i32>().unwrap());
    match code {
        Some(code) => std::process::exit(code),
        None => println!("Missing argument for exit"),
    };
}
fn echo(args: &[&str]) {
    println!("{}", args.join(" "));
}
fn type_(args: &[&str]) {
    match BUILTIN_HANDLERS.get(args.first().unwrap()) {
        Some(_) => println!("{} is a shell builtin", args.first().unwrap()),
        _ => println!("{} not found", args.first().unwrap()),
        _ => match env::var("PATH")
            .unwrap()
            .split(":")
            .map(|path| format!("{}/{}", path, args.first().unwrap()))
            .find(|path| std::fs::metadata(path).is_ok())
        {
            Some(path) => println!("{} is {}", args.first().unwrap(), path),
            _ => println!("{} not found", args.first().unwrap()),
        },
    }
}
type BuiltinHandler = fn(&[&str]);
lazy_static! {
    static ref BUILTIN_HANDLERS: HashMap<&'static str, BuiltinHandler> = HashMap::from([
        ("exit", exit as BuiltinHandler),
        ("echo", echo as BuiltinHandler),
        ("type", type_ as BuiltinHandler),
    ]);
}
fn main() {
    let stdin = io::stdin();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let prompt_parts: Vec<_> = input.trim().split(" ").collect();
        let [cmd, args@ ..] = prompt_parts.as_slice() else { continue };
        match BUILTIN_HANDLERS.get(*cmd) {
            Some(handler) => handler(args),
            _ => not_found(&[cmd]),
        }
    }
}