use std::io::{self, BufRead, Write};

use anyhow::Result;

fn main() -> Result<()> {
    print!("$ ");
    io::stdout().flush()?;

    let mut stdin = io::stdin().lock();
    let mut buf = String::new();
    stdin.read_line(&mut buf)?;

    match buf.trim() {
        cmd => println!("{cmd}: command not found"),
    };

    Ok(())
}
