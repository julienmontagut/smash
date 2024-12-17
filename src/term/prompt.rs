use crossterm::{
    style::{self, Attribute, Color, Stylize},
    QueueableCommand,
};
use std::env;
use std::io::{self, stdout, Write};

pub fn print_prompt() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    let home_dir = env::var("HOME").unwrap_or_default();

    let relative_path = if current_dir.starts_with(&home_dir) {
        current_dir
            .strip_prefix(&home_dir)
            .map(|p| format!("~{}", p.display()))
            .unwrap_or(current_dir.display().to_string())
    } else {
        current_dir.display().to_string()
    };

    let mut output = stdout();
    let styled = Stylize::new(relative_path)
        .with(Color::Blue)
        .attribute(Attribute::Bold);
    output
        .queue(style::PrintStyledContent(styled))?
        .queue(style::Print(" > "))?
        .flush()?;
    Ok(())
}

fn get_current_dir_string() -> io::Result<String> {
    let path = env::current_dir()?;
    match path.to_str() {
        Some(p) => Ok(p.to_string()),
        None => Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to convert path to string",
        )),
    }
}
