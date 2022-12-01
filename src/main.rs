use clap::{arg, Command};
use std::{
    io::{stderr, stdin, stdout, Write},
    process::exit,
};
mod term;

static LUSH_APP_NAME: &str = env!("CARGO_PKG_NAME");
static LUSH_APP_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = Command::new(LUSH_APP_NAME)
        .version(LUSH_APP_VERSION)
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .disable_version_flag(true)
        .args(&[arg!(-v --version "Prints version information")])
        .get_matches();

    if matches.get_flag("version") {
        println!("{} {}", LUSH_APP_NAME, LUSH_APP_VERSION);
        exit(0);
    }

    let mut input = String::new();

    loop {
        term::prompt::print_prompt();

        if let Err(error) = stdin().read_line(&mut input) {
            writeln!(stderr(), "Error: {}", error).unwrap();
            exit(1);
        }

        let mut args = input.trim().split_whitespace();
        let command = args.next();

        match command {
            Some("exit") => exit(0),
            Some("echo") => println!("{}", args.collect::<Vec<&str>>().join(" ")),
            Some("env") => {
                for (key, value) in std::env::vars() {
                    println!("{}={}", key, value);
                }
            }
            Some("cd") => {
                let default_dir = std::env::var("HOME").unwrap_or(".".to_string());
                let dir = args
                    .peekable()
                    .peek()
                    .map_or(default_dir, |dir| dir.to_string());
                let new_dir = std::env::set_current_dir(dir);
                match new_dir {
                    Ok(_) => (),
                    Err(e) => println!("{}", e),
                }
            }
            Some("clear") => {
                print!("\x1B[2J\x1B[1;1H");
            }
            Some("terminal") => {
                let terminal_size = term::specs::size();
                let terminal_kind = term::specs::kind();

                println!(
                    "Terminal: {} size {}x{}",
                    terminal_kind, terminal_size.width, terminal_size.height
                );
            }
            Some(command) => {
                let output = std::process::Command::new(command).args(args).output();

                match output {
                    Ok(output) => {
                        stdout().write_all(&output.stdout).unwrap();
                        stderr().write_all(&output.stderr).unwrap();
                    }
                    Err(err) => println!("lush: {}", err),
                }
            }
            None => continue,
        }

        input.clear();
    }
}
