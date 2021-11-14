use super::bus::MbcBus;
use super::consts;
use shared::Error;
use std::convert::AsRef;

#[derive(Debug)]
pub struct Mbc1 {
    ram_lock: bool,
    /// [false] = rom mode | [true] = ram mode
    bank_mode: bool,
    data: Vec<u8>,
    rom_bank: u8,
    ram_bank: u8,
}

impl Default for Mbc1 {
    fn default() -> Self {
        Mbc1 {
            ram_lock: false,
            bank_mode: false,
            data: vec![0; consts::MBC1_MAX_SIZE],
            rom_bank: 0,
            ram_bank: 0,
        }
    }
}

impl AsRef<Vec<u8>> for Mbc1 {
    fn as_ref(&self) -> &Vec<u8> {
        self.data.as_ref()
    }
}

impl MbcBus for Mbc1 {
    fn get(&self, address: usize) -> Result<u8, Error> {
        match address {
            consts::MBC_BANK0_START..=consts::MBC_BANK0_END => Ok(self.data[address]),
            consts::MBC_BANK1_START..=consts::MBC_BANK1_END => Ok(self.swap_bank_nbr(address)),
            consts::MBC_RAM_START..=consts::MBC_RAM_END => Ok(self.swap_bank_nbr(address)),
            _ => unreachable!(),
        }
    }

    fn set(&mut self, address: usize, data: u8) -> Result<(), Error> {
        match address {
            consts::MBC1_REG0_START..=consts::MBC1_REG0_END => self.update_ram_lock(data),
            consts::MBC1_REG1_START..=consts::MBC1_REG2_END => self.update_bank_nbr(address, data),
            consts::MBC1_REG3_START..=consts::MBC1_REG3_END => self.update_bank_mode(data),
            consts::MBC_RAM_START..=consts::MBC_RAM_END => self.set_ram(address, data),
            _ => Err(shared::Error::IllegalSet(address, data)),
        }
    }
}

impl Mbc1 {
    pub fn new(data: Vec<u8>) -> Box<Self> {
        Box::new(Mbc1 {
            ram_lock: false,
            bank_mode: false,
            data,
            rom_bank: 0,
            ram_bank: 0,
        })
    }

    /// Write the Data into the Ram at the address
    fn set_ram(&mut self, address: usize, data: u8) -> Result<(), Error> {
        if !self.ram_lock {
            return Err(shared::Error::RamLock(address));
        }
        let bank_nbr = if self.bank_mode {
            self.ram_bank as usize
        } else {
            // Should be undefined behavior
            0
        };
        let index = (bank_nbr * consts::MBC_RAM_BASE) | (address & consts::MBC_RAM_OFFSET);
        self.data[index] = data;
        Ok(())
    }

    /// Get the current state bank and return it;
    fn swap_bank_nbr(&self, address: usize) -> u8 {
        let (bank_nbr, start_off, end_off) = match address {
            consts::MBC_BANK1_START..=consts::MBC_BANK1_END => {
                let bank_nbr = if self.rom_bank != 0 {
                    self.rom_bank as usize
                } else {
                    1
                };
                (bank_nbr, consts::MBC_BANK1_START, consts::MBC_BANK1_START)
            }
            consts::MBC_RAM_START..=consts::MBC_RAM_END => {
                let bank_nbr = if self.ram_lock && self.bank_mode {
                    self.ram_bank as usize
                } else {
                    // Should be undefined behavior
                    0
                };
                (bank_nbr, consts::MBC_RAM_BASE, consts::MBC_RAM_START)
            }
            _ => unreachable!(),
        };
        let index = ((bank_nbr * start_off) + (address - end_off)) & (self.data.len() - 1);
        self.data[index]
    }

    /// Swap Banking mode between Rom (false), and Ram (true)
    fn update_bank_mode(&mut self, data: u8) -> Result<(), Error> {
        self.bank_mode = match data & 0x01 {
            0 => false,
            1 => true,
            _ => unreachable!(),
        };
        Ok(())
    }

    /// Update Rom Or Ram Bank depending on bank_mode
    fn update_bank_nbr(&mut self, address: usize, data: u8) -> Result<(), Error> {
        match address {
            consts::MBC1_REG1_START..=consts::MBC1_REG1_END => {
                self.rom_bank = match (self.rom_bank & !0x1f) | (data & 0x1f) {
                    // ROM bank cannot be $00/$20/$40/$60 => that's why there is 125 banks
                    0 => 1,
                    20 => 21,
                    40 => 41,
                    60 => 61,
                    _nbr => _nbr,
                }
            }
            consts::MBC1_REG2_START..=consts::MBC1_REG2_END => {
                if !self.bank_mode {
                    // upper bit of rom bank nbr
                    self.rom_bank = (self.rom_bank & !0x60) | ((data & 0x03) << 5);
                } else {
                    // RAM bank nbr 00-11
                    self.ram_bank = data & 0x03
                }
            }
            _ => unreachable!(),
        };
        Ok(())
    }

