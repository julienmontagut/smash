use std::{
    io::{stdin, stdout, Write},
    process::exit,
};
mod prompt;

fn main() {
    let args = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            clap::Arg::with_name("version")
                .short("v")
                .long("version")
                .help("Prints the lush shell version"),
        )
        .get_matches();

    if args.is_present("version") {
        println!(
            "{} version {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );
        exit(0);
    }
    let mut input = String::new();

    loop {
        prompt::print();

        stdin().read_line(&mut input).unwrap();

        let mut args = input.trim().split_whitespace();
        let command = args.next();

        match command {
            Some("exit") => exit(0),
            Some("echo") => println!("{}", args.collect::<Vec<&str>>().join(" ")),
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
            Some("help") => {
                println!("exit - exit the shell");
                println!("echo - echo arguments");
                println!("help - show this help");
            }
            Some(command) => {
                let output = std::process::Command::new(command)
                    .args(args)
                    .output()
                    .unwrap();

                stdout().write_all(&output.stdout).unwrap();
            }
            None => continue,
        }

        input.clear();
    }
}
