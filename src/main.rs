use std::io::{stdin, stdout, Write};

// A simple shell written in Rust
fn main() {
    let mut input = String::new();

    loop {
        print_prompt();

        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "exit" => return,
            "cd" => {
                let dir = args.peekable().peek().map_or("/", |d| *d);
                std::env::set_current_dir(dir).unwrap();
            }
            command => {
                let output = std::process::Command::new(command)
                    .args(args)
                    .output()
                    .unwrap();

                stdout().write_all(&output.stdout).unwrap();
            }
        }

        input.clear();
    }
}

// Print the prompt
fn print_prompt() {
    let path = std::env::current_dir().unwrap();
    let path = path.to_str().unwrap();
    let path = path.replace(std::env::var("HOME").unwrap().as_str(), "~");

    print!("{} > ", path);
    stdout().flush().unwrap();
}
