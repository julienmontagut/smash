use std::fmt;

use crossterm::terminal;

pub const ESC_BEGIN: &str = "\x1b[";
pub const ESC_END: &str = "m";

pub const ESC_VALUE_BELL: &str = "7";
pub const ESC_VALUE_CLEAR: &str = "2J";
pub const ESC_VALUE_CLEAR_LINE: &str = "2K";
pub const ESC_VALUE_CURSOR_UP: &str = "1A";
pub const ESC_VALUE_CURSOR_DOWN: &str = "1B";
pub const ESC_VALUE_CURSOR_LEFT: &str = "1D";
pub const ESC_VALUE_CURSOR_RIGHT: &str = "1C";
pub const ESC_VALUE_RESET: &str = "0";

pub enum Color {
    None = 0,
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37,
}

pub enum FontStyle {
    Normal,
    Bold,
}

pub enum Cursor {
    Up,
    Down,
    Left,
    Right,
}

pub enum CursorVisibility {
    Visible,
    Hidden,
}

pub enum Escape {
    Bell,
    ClearLine,
    ClearScreen,
    Color(Color),
    Cursor(Cursor),
    CursorPosition(u16, u16),
    CursorVisibility(CursorVisibility),
    FontStyle(FontStyle),
    Reset,
}

pub trait Escapable {
    fn to_escape(&self) -> Escape;
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        output.push_str(ESC_BEGIN);
        output.push_str("38;5;");
        output.push_str(&self.to_string());
        output.push_str(ESC_END);
        write!(f, "{}", output)
    }
}

pub fn escape_string(escape: Escape) -> String {
    let mut output = String::new();
    output.push_str(ESC_BEGIN);
    match escape {
        Escape::Bell => output.push_str(ESC_VALUE_BELL),
        Escape::ClearLine => output.push_str(ESC_VALUE_CLEAR_LINE),
        Escape::ClearScreen => output.push_str("2J"),
        Escape::Color(color) => {
            output.push_str("38;5;");
            output.push_str(&color.to_string())
        }
        Escape::Cursor(cursor) => match cursor {
            Cursor::Up => output.push_str(ESC_VALUE_CURSOR_UP),
            Cursor::Down => output.push_str(ESC_VALUE_CURSOR_DOWN),
            Cursor::Left => output.push_str(ESC_VALUE_CURSOR_LEFT),
            Cursor::Right => output.push_str(ESC_VALUE_CURSOR_RIGHT),
        },
        Escape::CursorPosition(x, y) => {
            output.push_str(&(y + 1).to_string());
            output.push_str(";");
            output.push_str(&(x + 1).to_string());
            output.push_str("H");
        }
        Escape::CursorVisibility(cursor_visibility) => match cursor_visibility {
            CursorVisibility::Visible => output.push_str("?25h"),
            CursorVisibility::Hidden => output.push_str("?25l"),
        },
        Escape::FontStyle(font_style) => match font_style {
            FontStyle::Normal => output.push_str("0"),
            FontStyle::Bold => output.push_str("1"),
        },
        Escape::Reset => output.push_str("0"),
    }
    output.push_str(ESC_END);
    output
}
