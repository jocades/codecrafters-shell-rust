use std::io::{self, BufRead, Write};

use anyhow::Result;

const BUILTINS: [&str; 3] = ["exit", "echo", "type"];

fn main() -> Result<()> {
    let mut stdin = io::stdin().lock();
    loop {
        print!("$ ");
        io::stdout().flush()?;
        let mut buf = String::new();
        stdin.read_line(&mut buf)?;

        let cmd: Vec<&str> = buf.split_whitespace().collect();
        match cmd[0] {
            "exit" => std::process::exit(cmd[1].parse()?),
            "echo" => println!("{}", &cmd[1..].join(" ")),
            "type" => {
                if BUILTINS.iter().any(|&b| b == cmd[1]) {
                    println!("{} is a shell builtin", cmd[1]);
                } else {
                    println!("{}: not found", cmd[1]);
                }
            }
            other => println!("{other}: command not found"),
        }
    }
}
