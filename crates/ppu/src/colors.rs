#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    LightGray,
    DarkGray,
    Black,
}

impl Default for Color {
    fn default() -> Self {
        Color::Black
    }
}

impl From<Color> for [u8; 4] {
    fn from(color: Color) -> [u8; 4] {
        match color {
            Color::White => [0x9B, 0xBC, 0x0f, 0xFF],
            Color::LightGray => [0x8B, 0xAC, 0x0F, 0xFF],
            Color::DarkGray => [0x30, 0x62, 0x30, 0xFF],
            Color::Black => [0x0F, 0x38, 0x0F, 0xFF],
        }
    }
}

impl From<u8> for Color {
    fn from(color: u8) -> Self {
        match color {
            0 => Color::White,
            1 => Color::LightGray,
            2 => Color::DarkGray,
            3 => Color::Black,
            _ => Color::Black,
        }
    }
}

impl From<Color> for u8 {
    fn from(color: Color) -> Self {
        match color {
            Color::White => 0,
            Color::LightGray => 1,
            Color::DarkGray => 2,
            Color::Black => 3,
        }
    }
}
