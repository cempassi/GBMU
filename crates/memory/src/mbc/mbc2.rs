use super::bus::MbcBus;
use super::consts;
use crate::MemoryBus;
use shared::Error;

#[derive(Debug)]
pub struct Mbc2 {
    ram_lock: bool,
    data: Vec<u8>,
    /// Max 16 0x00 ..= 0x0f
    rom_bank: u8,
}

impl MbcBus for Mbc2 {
    fn set(&mut self, address: usize, data: u8) -> Result<(), Error> {
        match address {
            consts::MBC2_REG_START..=consts::MBC2_REG_END => self.mbc2_register(address, data),
            consts::MBC_RAM_START..=consts::MBC2_ERAM_END => {
                if self.ram_lock {
                    let offset = self.get_ram_offset(address);
                    self.data[address - offset] = data & 0xf
                }
            } // Else should be undefined behavior Or Err
            _ => return Err(shared::Error::IllegalSet(address, data)),
        };
        Ok(())
    }
}

impl MemoryBus for Mbc2 {
    fn get(&self, address: usize) -> u8 {
        match address {
            consts::MBC_BANK0_START..=consts::MBC_BANK0_END => self.data[address],
            consts::MBC_BANK1_START..=consts::MBC_BANK1_END => {
                self.data[(self.rom_bank as usize * consts::MBC_BANK1_START)
                    + (address - consts::MBC_BANK1_START)]
            }
            consts::MBC_RAM_START..=consts::MBC2_ERAM_END => {
                if self.ram_lock {
                    self.data[address - self.get_ram_offset(address)] & 0xf
                } else {
                    // Should be Undefined behavior or raise an Error
                    0
                }
            }
            _ => unreachable!(),
        }
    }

    fn set(&mut self, address: usize, data: u8) {
        let _ = <Self as MbcBus>::set(self, address, data);
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

    /// If address & 0x100 == 1 : change the rom bank
    /// else if data == MBC_MAGIC_LOCK (0x0a) enable RAM operation by setting the ram lock as true
    fn mbc2_register(&mut self, address: usize, data: u8) {
        if address & consts::MBC2_MAGIC_BYTE != 0 {
            self.rom_bank = if data == 0 { 1 } else { data & 0xf };
        } else {
            self.ram_lock = (data & 0x0f) == consts::MBC_MAGIC_LOCK as u8;
        }
    }

    /// Get the RAM or the Echoes RAM Offset
    fn get_ram_offset(&self, address: usize) -> usize {
        match address {
            consts::MBC_RAM_START..=consts::MBC2_RAM_END => consts::MBC_RAM_START,
            consts::MBC2_ERAM_START..=consts::MBC2_ERAM_END => consts::MBC2_ERAM_START,
            _ => unreachable!(),
        }
    }
}

impl Default for Mbc2 {
    fn default() -> Self {
        Mbc2::new(vec![0; consts::MBC2_MAX_SIZE])
    }
}

#[cfg(test)]
mod mbc2_test {
    use super::Mbc2;
    use super::MbcBus;
    use super::MemoryBus;
    const FILE: &[u8; 65536] = include_bytes!("../../../../roms/Ayakashi.gb");

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

        <Mbc2 as MbcBus>::set(&mut mbc, 0x00ff, 0x0a).unwrap();
        assert_eq!(mbc.ram_lock, true);

        assert_eq!(
            <Mbc2 as MbcBus>::set(&mut mbc, 0x0000a0e0, 0xeb).unwrap(),
            ()
        );

        let data = mbc.get(0x0000a0e0);
        assert_eq!(data, 0x0b);

        <Mbc2 as MbcBus>::set(&mut mbc, 0x00ff, 0x00).unwrap();
        assert_eq!(mbc.ram_lock, false);
    }

    #[test]
    fn test_mbc2_unlock() {
        let rom_file = FILE.to_vec(); // MBC2 + RAM + BATTERY
        let mut mbc = Mbc2::new(rom_file);

        <Mbc2 as MbcBus>::set(&mut mbc, 0x00ff, 0x0a).unwrap();
        assert_eq!(mbc.ram_lock, true);
    }

