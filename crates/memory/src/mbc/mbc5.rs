use super::bus::MbcBus;
use super::consts;
use super::Mbc;
use crate::MemoryBus;
use shared::Error;
use std::convert::AsRef;

#[derive(Debug)]
pub struct Mbc5 {
    ram_lock: bool,
    data: Vec<u8>,
    rom_bank: u16,
    ram_bank: u8,
}

impl Default for Mbc5 {
    fn default() -> Self {
        Mbc5 {
            ram_lock: false,
            data: vec![0; consts::MBC5_MAX_SIZE],
            rom_bank: 0,
            ram_bank: 0,
        }
    }
}

impl AsRef<Vec<u8>> for Mbc5 {
    fn as_ref(&self) -> &Vec<u8> {
        self.data.as_ref()
    }
}

impl MbcBus for Mbc5 {
    fn set(&mut self, address: usize, data: u8) -> Result<(), Error> {
        match address {
            consts::MBC5_REG0_START..=consts::MBC5_REG0_END => self.update_ram_lock(data),
            consts::MBC5_REG1_START..=consts::MBC5_REG3_END => self.set_bank_number(address, data),
            consts::MBC_RAM_START..=consts::MBC_RAM_END => self.set_ram(address, data),
            _ => Err(shared::Error::IllegalSet(address, data)),
        }
    }
}

impl MemoryBus for Mbc5 {
    fn get(&self, address: usize) -> u8 {
        match address {
            consts::MBC_BANK0_START..=consts::MBC_BANK0_END => self.data[address],
            consts::MBC_BANK1_START..=consts::MBC_BANK1_END => self.swap_bank_nbr(address),
            consts::MBC_RAM_START..=consts::MBC_RAM_END => self.swap_bank_nbr(address),
            _ => unreachable!(),
        }
    }

    fn set(&mut self, address: usize, data: u8) {
        let _ = <Self as MbcBus>::set(self, address, data);
    }
}

impl Mbc for Mbc5 {}

impl Mbc5 {
    pub fn new(data: Vec<u8>) -> Box<Self> {
        Box::new(Mbc5 {
            ram_lock: false,
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
        let index =
            (self.ram_bank as usize * consts::MBC_RAM_BASE) | (address & consts::MBC_RAM_OFFSET);
        self.data[index] = data;
        Ok(())
    }

    /// Get the Data at the Address depending on the actual bank number
    fn swap_bank_nbr(&self, address: usize) -> u8 {
        let (bank_nbr, start_off, end_off) = match address {
            consts::MBC_BANK1_START..=consts::MBC_BANK1_END => (
                self.rom_bank,
                consts::MBC_BANK1_START as u16,
                consts::MBC_BANK1_START,
            ),
            consts::MBC_RAM_START..=consts::MBC_RAM_END => {
                let bank_nbr = if self.ram_lock {
                    self.ram_bank as usize
                } else {
                    // should ret error undefined behavior
                    0
                };
                (
                    bank_nbr as u16,
                    consts::MBC_RAM_BASE as u16,
                    consts::MBC_RAM_START,
                )
            }
            _ => unreachable!(),
        };
        let index =
            ((bank_nbr * start_off) + (address - end_off) as u16) & (self.data.len() - 1) as u16;
        self.data[index as usize]
    }

    /// To selected the Rom bank number, you've 2 area were to write
    /// since his value can be contain in a 9-bits wide number (max value is 0x1FF)
    /// Ram bank number goes from 0x0 to 0xf
    fn set_bank_number(&mut self, address: usize, data: u8) -> Result<(), Error> {
        match address {
            consts::MBC5_REG1_START..=consts::MBC5_REG1_END => {
                self.rom_bank = (self.rom_bank & 0x100) | (data as u16)
            }
            consts::MBC5_REG2_START..=consts::MBC5_REG2_END => {
                self.rom_bank = (self.rom_bank & 0x0FF) | (((data as u16) & 0x1) << 8)
            }
            consts::MBC5_REG3_START..=consts::MBC5_REG3_END => self.ram_bank = data & 0x0f,
            _ => unreachable!(),
        }
        Ok(())
    }

    /// enable RAM Read/Write Operations REG0
    fn update_ram_lock(&mut self, data: u8) -> Result<(), Error> {
        self.ram_lock = data == consts::MBC_MAGIC_LOCK;
        Ok(())
    }
}

#[cfg(test)]
mod mbc5_test {
    use super::Mbc5;
    use crate::MemoryBus;

    const FILE: &[u8; 1048576] = include_bytes!("../../../../roms/Pokemon_Rouge.gb");

    #[test]
    fn test_mbc5_get() {
        let rom_file = FILE.to_vec();
        let mbc = Mbc5::new(rom_file);
        let data = mbc.get(0x00000250);
        assert_eq!(data, 0xad);
    }

    #[test]
    fn test_mbc5_set_lock() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc5::new(rom_file);

        assert_eq!(mbc.data[0x147], 0x1b);

        mbc.set(0x01f5, 0x0a);
        assert_eq!(mbc.ram_lock, true);

        mbc.set(0x034b, 0x03);
        assert_eq!(mbc.ram_lock, false)
    }

    #[test]
    fn test_mbc5_reg1_0() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc5::new(rom_file);

        mbc.set(0x2156, 0x00);
        assert_eq!(mbc.rom_bank, 0x00);
    }

