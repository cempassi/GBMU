/// Memory Areas
pub const BIOS_MIN: u16 = 0x0000;
pub const BIOS_MAX: u16 = 0x00FF;
pub const ROM_MIN: u16 = 0x000;
pub const ROM_MAX: u16 = 0x7FFF;
pub const VRAM_MIN: u16 = 0x8000;
pub const VRAM_MAX: u16 = 0x9FFF;
pub const EXT_RAM_MIN: u16 = 0xA000;
pub const EXT_RAM_MAX: u16 = 0xBFFF;
pub const WRAM_MIN: u16 = 0xC000;
pub const WRAM_MAX: u16 = 0xDFFF;
pub const ECHO_MIN: u16 = 0xe000;
pub const ECHO_MAX: u16 = 0xFDFF;
pub const OAM_MIN: u16 = 0xFE00;
pub const OAM_MAX: u16 = 0xFE9F;
pub const IOREG_MIN: u16 = 0xFF00;
pub const IOREM_MAX: u16 = 0xFF7F;
pub const HRAM_MIN: u16 = 0xFF80;
pub const HRAM_MAX: u16 = 0xFFFE;
pub const RESTRICTED_MIN: u16 = 0xFEA0;
pub const RESTRICTED_MAX: u16 = 0xFEFF;

pub const HIGH_RAM_SIZE: usize = 127;

/// Registers Addresses
pub const INTERRUPT_FLAGS: u16 = 0xFF0F;
pub const INTERRUPT_ENABLED: u16 = 0xFFFF;

pub const LCD_CONTROL: u16 = 0xFF40;
pub const LY_COMPARE: u16 = 0xFF45;
pub const YWINDOW: u16 = 0xFF4A;
pub const XWINDOW: u16 = 0xFF4B;
pub const BGP: u16 = 0xFF47;
