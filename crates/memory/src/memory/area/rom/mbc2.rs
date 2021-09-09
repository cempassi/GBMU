use shared::{traits::Bus, Error};
pub const MBC2_MAX_SIZE: usize = 262_144; // Controller for up to 2 Mbits (256 Kbytes) of ROM with built-in backup RAM (512 x 4 bits).
pub const MBC_BANK0_START: usize = 0x0000;
pub const MBC_BANK0_END: usize = 0x3fff;
pub const MBC_BANK1_START: usize = 0x4000;
pub const MBC_BANK1_END: usize = 0x7fff;
pub const MBC_RAM_START: usize = 0xa000;
pub const MBC_RAM_END: usize = 0xa1ff;
pub const MBC2_ERAM_START: usize = 0xa200;
pub const MBC2_ERAM_END: usize = 0xbfff;
pub const MBC2_MAGIC_LOCK: usize = 0x0a;
pub const MBC2_MAGIC_BYTE: usize = 0x100;
pub const MBC2_REG_START: usize = 0x0;
pub const MBC2_REG_END: usize = 0x3fff;

pub struct Mbc2 {
    ram_lock: bool,
    data: Vec<u8>,
    rom_bank: u8, // Max 16 [0..=f]
}

impl Bus<usize> for Mbc2 {
    type Item = u8;
    type Result = Result<(), Error>;
    type Data = u8;

    fn get(&self, address: usize) -> Self::Item {
        match address {
            MBC_BANK0_START..=MBC_BANK0_END => self.data[address],
            MBC_BANK1_START..=MBC_BANK1_END => {
                self.data[(self.rom_bank as usize * MBC_BANK1_START) + (address - MBC_BANK1_START)]
            }
            MBC_RAM_START..=MBC2_ERAM_END => {
                if self.ram_lock {
                    let offset = self.get_ram_offset(address);
                    self.data[address - offset] & 0xf
                } else {
                    0
                }
            }
            _ => panic!("Invalid Address {:04X}", address),
        }
    }

    fn set(&mut self, address: usize, data: Self::Data) -> Self::Result {
        match address {
            MBC2_REG_START..=MBC2_REG_END => self.mbc2_register(address, data),
            MBC_RAM_START..=MBC2_ERAM_END => {
                if self.ram_lock {
                    let offset = self.get_ram_offset(address);
                    self.data[address - offset] = data & 0xf
                }
            }
            _ => return Err(shared::Error::IllegalSet(address, data)),
        };
        Ok(())
    }
}

impl Mbc2 {
    pub fn new(data: Vec<u8>) -> Self {
        Mbc2 {
            ram_lock: false,
            data,
            rom_bank: 1,
        }
    }

    fn mbc2_register(&mut self, address: usize, data: u8) {
        if address & MBC2_MAGIC_BYTE != 0 {
            self.rom_bank = if data == 0 { 1 } else { data & 0xf };
        } else {
            self.ram_lock = (data & 0x0f) == MBC2_MAGIC_LOCK as u8;
        }
    }

    fn get_ram_offset(&self, address: usize) -> usize {
        match address {
            MBC_RAM_START..=MBC_RAM_END => MBC_RAM_START,
            MBC2_ERAM_START..=MBC2_ERAM_END => MBC2_ERAM_START,
            _ => unreachable!(),
        }
    }
}

impl Default for Mbc2 {
    fn default() -> Self {
        Mbc2::new(vec![0; MBC2_MAX_SIZE])
    }
}

#[cfg(test)]
mod mbc2_test {
    use super::Mbc2;
    use shared::traits::Bus;
    const FILE: &[u8; 65536] = include_bytes!("../../../../../../roms/Ayakashi.gb");

    #[test]
    fn test_is_mbc2_rom() {
        let rom_file = FILE.to_vec();
        let mbc = Mbc2::new(rom_file);
        assert_eq!(mbc.data[0x147], 0x06);
    }

    #[test]
    fn test_mbc2_get_0() {
        let rom_file = FILE.to_vec();
        let mbc = Mbc2::new(rom_file);
        let data = mbc.get(0);
        assert_eq!(data, 0xe1);
    }