    #[test]
    fn test_mbc5_reg1_ff() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc5::new(rom_file);

        mbc.set(0x2156, 0xff);
        assert_eq!(mbc.rom_bank, 0xff);
    }

    #[test]
    fn test_mbc5_reg2_reg1_1ff() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc5::new(rom_file);

        mbc.set(0x2fff, 0xff);
        assert_eq!(mbc.rom_bank, 0xff);

        mbc.set(0x3000, 0x01);
        assert_eq!(mbc.rom_bank, 0x1ff);
    }

    #[test]
    fn test_mbc5_get_rom_bank_1ff() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc5::new(rom_file);

        mbc.set(0x01f5, 0x0a);
        assert_eq!(mbc.ram_lock, true);

        mbc.set(0x2fff, 0xff);
        assert_eq!(mbc.rom_bank, 0xff);

        mbc.set(0x3000, 0x01);
        assert_eq!(mbc.rom_bank, 0x1ff);

        mbc.set(0xa042, 0x42);

        let data = mbc.get(0xa042);
        assert_eq!(data, 0x42);

        mbc.set(0x01f5, 0x00);
        assert_eq!(mbc.ram_lock, false);
    }

    #[test]
    fn test_mbc5_reg3_get_ram_bank_ff() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc5::new(rom_file);

        mbc.set(0x4f4f, 0xff);
        assert_eq!(mbc.ram_bank, 0x0f);
    }

    #[test]
    fn test_mbc5_write_in_ram() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc5::new(rom_file);

        mbc.set(0x01f5, 0x0a);
        assert_eq!(mbc.ram_lock, true);

        mbc.set(0x4f4f, 0x00);
        assert_eq!(mbc.ram_bank, 0);

        mbc.set(0x0000a630, 0xca);

        let data = mbc.get(0x0000a630);
        assert_eq!(data, 0xca);

        mbc.set(0x01ff, 0x00);
        assert_eq!(mbc.ram_lock, false);
    }

    #[test]
    fn test_mbc5_change_ram_bank() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc5::new(rom_file);

        mbc.set(0x01f5, 0x0a);
        assert_eq!(mbc.ram_lock, true);

        mbc.set(0x4f4f, 0x01);
        assert_eq!(mbc.ram_bank, 1);

        mbc.set(0x01ff, 0x00);
        assert_eq!(mbc.ram_lock, false);
    }
}
