use std::io::{stdout, Write};

pub fn print() {
    let path = std::env::current_dir().unwrap();
    let path = path.to_str().unwrap();
    let path = path.replace(
        std::env::var("HOME").unwrap().as_str(),
        "~"
    );

    print!("{} > ", path);
    stdout().flush().unwrap();
}
