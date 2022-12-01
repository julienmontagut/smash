use super::color::BasicColor;

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub fn size() -> Size {
    use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
    use std::mem;

    unsafe {
        let mut size: winsize = mem::zeroed();
        ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut size);
        Size {
            width: size.ws_col,
            height: size.ws_row,
        }
    }
}

pub fn kind() -> String {
    let terminal_name = std::env::var("TERM").unwrap();
    terminal_name
}

/// Writes a string to the terminal at the given position, with the given color and background color.
/// The position is 0-indexed.
/// The color is a 3-tuple of RGB values.
/// The string is truncated if it is too long to fit on the terminal.
/// The string is not written if the position is outside the terminal.
pub fn print_string(string: &str, position: (i8, i8), text_color: (i16, i16, i16)) {
    let size = size();
    let (x, y) = position;
    let (r, g, b) = text_color;

    if x < 0 || y < 0 || x as u16 >= size.width || y as u16 >= size.height {
        return;
    }

    let mut string = string.to_string();
    if string.len() > size.width as usize {
        string.truncate(size.width as usize);
    }

    let mut color = 16 + 36 * r + 6 * g + b;
    if color < 16 || color > 231 {
        color = 16;
    }

    let mut output = String::new();
    output.push_str("\x1b[38;5;");
    output.push_str(&color.to_string());
    output.push_str("m");
    output.push_str("\x1b[");
    output.push_str(&(y + 1).to_string());
    output.push_str(";");
    output.push_str(&(x + 1).to_string());
    output.push_str("H");
    output.push_str(&string);
    output.push_str("\x1b[0m");

    print!("{}", output);
}

pub enum FontStyle {
    Normal,
    Bold,
}

pub struct Style {
    pub fore_color: BasicColor,
    pub back_color: BasicColor,
    pub font_style: FontStyle,
}

impl Style {
    pub fn new(fore_color: BasicColor, back_color: BasicColor, font_style: FontStyle) -> Style {
        Style {
            fore_color,
            back_color,
            font_style,
        }
    }
}

pub fn escape_string_from_style(style: &Style) -> String {
    let mut output = String::new();
    output.push_str("\x1b[");
    match style.font_style {
        FontStyle::Normal => output.push_str("0"),
        FontStyle::Bold => output.push_str("1"),
    }
    output.push_str(";");
    match style.fore_color {
        BasicColor::Black => output.push_str("30"),
        BasicColor::Red => output.push_str("31"),
        BasicColor::Green => output.push_str("32"),
        BasicColor::Yellow => output.push_str("33"),
        BasicColor::Blue => output.push_str("34"),
        BasicColor::Magenta => output.push_str("35"),
        BasicColor::Cyan => output.push_str("36"),
        BasicColor::White => output.push_str("37"),
        BasicColor::Default => output.push_str("39"),
    }
    output.push_str(";");
    match style.back_color {
        BasicColor::Black => output.push_str("40"),
        BasicColor::Red => output.push_str("41"),
        BasicColor::Green => output.push_str("42"),
        BasicColor::Yellow => output.push_str("43"),
        BasicColor::Blue => output.push_str("44"),
        BasicColor::Magenta => output.push_str("45"),
        BasicColor::Cyan => output.push_str("46"),
        BasicColor::White => output.push_str("47"),
        BasicColor::Default => output.push_str("49"),
    }
    output.push_str("m");
    output
}
