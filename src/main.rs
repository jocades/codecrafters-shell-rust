use std::{
    env, fs,
    io::{self, BufRead, Write},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const BUILTINS: [&str; 3] = ["exit", "echo", "type"];

fn main() -> Result<()> {
    let mut stdin = io::stdin().lock();
    let mut buf = String::new();

    loop {
        print!("$ ");
        io::stdout().flush()?;
        stdin.read_line(&mut buf)?;

        let cmd: Vec<&str> = buf.split_whitespace().collect();
        match cmd[0] {
            "exit" => std::process::exit(cmd[1].parse()?),
            "echo" => println!("{}", &cmd[1..].join(" ")),
            "type" => {
                if BUILTINS.iter().any(|&b| b == cmd[1]) {
                    println!("{} is a shell builtin", cmd[1]);
                } else {
                    let mut ok = false;
                    for path in env::var("PATH")?.split(':') {
                        for entry in fs::read_dir(path)?.filter_map(|e| e.ok()) {
                            if entry.file_name() == cmd[1] {
                                println!("{} is {}", cmd[1], entry.path().display());
                                ok = true;
                                break;
                            }
                        }
                        if ok {
                            break;
                        }
                    }
                    if !ok {
                        println!("{}: not found", cmd[1]);
                    }
                }
            }
            other => println!("{other}: command not found"),
        };

        buf.clear();
    }
}
