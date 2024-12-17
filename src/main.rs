use std::{
    env,
    io::{self, BufRead, Write},
    path::{Path, PathBuf},
    process::{self, Command},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const BUILTINS: [&str; 4] = ["exit", "echo", "type", "pwd"];

fn find_exe(cmd: &str) -> Result<Option<PathBuf>> {
    Ok(env::var("PATH")?.split(':').find_map(|p| {
        let path = Path::new(p).join(cmd);
        path.is_file().then_some(path)
    }))
}

fn main() -> Result<()> {
    let mut stdin = io::stdin().lock();

    loop {
        print!("$ ");
        io::stdout().flush()?;

        let mut buf = String::new();
        stdin.read_line(&mut buf)?;

        let args: Vec<&str> = buf.split_whitespace().collect();
        match args[0] {
            "exit" => process::exit(args[1].parse()?),
            "echo" => println!("{}", &args[1..].join(" ")),
            "type" => {
                let cmd = args[1];
                if BUILTINS.contains(&cmd) {
                    println!("{cmd} is a shell builtin");
                    continue;
                }
                if let Some(path) = find_exe(cmd)? {
                    println!("{cmd} is {}", path.display());
                } else {
                    println!("{cmd}: not found");
                }
            }
            "pwd" => println!("{}", env::current_dir()?.display()),
            cmd => {
                if let Some(path) = find_exe(cmd)? {
                    Command::new(path).args(&args[1..]).status()?;
                } else {
                    println!("{cmd}: command not found");
                }
            }
        };
    }
}
