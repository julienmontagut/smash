use clap::ArgMatches;
use std::process::exit;
use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::terminal;
use std::error::Error;
use std::io;
use std::io::{stderr, stdin, stdout, Write};

mod term;

struct Command<'a> {
    name: &'a str,
    args: Vec<String>,
}

pub fn run_loop(_matches: ArgMatches) -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut exiting = false;
    
    while !exiting {
        term::prompt::print_prompt();
        if let Ok(input) = read_input() {
            let command = parse_command(&input);

            match command {
                Some(Command { name: "exit", .. }) => exiting = true,
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
            Some(Command { name: "pwd", .. }) => {
                let current_dir = std::env::current_dir().unwrap();
                println!("{}", current_dir.display());
            }
            Some(Command { name: "clear", .. }) => {
                print!("\x1B[2J\x1B[1;1H");
            }
            Some(Command {
                name: "terminal", ..
            }) => {
                let terminal_size = classic::term::specs::size();
                let terminal_kind = classic::term::specs::kind();

                    println!("Terminal: {} size {}x{}", terminal_kind, terminal_size.width, terminal_size.height);
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
        }
    }

    Ok(())
}

fn read_input() -> io::Result<String> {
    let mut input = String::new();
    while let Event::Key(KeyEvent { code, .. }) = event::read()? {
        match code {
            KeyCode::Enter => {
                break;
            }
            KeyCode::Char(char) => {
                print!("{}", char);
                input.push(char);
                stdout().write(char.to_string().as_bytes())?;
                stdout().flush()?;
            }
            _ => {}
        }
        input.clear();
    }
    Ok(input)
}

fn init_posix() -> Result<(), Box<dyn Error>> {
    todo!()
}

fn read_line(input: &mut String) -> Option<Command> {
    if let Err(error) = stdin().read_line(input) {
        writeln!(stderr(), "Error: {}", error).unwrap();
        exit(1);
    }
    Ok(input)
}

fn parse_command(input: &String) -> Option<Command> {
    let mut split_input = input.trim().split_whitespace();

    if let Some(name) = split_input.next() {
        let args = split_input.map(|arg| arg.to_string()).collect();
        Some(Command { name, args })
    } else {
        None
    }
}
