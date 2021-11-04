/// LCDC is the main LCD Control register.
/// Its bits toggle what elements are displayed on the screen, and how.

/// Bit  Name                           Usage notes              Mask
/// 7    LCD and PPU enable             0=Off, 1=On
/// 6    Window tile map area           0=9800-9BFF, 1=9C00-9FFF
/// 5    Window enable                  0=Off, 1=On
/// 4    BG and Window tile data area   0=8800-97FF, 1=8000-8FFF
/// 3    BG tile map area               0=9800-9BFF, 1=9C00-9FFF
/// 2    OBJ size                       0=8x8, 1=8x16
/// 1    OBJ enable                     0=Off, 1=On
/// 0    BG and Window enable/priority  0=Off, 1=On

#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(dead_code)]
pub struct Control {
    pub(crate) lcd_enabled: bool,
    pub(crate) window_area: u16,
    pub(crate) window_enabled: bool,
    pub(crate) data_area: u16,
    pub(crate) bg_area: u16,
    pub(crate) sprite_size: u8,
    pub(crate) sprite_enabled: bool,
    pub(crate) priority: bool,
}

impl Default for Control {
    fn default() -> Self {
        Self {
            lcd_enabled: false,
            window_area: 0x9C00,
            window_enabled: false,
            data_area: 0x8000,
            bg_area: 0x9C00,
            sprite_size: 8,
            sprite_enabled: false,
            priority: false,
        }
    }
}

impl Control {
    pub fn get(&self) -> u8 {
        let mut byte: u8 = 0;
        byte |= if self.lcd_enabled { 0x80 } else { 0 };
        byte |= if self.window_area == 0x9C00 { 0x40 } else { 0 };
        byte |= if self.window_enabled { 0x20 } else { 0 };
        byte |= if self.data_area == 0x8000 { 0x10 } else { 0 };
        byte |= if self.bg_area == 0x9C00 { 0x08 } else { 0 };
        byte |= if self.sprite_size == 16 { 0x04 } else { 0 };
        byte |= if self.sprite_enabled { 0x02 } else { 0 };
        byte |= if self.priority { 0x01 } else { 0 };
        byte
    }

    pub fn set(&mut self, byte: u8) {
        self.lcd_enabled = byte & 0x80 == 0x80;
        self.window_area = if byte & 0x40 == 0x40 { 0x9C00 } else { 0x9800 };
        self.window_enabled = byte & 0x20 == 0x20;
        self.data_area = if byte & 0x10 == 0x10 { 0x8000 } else { 0x8800 };
        self.bg_area = if byte & 0x08 == 0x08 { 0x9C00 } else { 0x9800 };
        self.sprite_size = if byte & 0x04 == 0x04 { 16 } else { 8 };
        self.sprite_enabled = byte & 0x02 == 0x02;
        self.priority = byte & 0x01 == 0x01;
    }
}
