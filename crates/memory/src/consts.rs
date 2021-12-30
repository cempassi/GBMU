/// Memory Areas
pub const BIOS_MIN: u16 = 0x0000;
pub const BIOS_MAX: u16 = 0x00ff;
pub const ROM_MIN: u16 = 0x0000;
pub const ROM_MAX: u16 = 0x7fff;
pub const VRAM_MIN: u16 = 0x8000;
pub const VRAM_MAX: u16 = 0x9fff;
pub const EXT_RAM_MIN: u16 = 0xa000;
pub const EXT_RAM_MAX: u16 = 0xbfff;
pub const WRAM_MIN: u16 = 0xc000;
pub const WRAM_MAX: u16 = 0xdfff;
pub const ECHO_MIN: u16 = 0xe000;
pub const ECHO_MAX: u16 = 0xfdff;
pub const OAM_MIN: u16 = 0xfe00;
pub const OAM_MAX: u16 = 0xfe9f;
pub const IOREG_MIN: u16 = 0xff00;
pub const IOREM_MAX: u16 = 0xff7f;
pub const HRAM_MIN: u16 = 0xff80;
pub const HRAM_MAX: u16 = 0xfffe;
pub const RESTRICTED_MIN: u16 = 0xfea0;
pub const RESTRICTED_MAX: u16 = 0xfeff;

pub const BIOS_DISABLE: u16 = 0xFF50;

pub const HIGH_RAM_SIZE: usize = 127;

pub const DMA_TRANSFERT: u16 = 0xFF46;
pub const DMA_LEN: usize = 0xA0;

pub const LCD_CONTROL: u16 = 0xFF40;
pub const LY_COMPARE: u16 = 0xFF45;
pub const YWINDOW: u16 = 0xFF4A;
pub const XWINDOW: u16 = 0xFF4B;
pub const BGP: u16 = 0xFF47;

// Timer
pub const DIV: u16 = 0xFF04;
pub const TIMA: u16 = 0xFF05;
pub const TMA: u16 = 0xFF06;
pub const TAC: u16 = 0xFF07;

// Serial
pub const SERIAL_DATA: u16 = 0xFF01;
pub const SERIAL_CONTROL: u16 = 0xFF02;

// Joypad
pub const JOYPAD: u16 = 0xFF00;

/// Registers Addresses
pub const INTERRUPT_FLAGS: u16 = 0xFF0F;
pub const INTERRUPT_ENABLED: u16 = 0xFFFF;
