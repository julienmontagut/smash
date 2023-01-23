use std::io::{stdout, Write};
use super::specs::{escape_string_from_style,FontStyle, Style};
use super::color::{BasicColor};

pub fn print_prompt() {
    let path = std::env::current_dir().unwrap();
    let path = path.to_str().unwrap();
    let path = path.replace(
        std::env::var("HOME").unwrap().as_str(),
        "~"
    );

    let style_emphasis = Style::new(BasicColor::Blue, BasicColor::Default, FontStyle::Bold);
    let style_default = Style::new(BasicColor::Default, BasicColor::Default, FontStyle::Normal);

    println!("{}{}{}", escape_string_from_style(&style_emphasis), path, escape_string_from_style(&style_default));
    print!("> ");
    if let Err(err) = stdout().flush() {
        panic!("kosh: Failed to flush stdout: {}", err);
    }
}