    #[test]
    fn test_mbc2_lock() {
        let rom_file = FILE.to_vec(); // MBC2 + RAM + BATTERY
        let mut mbc = Mbc2::new(rom_file);

        <Mbc2 as MbcBus>::set(&mut mbc, 0x00ff, 0x0a).unwrap();
        assert_eq!(mbc.ram_lock, true);

        <Mbc2 as MbcBus>::set(&mut mbc, 0x00fb, 0x03).unwrap();
        assert_eq!(mbc.ram_lock, false)
    }

    #[test]
    fn test_mbc2_change_bank_to_0() {
        let rom_file = FILE.to_vec(); // MBC2 + RAM + BATTERY
        let mut mbc = Mbc2::new(rom_file);

        <Mbc2 as MbcBus>::set(&mut mbc, 0x2152, 0x00).unwrap();
        assert_eq!(mbc.rom_bank, 0x01);
    }

    #[test]
    fn test_mbc2_change_bank_0a() {
        let rom_file = FILE.to_vec(); // MBC2 + RAM + BATTERY
        let mut mbc = Mbc2::new(rom_file);

        <Mbc2 as MbcBus>::set(&mut mbc, 0x3f52, 0x1a).unwrap();
        assert_eq!(mbc.rom_bank, 0x0a);
    }

    #[test]
    fn test_mbc2_change_bank_0f() {
        let rom_file = FILE.to_vec(); // MBC2 + RAM + BATTERY
        let mut mbc = Mbc2::new(rom_file);

        <Mbc2 as MbcBus>::set(&mut mbc, 0x2fff, 0xff).unwrap();
        assert_eq!(mbc.rom_bank, 0x0f);
    }

    #[test]
    fn test_mbc2_write_in_ram_a130() {
        let rom_file = FILE.to_vec(); // MBC2 + RAM + BATTERY
        let mut mbc = Mbc2::new(rom_file);

        <Mbc2 as MbcBus>::set(&mut mbc, 0x00f5, 0x0a).unwrap();
        assert_eq!(mbc.ram_lock, true);

        <Mbc2 as MbcBus>::set(&mut mbc, 0x0000a130, 0xca).unwrap();

        let data = mbc.get(0x0000a130);
        assert_eq!(data, 0x0a);

        <Mbc2 as MbcBus>::set(&mut mbc, 0x00ff, 0x00).unwrap();
        assert_eq!(mbc.ram_lock, false);
    }

    #[test]
    fn test_mbc2_read_in_eram_a630() {
        let rom_file = FILE.to_vec(); // MBC2 + RAM + BATTERY
        let mut mbc = Mbc2::new(rom_file);

        <Mbc2 as MbcBus>::set(&mut mbc, 0x00f5, 0x0a).unwrap();
        assert_eq!(mbc.ram_lock, true);

        let data = mbc.get(0x0000a630);
        assert_eq!(data, 0x0f);

        <Mbc2 as MbcBus>::set(&mut mbc, 0x00ff, 0x00).unwrap();
        assert_eq!(mbc.ram_lock, false);
    }

    #[test]
    fn test_mbc2_change_bank_13() {
        let rom_file = FILE.to_vec(); // MBC2 + RAM + BATTERY
        let mut mbc = Mbc2::new(rom_file);

        <Mbc2 as MbcBus>::set(&mut mbc, 0x00f5, 0x0a).unwrap();
        assert_eq!(mbc.ram_lock, true);

        <Mbc2 as MbcBus>::set(&mut mbc, 0x1f4b, 0x0d).unwrap();
        assert_eq!(mbc.rom_bank, 13);

        <Mbc2 as MbcBus>::set(&mut mbc, 0x00ff, 0x00).unwrap();
        assert_eq!(mbc.ram_lock, false);
    }
}
