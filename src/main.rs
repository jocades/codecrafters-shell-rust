use std::io::{self, BufRead, Write};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let mut stdin = io::stdin().lock();
    loop {
        print!("$ ");
        io::stdout().flush()?;
        let mut buf = String::new();
        stdin.read_line(&mut buf)?;

        match buf.trim() {
            cmd if cmd.starts_with("exit") => {
                let (_, code) = cmd.split_once(" ").context("must include code")?;
                std::process::exit(code.parse()?)
            }
            cmd => {
                println!("{cmd}: command not found");
            }
        };
    }

    Ok(())
}
