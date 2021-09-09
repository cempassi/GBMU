pub const MBC1_MAX_SIZE: usize = 65_536; // / 8;
pub const MBC_BANK0_START: usize = 0x0000;
pub const MBC_BANK0_END: usize = 0x3fff;
pub const MBC_BANK1_START: usize = 0x4000;
pub const MBC_BANK1_END: usize = 0x7fff;
pub const MBC_RAM_START: usize = 0xa000;
pub const MBC_RAM_END: usize = 0xbfff;

pub const MBC1_RAM_BASE: usize = 0x2000;
pub const MBC1_RAM_OFFSET: usize = 0x1fff;
pub const MBC1_MAGIC_LOCK: u8 = 0x0a;
pub const MBC1_REG0_START: usize = 0x0;
pub const MBC1_REG0_END: usize = 0x1fff;
pub const MBC1_REG1_START: usize = 0x2000;
pub const MBC1_REG1_END: usize = 0x3fff;
pub const MBC1_REG2_START: usize = 0x4000;
pub const MBC1_REG2_END: usize = 0x5fff;
pub const MBC1_REG3_START: usize = 0x6000;
pub const MBC1_REG3_END: usize = 0x7fff;

pub const MBC2_MAX_SIZE: usize = 262_144; // Controller for up to 2 Mbits (256 Kbytes) of ROM with built-in backup RAM (512 x 4 bits).
pub const MBC2_RAM_END: usize = 0xa1ff;
pub const MBC2_ERAM_START: usize = 0xa200;
pub const MBC2_ERAM_END: usize = 0xbfff;
pub const MBC2_MAGIC_LOCK: usize = 0x0a;
pub const MBC2_MAGIC_BYTE: usize = 0x100;
pub const MBC2_REG_START: usize = 0x0;
pub const MBC2_REG_END: usize = 0x3fff;