    /// enable RAM REG0
    fn update_ram_lock(&mut self, data: u8) -> Result<(), Error> {
        self.ram_lock = data == consts::MBC_MAGIC_LOCK;
        Ok(())
    }
}

#[cfg(test)]
mod mbc1_test {
    use super::{Mbc1, MbcBus};

    const FILE: &[u8; 262144] = include_bytes!("../../../../roms/Metroid II - Return of Samus.gb");

    #[test]
    fn test_mbc1_get() {
        let rom_file = FILE.to_vec();
        let mbc = Mbc1::new(rom_file);
        let data = mbc.get(0x00000000);
        assert_eq!(data.unwrap(), 0xc3);
    }

    #[test]
    fn test_mbc1_set_lock() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc1::new(rom_file);

        assert_eq!(mbc.data[0x147], 0x03);

        mbc.set(0x01f5, 0x0a).unwrap();
        assert!(mbc.ram_lock);

        mbc.set(0x034b, 0x03).unwrap();
        assert!(!mbc.ram_lock);
    }

    #[test]
    fn test_mbc1_bank_mod() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc1::new(rom_file);

        mbc.set(0x6abc, 4).unwrap();
        assert!(!mbc.bank_mode);

        mbc.set(0x7abc, 3).unwrap();
        assert!(mbc.bank_mode);
    }

    #[test]
    fn test_mbc1_reg1_0() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc1::new(rom_file);

        mbc.set(0x2156, 0x00).unwrap();
        assert_eq!(mbc.rom_bank, 0x01);
    }

    #[test]
    fn test_mbc1_reg1_1a() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc1::new(rom_file);

        mbc.set(0x2156, 0x1a).unwrap();
        assert_eq!(mbc.rom_bank, 0x1a);
    }

    #[test]
    fn test_mbc1_reg1_14() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc1::new(rom_file);

        mbc.set(0x2fff, 0x14).unwrap();
        assert_eq!(mbc.rom_bank, 0x15);
    }

    #[test]
    fn test_mbc1_reg2_1_28() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc1::new(rom_file);

        mbc.set(0x4f4f, 0x01).unwrap();
        assert_eq!(mbc.rom_bank, 0x20);
        mbc.set(0x2fff, 0x08).unwrap();
        assert_eq!(mbc.rom_bank, 0x29);
    }

    #[test]
    fn test_mbc1_reg2_1_3c() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc1::new(rom_file);

        mbc.set(0x4f4f, 0x01).unwrap();
        assert_eq!(mbc.rom_bank, 0x20);
        mbc.set(0x2fff, 0x1c).unwrap();
        assert_eq!(mbc.rom_bank, 0x3d);
    }

    #[test]
    fn test_mbc1_reg2_1_14() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc1::new(rom_file);

        mbc.set(0x4f4f, 0x03).unwrap();
        assert_eq!(mbc.rom_bank, 0x60);
        mbc.set(0x2fff, 0x14).unwrap();
        assert_eq!(mbc.rom_bank, 0x74);
    }

    #[test]
    fn test_mbc1_get_last_rom_bank() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc1::new(rom_file);

        mbc.set(0x4f4f, 0x03).unwrap();
        assert_eq!(mbc.rom_bank, 0x60);
        mbc.set(0x2fff, 0x1f).unwrap();
        assert_eq!(mbc.rom_bank, 0x7f);
        // let data = mbc.get(0x7b80);
        // assert_eq!(data, 0xc0);
    }

    #[test]
    fn test_mbc1_write_in_ram() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc1::new(rom_file);

        mbc.set(0x01f5, 0x0a).unwrap();
        assert!(mbc.ram_lock);

        mbc.set(0x7abc, 3).unwrap(); // enable bank_mod
        assert!(mbc.bank_mode);

        mbc.set(0x4f4f, 0x00).unwrap();
        assert_eq!(mbc.ram_bank, 0);

        mbc.set(0x0000a630, 0xca).unwrap();

        let data = mbc.get(0x0000a630);
        assert_eq!(data.unwrap(), 0xca);

        mbc.set(0x01ff, 0x00).unwrap();
        assert!(!mbc.ram_lock);
    }

    #[test]
    fn test_mbc1_change_ram_bank() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc1::new(rom_file);

        mbc.set(0x01f5, 0x0a).unwrap();
        assert!(mbc.ram_lock);

        mbc.set(0x7abc, 3).unwrap();
        assert!(mbc.bank_mode);

        mbc.set(0x4f4f, 0x01).unwrap();
        assert_eq!(mbc.ram_bank, 1);

        mbc.set(0x01ff, 0x00).unwrap();
        assert!(!mbc.ram_lock);
    }
}
