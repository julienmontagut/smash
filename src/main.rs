use std::{
    io::{stderr, stdin, stdout, Write},
    process::exit,
};

use clap::{arg, Command};

static KOSH_APP_NAME: &str = env!("CARGO_PKG_NAME");
static KOSH_APP_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = Command::new(KOSH_APP_NAME)
        .version(KOSH_APP_VERSION)
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .disable_version_flag(true)
        .args(&[arg!(-v --version "Prints version information")])
        .get_matches();

    if matches.get_flag("version") {
        println!("{} {}", KOSH_APP_NAME, KOSH_APP_VERSION);
        exit(0);
    }

    // kosh::run_loop(matches);
    kosh::run_basic_loop();
}
