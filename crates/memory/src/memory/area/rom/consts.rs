/// (max 2MByte ROM and/or 32 KiB RAM)
pub const MBC1_MAX_SIZE: usize = 2_097_152;
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

///(max 2MByte ROM and/or 32KByte RAM and Timer)
pub const MBC3_MAX_SIZE: usize = 16_777_216;
pub const MBC3_RAM_BASE: usize = 0x2000;
pub const MBC3_RAM_OFFSET: usize = 0x1fff;
pub const MBC3_MAGIC_LOCK: u8 = 0x0a;
pub const MBC3_RTC_OFFSET: usize = 0x0a;
pub const MBC3_REG0_START: usize = 0x0;
pub const MBC3_REG0_END: usize = 0x1fff;
pub const MBC3_REG1_START: usize = 0x2000;
pub const MBC3_REG1_END: usize = 0x3fff;
pub const MBC3_REG2_START: usize = 0x4000;
pub const MBC3_REG2_END: usize = 0x5fff;
pub const MBC3_REG3_START: usize = 0x6000;
pub const MBC3_REG3_END: usize = 0x7fff;
pub const MBC3_REG4_START: usize = 0xa000;
pub const MBC3_REG4_END: usize = 0xbfff;
