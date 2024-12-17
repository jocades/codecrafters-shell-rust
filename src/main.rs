use std::io::{self, BufRead, Write};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let mut stdin = io::stdin().lock();
    loop {
        print!("$ ");
        io::stdout().flush()?;
        let mut buf = String::new();
        stdin.read_line(&mut buf)?;

        let cmd: Vec<&str> = buf.trim().split_whitespace().collect();
        match cmd[0] {
            "exit" => std::process::exit(cmd[1].parse()?),
            "echo" => println!("{}", &cmd[1..].join(" ")),
            other => println!("{other}: command not found"),
        }
    }
}
