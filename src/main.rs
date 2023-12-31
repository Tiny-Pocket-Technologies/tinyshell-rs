use std::{
    env,
    io::{stdin, stdout, Write},
    path::Path,
    process::Command,
};

fn main() {
    loop {
        // Shell character
        print!("tiny> ");
        let _ = stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // Everything after first whitespace
        // is argument to entered command.
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                // default to '/' as new directory if one was not provided.
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{e}");
                }
            }
            "exit" => return,
            command => {
                let child = Command::new(command).args(args).spawn();

                // gracefully handle malformed user input
                match child {
                    Ok(mut child) => {
                        let _ = child.wait();
                    }
                    Err(e) => eprintln!("{e}"),
                };
            }
        }
    }
}
