use super::consts;
use shared::{traits::Bus, Error};

pub struct Mbc1 {
    ram_lock: bool,
    bank_mode: bool, // 0 false = rom mode / 1 true = ram mode
    data: Vec<u8>,
    rom_bank: u8,
    ram_bank: u8,
}

impl Bus<usize> for Mbc1 {
    type Item = u8;
    type Result = Result<(), Error>;
    type Data = u8;

    fn get(&self, address: usize) -> Self::Item {
        match address {
            consts::MBC_BANK0_START..=consts::MBC_BANK0_END => self.data[address],
            consts::MBC_BANK1_START..=consts::MBC_RAM_END => self.swap_bank_nbr(address),
            _ => unreachable!(),
        }
    }

    fn set(&mut self, address: usize, data: Self::Data) -> Self::Result {
        match address {
            consts::MBC1_REG0_START..=consts::MBC1_REG0_END => self.update_ram_lock(data), // enable RAM REG0
            consts::MBC1_REG1_START..=consts::MBC1_REG2_END => self.update_bank_nbr(address, data), // change bank nbr REG1 REG2
            consts::MBC1_REG3_START..=consts::MBC1_REG3_END => self.update_bank_mode(data), // change RAM bank nbr if  REG3
            consts::MBC_RAM_START..=consts::MBC_RAM_END => self.write_ram_bank(address, data),
            _ => Err(shared::Error::IllegalSet(address, data)),
        }
    }
}

impl Mbc1 {
    pub fn new(data: Vec<u8>) -> Self {
        Mbc1 {
            ram_lock: false,
            bank_mode: false,
            data,
            rom_bank: 0,
            ram_bank: 0,
        }
    }

    fn write_ram_bank(&mut self, address: usize, data: u8) -> Result<(), Error> {
        if !self.ram_lock {
            return Err(shared::Error::IllegalSet(address, data));
        }
        let bank_nbr = if self.bank_mode {
            self.ram_bank as usize
        } else {
            0
        };
        let index = (bank_nbr * consts::MBC1_RAM_BASE) | (address & consts::MBC1_RAM_OFFSET);
        self.data[index] = data;
        Ok(())
    }

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
                    0
                };
                (bank_nbr, consts::MBC1_RAM_BASE, consts::MBC_RAM_START)
            }
            _ => unreachable!(),
        };
        let index = ((bank_nbr * start_off) + (address - end_off)) & (self.data.len() - 1);
        self.data[index]
    }

    fn update_bank_mode(&mut self, data: u8) -> Result<(), Error> {
        self.bank_mode = match data & 0x01 {
            // only lsb matter
            0 => false,
            1 => true,
            _ => unreachable!(),
        };
        Ok(())
    }

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

    fn update_ram_lock(&mut self, data: u8) -> Result<(), Error> {
        self.ram_lock = data == consts::MBC1_MAGIC_LOCK;
        Ok(())
    }
}

impl Default for Mbc1 {
    fn default() -> Self {
        Mbc1::new(vec![0; consts::MBC1_MAX_SIZE])
    }
}

#[cfg(test)]
mod mbc1_test {
    use super::Mbc1;
    use shared::traits::Bus;

    const FILE: &[u8; 262144] =
        include_bytes!("../../../../../../roms/Metroid II - Return of Samus.gb");

    #[test]
    fn test_mbc1_get() {
        let rom_file = FILE.to_vec();
        let mbc = Mbc1::new(rom_file);
        let data = mbc.get(0x00000000);
        assert_eq!(data, 0xc3);
    }

    #[test]
    fn test_mbc1_set_lock() {
        let rom_file = FILE.to_vec(); // MBC1 + RAM + BATTERY
        let mut mbc = Mbc1::new(rom_file);

        // assert_eq!(mbc.data[0x149], 0x02);

        // assert_eq!(mbc.ram, vec!(0, 0));

        mbc.set(0x01f5, 0x0a).unwrap();
        assert_eq!(mbc.ram_lock, true);

        mbc.set(0x034b, 0x03).unwrap();
        assert_eq!(mbc.ram_lock, false)
    }

