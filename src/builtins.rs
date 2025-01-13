pub const BUILD_INS: [&str; 3] = ["exit", "echo", "type"];

pub fn exit(_args: &[&str]) {
    std::process::exit(0);
}

pub fn echo(args: &[&str]) {
    let output = args.join(" ");
    println!("{}", output);
}

pub fn cmd_type(_cmd: &str, args: &[&str]) {
    if args.len() != 1 {
        println!("type: expected 1 argument, got {}", args.len());
        return;
    }

    let query = args[0];
    let builtins = ["exit", "echo", "type"];

    if builtins.contains(&query) {
        println!("{} is a shell builtin", query);
    } else if let Some(path) = super::find_exe(query) {
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
