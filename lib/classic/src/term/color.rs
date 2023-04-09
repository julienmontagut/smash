
pub enum BasicColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Default,
}

pub enum Color8bit {
    BasicColor(BasicColor),
    BrightColor(BasicColor),
    Color(u8),
    Gray(u8),
}
