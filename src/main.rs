use core::str;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    env,
    path::{self, Path, PathBuf},
    process::{exit, Command},
    str::FromStr,
    sync::LazyLock,
};
static BUILTIN: LazyLock<Vec<&str>> = LazyLock::new(|| {
    #[rustfmt::skip]
    let mut v = vec![
        "cd",
        "echo",
        "exit",
        "pwd",
        "type",
    ];
    v.sort_unstable();
    v
});
fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();
    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    let path_var = env::var("PATH").ok();
    let paths = path_var.as_ref().map(|p| env::split_paths(p).collect::<Vec<_>>());
    let Ok(mut current_dir) = env::current_dir() else {
        println!("current directory does not exist");
        exit(-1);
    };
    while stdin.read_line(&mut input).is_ok() {
        let commands: Vec<String> = parse_input(&input).expect("commmand parse error");
        if let Some(command) = commands.first() {
            match command.as_str() {
                "exit" => {
                    if commands.get(1).map_or(false, |x| (*x == "0")) {
                        break;
                    } else {
                        todo!()
                    }
                }
                "echo" => {
                    println!("{}", commands[1..].join(" "));
                }
                "type" => {
                    let Some(cmd) = commands.get(1).map(|x| x.as_str()) else {
                        continue;
                    };
                    if BUILTIN.binary_search(&cmd).is_ok() {
                        println!("{cmd} is a shell builtin");
                    } else if let Some(cmd_absolutepath) = find_command_in_paths(cmd, &paths) {
                        println!("{cmd} is {cmd_absolutepath}");
                    }
                }
                "pwd" => {
                    let Ok(path) = path::absolute(&current_dir) else {
                        println!("error: pwd is empty");
                        continue;
                    };
                    println!("{}", path.to_str().unwrap());
                }
                "cd" => {
                    let Some(target) = commands.get(1) else {
                        println!("USAGE: cd TARGET");
                        continue;
                    };
                    let target = resolve_relative_path(target, &current_dir);
                    if target.exists() {
                        current_dir = target;
                    } else {
                        println!(
                            "cd: {}: No such file or directory",
                            target.to_string_lossy()
                        );
                    }
                }
                _ => {
                    let Some(command) = commands.first() else {
                        continue;
                    };
                    if let Some(program_name) = find_command_in_paths(command, &path) {
                        let full_path = paths.as_ref().ok().and_then(|paths| {
                            paths.iter().find_map(|path| {
                                let path = path.join(&program_name);
                                path.exists().then(|| path)
                            })
                        }).unwrap();
                
                        let out = Command::new(full_path)
                            .current_dir(&current_dir)
                            .args(&commands[1..])
                            .output()
                            .expect("failed to execute process");
                        io::stdout().write_all(&out.stdout).unwrap();
                    } else {
                        println!("{}: command not found", input.trim());
                    }
                }
            }
        }
        print!("$ ");
        io::stdout().flush().unwrap();
        input.clear();
    }
}
fn parse_input(input: &String) -> Option<Vec<String>> {
    let input = input.trim();
    if input.is_empty() {
        return Some(vec![]);
    }

    let mut result = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\'' => {
                if in_quotes {
                    // End of quoted section
                    in_quotes = false;
                    // Check if next char is another quote
                    if chars.peek() == Some(&'\'') {
                        in_quotes = true;
                        chars.next(); // consume the next quote
                    }
                } else {
                    // Start of quoted section
                    in_quotes = true;
                }
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    result.push(current);
                    current = String::new();
                }
            }
            _ => {
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        result.push(current);
    }

    Some(result)
}
fn resolve_relative_path(target: &str, current_dir: &Path) -> PathBuf {
    let mut path: PathBuf = PathBuf::new();
    if !target.starts_with('/') {
        path = current_dir.to_path_buf();
    } else {
        path.push("/");
    }
    for dir in target.split('/') {
        match dir {
            ".." => {
                path.pop();
            }
            "." => {}
            "~" => {
                let dir = env::var("HOME").unwrap();
                path = PathBuf::from_str(&dir).unwrap();
            }
            dir if !dir.is_empty() => {
                path.push(dir);
            }
            _ => (),
        }
    }
    path
}
fn find_command_in_paths(cmd: &str, paths: &Option<Vec<PathBuf>>) -> Option<String> {
    let paths = paths.as_ref()?;
    
    for path in paths {
        let full_path = path.join(cmd);
        if full_path.is_file() {
            return full_path.to_str().map(String::from);
        }
    }
    None
}