use std::{
    io::{stderr, stdin, stdout, Write},
    process::exit,
};

use clap::ArgMatches;

mod classic;
mod term;

struct Command<'a> {
    name: &'a str,
    args: Vec<String>,
}

pub fn run_loop(matches: ArgMatches) -> ! {

    let mut input = String::new();

    loop {
        term::prompt::print_prompt();

        let command = read_line(&mut input);

        match command {
            Some(Command { name: "exit", .. }) => exit(0),
            Some(Command { name: "echo", args }) => println!("{}", args.join(" ")),
            Some(Command { name: "env", .. }) => {
                for (key, value) in std::env::vars() {
                    println!("{}={}", key, value);
                }
            }
            Some(Command { name: "cd", args }) => {
                let default_dir = std::env::var("HOME").unwrap_or(".".to_string());
                let dir = args
                    .iter()
                    .peekable()
                    .peek()
                    .map_or(default_dir, |dir| dir.to_string());
                let new_dir = std::env::set_current_dir(dir);
                match new_dir {
                    Ok(_) => (),
                    Err(e) => println!("{}", e),
                }
            }
            Some(Command { name: "clear", .. }) => {
                print!("\x1B[2J\x1B[1;1H");
            }
            Some(Command {
                name: "terminal", ..
            }) => {
                let terminal_size = term::specs::size();
                let terminal_kind = term::specs::kind();

                println!(
                    "Terminal: {} size {}x{}",
                    terminal_kind, terminal_size.width, terminal_size.height
                );
            }
            Some(Command { name, args }) => {
                let output = std::process::Command::new(name).args(args).output();

                match output {
                    Ok(output) => {
                        stdout().write_all(&output.stdout).unwrap();
                        stderr().write_all(&output.stderr).unwrap();
                    }
                    Err(err) => println!("kosh: {}", err),
                }
            }
            None => continue,
        }

        input.clear();
    }
}

fn read_line(input: &mut String) -> Option<Command> {
    if let Err(error) = stdin().read_line(input) {
        writeln!(stderr(), "Error: {}", error).unwrap();
        exit(1);
    }

    let mut split_input = input.trim().split_whitespace();

    if let Some(name) = split_input.next() {
        let args = split_input.map(|arg| arg.to_string()).collect();
        Some(Command { name, args })
    } else {
        None
    }
}
