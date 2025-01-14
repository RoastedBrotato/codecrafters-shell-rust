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
    let path = env::var("PATH").map(|x| env::split_paths(&x).collect::<Vec<_>>());
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
                    if commands.len() > 1 {
                        println!("{}", parse_echo_args(&commands[1..]));
                    }
                }
                "type" => {
                    let Some(cmd) = commands.get(1).map(|x| x.as_str()) else {
                        continue;
                    };
                    if BUILTIN.binary_search(&cmd).is_ok() {
                        println!("{cmd} is a shell builtin");
                    } else if let Some(cmd_absolutepath) = find_command_in_paths(cmd, &path) {
                        println!("{cmd} is {cmd_absolutepath}");
                    } else {
                        println!("{cmd}: not found");
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
                    if let Some(command) = find_command_in_paths(command, &path) {
                        let out = Command::new(command)
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
    let (cmd, rest) = input.split_once(" ").unwrap_or((input, ""));
    let mut result = vec![cmd.to_string()];
    let mut rest = rest.trim();
    while !rest.is_empty() {
        match rest.chars().next().unwrap() {
            '\'' => {
                let (arg, r) = rest[1..].split_once('\'')?;
                result.push(arg.to_string());
                rest = r;
            }
            ' ' => {
                rest = rest.trim_start();
            }
            _c => {
                let (arg, r) = rest.split_once(' ').unwrap_or((rest, ""));
                result.push(arg.to_string());
                rest = r;
            }
        }
    }
    Some(result)
    // input
    //     .split_ascii_whitespace()
    //     .map(|x| x.to_string())
    //     .collect::<Vec<_>>()
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
fn find_command_in_paths(cmd: &str, paths: &Result<Vec<PathBuf>, env::VarError>) -> Option<String> {
    paths.as_ref().ok().and_then(|paths| {
        paths.iter().find_map(|path| {
            let path = path.join(cmd);
            path.exists().then(|| path.to_string_lossy().to_string())
        })
    })
}

fn parse_echo_args(args: &[String]) -> String {
    let mut result = String::new();
    let mut in_quotes = false;
    let mut is_first = true;

    for arg in args {
        if arg.starts_with('\'') && arg.ends_with('\'') {
            // Single quoted argument
            if !is_first && !in_quotes {
                result.push(' ');
            }
            result.push_str(&arg[1..arg.len()-1]);
        } else if arg.starts_with('\'') {
            // Start of quoted section
            if !is_first && !in_quotes {
                result.push(' ');
            }
            in_quotes = true;
            result.push_str(&arg[1..]);
        } else if arg.ends_with('\'') {
            // End of quoted section
            in_quotes = false;
            result.push_str(&arg[..arg.len()-1]);
        } else {
            // Normal argument or middle of quoted section
            if !is_first && !in_quotes {
                result.push(' ');
            }
            result.push_str(arg);
        }
        is_first = false;
    }

    result
}