    #[test]
    fn test_mbc2_get_ram_a0e0() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc2::new(rom_file);

        mbc.set(0x00ff, 0x0a).unwrap();
        assert_eq!(mbc.ram_lock, true);

        mbc.set(0x0000a0e0, 0xeb);
        assert_eq!(FILE[0x0000a0e0], 0xea);

        let data = mbc.get(0x0000a0e0);
        assert_eq!(data, 0x0b);

        mbc.set(0x00ff, 0x00).unwrap();
        assert_eq!(mbc.ram_lock, false);
    }

    #[test]
    fn test_mbc2_unlock() {
        let rom_file = FILE.to_vec(); // MBC2 + RAM + BATTERY
        let mut mbc = Mbc2::new(rom_file);

        mbc.set(0x00ff, 0x0a).unwrap();
        assert_eq!(mbc.ram_lock, true);
    }

    #[test]
    fn test_mbc2_lock() {
        let rom_file = FILE.to_vec(); // MBC2 + RAM + BATTERY
        let mut mbc = Mbc2::new(rom_file);

        mbc.set(0x00ff, 0x0a).unwrap();
        assert_eq!(mbc.ram_lock, true);

        mbc.set(0x00fb, 0x03).unwrap();
        assert_eq!(mbc.ram_lock, false)
    }

    #[test]
    fn test_mbc2_change_bank_to_0() {
        let rom_file = FILE.to_vec(); // MBC2 + RAM + BATTERY
        let mut mbc = Mbc2::new(rom_file);

        mbc.set(0x2152, 0x00).unwrap();
        assert_eq!(mbc.rom_bank, 0x01);
    }

    #[test]
    fn test_mbc2_change_bank_0a() {
        let rom_file = FILE.to_vec(); // MBC2 + RAM + BATTERY
        let mut mbc = Mbc2::new(rom_file);

        mbc.set(0x3f52, 0x1a).unwrap();
        assert_eq!(mbc.rom_bank, 0x0a);
    }

    #[test]
    fn test_mbc2_change_bank_0f() {
        let rom_file = FILE.to_vec(); // MBC2 + RAM + BATTERY
        let mut mbc = Mbc2::new(rom_file);

        mbc.set(0x2fff, 0xff).unwrap();
        assert_eq!(mbc.rom_bank, 0x0f);
    }

    #[test]
    fn test_mbc2_write_in_ram_a130() {
        let rom_file = FILE.to_vec(); // MBC2 + RAM + BATTERY
        let mut mbc = Mbc2::new(rom_file);

        mbc.set(0x00f5, 0x0a).unwrap();
        assert_eq!(mbc.ram_lock, true);

        mbc.set(0x0000a130, 0xca).unwrap();

        let data = mbc.get(0x0000a130);
        assert_eq!(data, 0x0a);

        mbc.set(0x00ff, 0x00).unwrap();
        assert_eq!(mbc.ram_lock, false);
    }

    #[test]
    fn test_mbc2_read_in_eram_a630() {
        let rom_file = FILE.to_vec(); // MBC2 + RAM + BATTERY
        let mut mbc = Mbc2::new(rom_file);

        mbc.set(0x00f5, 0x0a).unwrap();
        assert_eq!(mbc.ram_lock, true);

        let data = mbc.get(0x0000a630);
        assert_eq!(data, 0x0f);

        mbc.set(0x00ff, 0x00).unwrap();
        assert_eq!(mbc.ram_lock, false);
    }

    #[test]
    fn test_mbc2_change_bank_13() {
        let rom_file = FILE.to_vec(); // MBC2 + RAM + BATTERY
        let mut mbc = Mbc2::new(rom_file);

        mbc.set(0x00f5, 0x0a).unwrap();
        assert_eq!(mbc.ram_lock, true);

        mbc.set(0x1f4b, 0x0d).unwrap();
        assert_eq!(mbc.rom_bank, 13);

        mbc.set(0x00ff, 0x00).unwrap();
        assert_eq!(mbc.ram_lock, false);
    }
}