    #[test]
    fn test_mbc1_bank_mod() {
        let rom_file = FILE.to_vec(); // MBC1 + RAM + BATTERY
        let mut mbc = Mbc1::new(rom_file);

        mbc.set(0x6abc, 4).unwrap();
        assert_eq!(mbc.bank_mode, false);

        mbc.set(0x7abc, 3).unwrap();
        assert_eq!(mbc.bank_mode, true);
    }

    #[test]
    fn test_mbc1_reg1_0() {
        let rom_file = FILE.to_vec(); // MBC1 + RAM + BATTERY
        let mut mbc = Mbc1::new(rom_file);

        mbc.set(0x2156, 0x00).unwrap();
        assert_eq!(mbc.rom_bank, 0x01);
    }

    #[test]
    fn test_mbc1_reg1_1a() {
        let rom_file = FILE.to_vec(); // MBC1 + RAM + BATTERY
        let mut mbc = Mbc1::new(rom_file);

        mbc.set(0x2156, 0x1a).unwrap();
        assert_eq!(mbc.rom_bank, 0x1a);
    }

    #[test]
    fn test_mbc1_reg1_14() {
        let rom_file = FILE.to_vec(); // MBC1 + RAM + BATTERY
        let mut mbc = Mbc1::new(rom_file);

        mbc.set(0x2fff, 0x14).unwrap();
        assert_eq!(mbc.rom_bank, 0x15);
    }

    #[test]
    fn test_mbc1_reg2_1_28() {
        let rom_file = FILE.to_vec(); // MBC1 + RAM + BATTERY
        let mut mbc = Mbc1::new(rom_file);

        mbc.set(0x4f4f, 0x01).unwrap();
        assert_eq!(mbc.rom_bank, 0x20);
        mbc.set(0x2fff, 0x08).unwrap();
        assert_eq!(mbc.rom_bank, 0x29);
    }

    #[test]
    fn test_mbc1_reg2_1_3c() {
        let rom_file = FILE.to_vec(); // MBC1 + RAM + BATTERY
        let mut mbc = Mbc1::new(rom_file);

        mbc.set(0x4f4f, 0x01).unwrap();
        assert_eq!(mbc.rom_bank, 0x20);
        mbc.set(0x2fff, 0x1c).unwrap();
        assert_eq!(mbc.rom_bank, 0x3d);
    }

    #[test]
    fn test_mbc1_reg2_1_14() {
        let rom_file = FILE.to_vec(); // MBC1 + RAM + BATTERY
        let mut mbc = Mbc1::new(rom_file);

        mbc.set(0x4f4f, 0x03).unwrap();
        assert_eq!(mbc.rom_bank, 0x60);
        mbc.set(0x2fff, 0x14).unwrap();
        assert_eq!(mbc.rom_bank, 0x74);
    }

    #[test]
    fn test_mbc1_get_last_rom_bank() {
        let rom_file = FILE.to_vec(); // MBC1 + RAM + BATTERY
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
        let rom_file = FILE.to_vec(); // MBC1 + RAM + BATTERY
        let mut mbc = Mbc1::new(rom_file);

        mbc.set(0x01f5, 0x0a).unwrap();
        assert_eq!(mbc.ram_lock, true);

        mbc.set(0x7abc, 3).unwrap(); // enable bank_mode
        assert_eq!(mbc.bank_mode, true);

        mbc.set(0x4f4f, 0x00).unwrap();
        assert_eq!(mbc.ram_bank, 0);

        mbc.set(0x0000a630, 0xca).unwrap();

        let data = mbc.get(0x0000a630);
        assert_eq!(data, 0xca);

        mbc.set(0x01ff, 0x00).unwrap();
        assert_eq!(mbc.ram_lock, false);
    }

    #[test]
    fn test_mbc1_change_ram_bank() {
        let rom_file = FILE.to_vec(); // MBC1 + RAM + BATTERY
        let mut mbc = Mbc1::new(rom_file);

        mbc.set(0x01f5, 0x0a).unwrap();
        assert_eq!(mbc.ram_lock, true);

        mbc.set(0x7abc, 3).unwrap(); // enable bank_mode
        assert_eq!(mbc.bank_mode, true);

        mbc.set(0x4f4f, 0x01).unwrap();
        assert_eq!(mbc.ram_bank, 1);

        mbc.set(0x01ff, 0x00).unwrap();
        assert_eq!(mbc.ram_lock, false);
    }
}
