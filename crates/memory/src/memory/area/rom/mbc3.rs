use super::consts;
use shared::{traits::Bus, Error};

pub struct Mbc3 {
    ram_lock: bool,
    rtc_lock: bool,
    latch: bool,
    data: Vec<u8>,
    rom_bank: u8,
    ram_bank: u8,
    rtc: Mbc3Rtc,
}

/// [Name]    [Range]  [Id]    [Description]
/// Seconds    0-59    0x08
/// Minutes    0-59    0x09
/// Hours      0-23    0x0A
/// DC Lower   0-255   0x0B    The lower 8 bits of the Day Counter
/// DC Upper           0x0C    bit 0 => 9th bit of the Day Counter, bit 6 => Halt, bit 7 => Day Counter Carry Bit
struct Mbc3Rtc {
    seconds: u8,
    minutes: u8,
    hours: u8,
    dc_lower: u8,
    dc_upper: u8,
    epoch: u64,
}

fn get_epoch() -> u64 {
    // Return the epoch in microseconds.
    let epoch = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Couldn't get epoch");
    (epoch.as_micros() as u64) / 1_000_000
}

impl Bus<usize> for Mbc3 {
    type Item = u8;
    type Result = Result<(), Error>;
    type Data = u8;

    fn get(&self, address: usize) -> Self::Item {
        match address {
            consts::MBC_BANK0_START..=consts::MBC_BANK0_END => self.data[address],
            consts::MBC_BANK1_START..=consts::MBC_BANK1_END => {
                self.data[(self.rom_bank as usize * consts::MBC_BANK1_START)
                    + (address - consts::MBC_BANK1_START)]
            }
            consts::MBC_RAM_START..=consts::MBC_RAM_END => self.get_ram(address),
            _ => unreachable!(),
        }
    }

    fn set(&mut self, address: usize, data: Self::Data) -> Self::Result {
        match address {
            consts::MBC3_REG0_START..=consts::MBC3_REG0_END => self.update_ram_rtc_lock(data), // enable RAM / RTC REG0
            consts::MBC3_REG1_START..=consts::MBC3_REG1_END => self.update_rom_bank_nbr(data), // change ROM bank nbr REG1
            consts::MBC3_REG2_START..=consts::MBC3_REG2_END => self.update_ram_bank_rtc_reg(data), // change RAN bank OR RTC register nbr
            consts::MBC3_REG3_START..=consts::MBC3_REG3_END => self.latch_rtc_register(data), // latch la struct rtc dans la ram pour la lire
            consts::MBC3_REG4_START..=consts::MBC3_REG4_END => self.write_ram_bank(address, data),
            _ => Err(shared::Error::IllegalSet(address, data)),
        }
    }
}

impl Mbc3 {
    pub fn new(data: Vec<u8>) -> Self {
        Mbc3 {
            ram_lock: false,
            rtc_lock: false,
            latch: false,
            data,
            rom_bank: 0,
            ram_bank: 0,
            rtc: Mbc3Rtc {
                seconds: 0,
                minutes: 0,
                hours: 0,
                dc_lower: 0,
                dc_upper: 0,
                epoch: 0,
            },
        }
    }

    fn get_ram(&self, address: usize) -> u8 {
        if !self.ram_lock {
            return 0;
        }
        match self.ram_bank {
            0x00..=0x03 => {
                self.data[(self.ram_bank as usize * consts::MBC3_RAM_BASE)
                    | (address - consts::MBC3_RAM_OFFSET)]
            }
            0x08 => self.rtc.seconds,
            0x09 => self.rtc.minutes,
            0x0a => self.rtc.hours,
            0x0b => self.rtc.dc_lower,
            0x0c => self.rtc.dc_upper,
            _ => self.data[self.ram_bank as usize - consts::MBC3_RTC_OFFSET],
        }
    }

    fn latch(&mut self) {
        let new_epoch = if self.rtc.dc_upper & 0x40 == 0 {
            get_epoch()
        } else {
            self.rtc.epoch
        };
        let elapsed = new_epoch - self.rtc.epoch;

        let last_day = self.get_days();
        let last_secs = self.rtc_to_epoch();
        self.epoch_to_rtc(last_secs + elapsed);
        let new_day = self.get_days();

        // Overflow
        if new_day < last_day {
            self.rtc.dc_upper |= 0x80;
        }

        dbg!(
            "Latching RTC: {:04}/{:02}:{:02}:{:02}",
            self.get_days(),
            self.rtc.hours,
            self.rtc.minutes,
            self.rtc.seconds
        );

        self.rtc.epoch = new_epoch;
    }

    fn latch_rtc_register(&mut self, data: u8) -> Result<(), Error> {
        if self.latch {
            if data == 0x1 {
                self.latch();
            }
            self.latch = false;
        } else if data == 0 {
            self.latch = true;
        }
        Ok(())
    }

    fn write_ram_bank(&mut self, address: usize, data: u8) -> Result<(), Error> {
        if !self.ram_lock {
            return Err(shared::Error::IllegalSet(address, data));
        }
        if self.ram_bank <= 0x03 {
            self.data[(self.ram_bank as usize * consts::MBC3_RAM_BASE)
                | (address - consts::MBC3_RAM_OFFSET)] = data;
            return Ok(());
        }
        match self.ram_bank {
            0x08 => {
                self.rtc.seconds = data;
            }
            0x09 => {
                self.rtc.minutes = data;
            }
            0x0a => {
                self.rtc.hours = data;
            }
            0x0b => {
                self.rtc.dc_lower = data;
            }
            0x0c => {
                self.rtc.dc_upper = data;
            }
            _ => return Err(shared::Error::IllegalSet(address, data)), //Ok(self.data[self.ram_bank - consts::MBC3_RTC_OFFSET] = data),
        }
        self.rtc.epoch = get_epoch();
        Ok(())
    }

    fn update_ram_bank_rtc_reg(&mut self, data: u8) -> Result<(), Error> {
        self.ram_bank = data & 0x0f;
        Ok(())
    }

    fn update_rom_bank_nbr(&mut self, data: u8) -> Result<(), Error> {
        self.rom_bank = match data & 0x7f {
            0 => 1,
            _nbr => _nbr,
        };
        Ok(())
    }

    fn update_ram_rtc_lock(&mut self, data: u8) -> Result<(), Error> {
        self.ram_lock = data == consts::MBC3_MAGIC_LOCK;
        self.rtc_lock = data == consts::MBC3_MAGIC_LOCK;
        Ok(())
    }

    fn get_days(&mut self) -> u64 {
        ((self.rtc.dc_upper as u64 & 1) << 8) & self.rtc.dc_lower as u64
    }

    fn rtc_to_epoch(&mut self) -> u64 {
        let sec = self.rtc.seconds as u64;
        let min = self.rtc.minutes as u64;
        let hours = self.rtc.hours as u64;
        let days = self.get_days();
        (days * 24 + hours) * 3600 + min * 60 + sec
    }

    fn epoch_to_rtc(&mut self, epoch: u64) {
        self.rtc.seconds = (epoch % 60) as u8;
        self.rtc.minutes = ((epoch / 60) % 60) as u8;
        self.rtc.hours = ((epoch / 3600) % 24) as u8;
        let day = epoch / (3600 * 24);
        self.rtc.dc_lower = day as u8;
        self.rtc.dc_upper = (self.rtc.dc_upper & !1) | ((day >> 8) & 1) as u8;
    }
}

impl Default for Mbc3 {
    fn default() -> Self {
        Mbc3::new(vec![0; consts::MBC3_MAX_SIZE])
    }
}

#[cfg(test)]
mod mbc3_test {
    use super::Mbc3;
    use shared::traits::Bus;

    const FILE: &[u8; 1048576] = include_bytes!(
        "../../../../../../roms/Mary-Kate and Ashley - Pocket Planner (USA, Europe).gbc"
    );

    #[test]
    fn test_mbc3_get_0x0() {
        let rom_file = FILE.to_vec();
        let mbc = Mbc3::new(rom_file);
        let data = mbc.get(0x00000040);
        assert_eq!(data, 0xc3);
    }

    #[test]
    fn test_mbc3_set_lock() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc3::new(rom_file);

        assert_eq!(mbc.data[0x147], 0x10);

        mbc.set(0x01f5, 0x0a).unwrap();
        assert_eq!(mbc.ram_lock, true);

        mbc.set(0x034b, 0x03).unwrap();
        assert_eq!(mbc.ram_lock, false)
    }

    #[test]
    fn test_mbc3_reg1_0() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc3::new(rom_file);

        mbc.set(0x2156, 0x00).unwrap();
        assert_eq!(mbc.rom_bank, 0x01);
    }

    #[test]
    fn test_mbc3_reg1_1a() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc3::new(rom_file);

        mbc.set(0x2156, 0x1a).unwrap();
        assert_eq!(mbc.rom_bank, 0x1a);
    }

    #[test]
    fn test_mbc3_reg1_14() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc3::new(rom_file);

        mbc.set(0x2fff, 0x14).unwrap();
        assert_eq!(mbc.rom_bank, 0x14);
    }

    #[test]
    fn test_mbc3_get_last_rom_bank() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc3::new(rom_file);

        mbc.set(0x2fff, 0x1f).unwrap();
        assert_eq!(mbc.rom_bank, 0x1f);
    }

    #[test]
    fn test_mbc3_change_rom_bank() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc3::new(rom_file);

        mbc.set(0x3564, 0x11).unwrap();
        assert_eq!(mbc.rom_bank, 0x11);
    }

    #[test]
    fn test_mbc3_write_in_ram() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc3::new(rom_file);

        mbc.set(0x01f5, 0x0a).unwrap();
        assert_eq!(mbc.ram_lock, true);

        mbc.set(0x4f4f, 0x00).unwrap();
        assert_eq!(mbc.ram_bank, 0);

        mbc.set(0x0000a010, 0xca).unwrap();

        let data = mbc.get(0x0000a010);
        assert_eq!(data, 0xca);

        mbc.set(0x01ff, 0x00).unwrap();
        assert_eq!(mbc.ram_lock, false);
    }

    #[test]
    fn test_mbc3_change_ram_bank() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc3::new(rom_file);

        mbc.set(0x01f5, 0x0a).unwrap();
        assert_eq!(mbc.ram_lock, true);

        mbc.set(0x4f4f, 0x01).unwrap();
        assert_eq!(mbc.ram_bank, 1);

        mbc.set(0x01ff, 0x00).unwrap();
        assert_eq!(mbc.ram_lock, false);
    }

    #[test]
    fn test_mbc3_set_seconds() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc3::new(rom_file);

        mbc.set(0x01f5, 0x0a).unwrap();
        assert_eq!(mbc.ram_lock, true);

        mbc.set(0x4f4f, 0x08).unwrap();
        assert_eq!(mbc.ram_bank, 8);

        mbc.set(0x0aff, 0x28).unwrap();

        let data = mbc.get(0x0aff);
        assert_eq!(data, 0xea);

        mbc.set(0x01ff, 0x00).unwrap();
        assert_eq!(mbc.ram_lock, false);
    }

    #[test]
    fn test_mbc3_set_minutes() {
        let rom_file = FILE.to_vec();
        let mut mbc = Mbc3::new(rom_file);

        mbc.set(0x01f5, 0x0a).unwrap();
        assert_eq!(mbc.ram_lock, true);

        mbc.set(0x4f4f, 0x09).unwrap();
        assert_eq!(mbc.ram_bank, 9);

        mbc.set(0x0aff, 0x29).unwrap();

        let data = mbc.get(0x0aff);
        assert_eq!(data, 0xea);

        mbc.set(0x01ff, 0x00).unwrap();
        assert_eq!(mbc.ram_lock, false);
    }

    // #[test]
    // fn test_mbc3_set_hours() {
    //     let rom_file = FILE.to_vec();
    //     let mut mbc = Mbc3::new(rom_file);
    //
    //     mbc.set(0x01f5, 0x0a).unwrap();
    //     assert_eq!(mbc.ram_lock, true);
    //
    //     mbc.set(0x4f4f, 0x0a).unwrap();
    //     assert_eq!(mbc.ram_bank, 10);
    //
    //     mbc.set(0x0aff, 0x30).unwrap();
    //
    //     let data = mbc.get(0x0aff);
    //     assert_eq!(data, 0x30);
    //
    //     mbc.set(0x01ff, 0x00).unwrap();
    //     assert_eq!(mbc.ram_lock, false);
    // }
    //
    // #[test]
    // fn test_mbc3_set_dc_lower() {
    //     let rom_file = FILE.to_vec();
    //     let mut mbc = Mbc3::new(rom_file);
    //
    //     mbc.set(0x01f5, 0x0a).unwrap();
    //     assert_eq!(mbc.ram_lock, true);
    //
    //     mbc.set(0x4f4f, 0x0b).unwrap();
    //     assert_eq!(mbc.ram_bank, 11);
    //
    //     mbc.set(0x0aff, 0x31).unwrap();
    //
    //     let data = mbc.get(0x0aff);
    //     assert_eq!(data, 0x31);
    //
    //     mbc.set(0x01ff, 0x00).unwrap();
    //     assert_eq!(mbc.ram_lock, false);
    // }

    // #[test]
    // fn test_mbc3_set_dc_upper() {
    //     let rom_file = FILE.to_vec();
    //     let mut mbc = Mbc3::new(rom_file);
    //
    //     mbc.set(0x01f5, 0x0a).unwrap();
    //     assert_eq!(mbc.ram_lock, true);
    //
    //     mbc.set(0x4f4f, 0x0c).unwrap();
    //     assert_eq!(mbc.ram_bank, 12);
    //
    //     mbc.set(0x0aff, 0x0a).unwrap();
    //
    //     let data = mbc.get(0x0aff);
    //     assert_eq!(data, 0x0a);
    //
    //     mbc.set(0x01ff, 0x00).unwrap();
    //     assert_eq!(mbc.ram_lock, false);
    // }
    //
    // #[test]
    // fn test_mbc3_get_dc_upper() {
    //     let rom_file = FILE.to_vec();
    //     let mut mbc = Mbc3::new(rom_file);
    //
    //     mbc.set(0x01f5, 0x0a).unwrap();
    //     assert_eq!(mbc.ram_lock, true);
    //
    //     mbc.set(0x4f4f, 0x0c).unwrap();
    //     assert_eq!(mbc.ram_bank, 12);
    //
    //     let data = mbc.get(0x0aff);
    //     assert_eq!(data, 0x30);
    //
    //     mbc.set(0x01ff, 0x00).unwrap();
    //     assert_eq!(mbc.ram_lock, false);
    // }
    //
    // #[test]
    // fn test_mbc3_get_seconds() {
    //     let rom_file = FILE.to_vec();
    //     let mut mbc = Mbc3::new(rom_file);
    //
    //     mbc.set(0x01f5, 0x0a).unwrap();
    //     assert_eq!(mbc.ram_lock, true);
    //
    //     mbc.set(0x4f4f, 0x08).unwrap();
    //     assert_eq!(mbc.ram_bank, 8);
    //
    //     let data = mbc.get(0x0aff);
    //     assert_eq!(data, 0xea);
    //
    //     mbc.set(0x01ff, 0x00).unwrap();
    //     assert_eq!(mbc.ram_lock, false);
    // }
    //
    // #[test]
    // fn test_mbc3_get_minutes() {
    //     let rom_file = FILE.to_vec();
    //     let mut mbc = Mbc3::new(rom_file);
    //
    //     mbc.set(0x01f5, 0x0a).unwrap();
    //     assert_eq!(mbc.ram_lock, true);
    //
    //     mbc.set(0x4f4f, 0x09).unwrap();
    //     assert_eq!(mbc.ram_bank, 9);
    //
    //     let data = mbc.get(0x0aff);
    //     assert_eq!(data, 0xea);
    //
    //     mbc.set(0x01ff, 0x00).unwrap();
    //     assert_eq!(mbc.ram_lock, false);
    // }
    //
    // #[test]
    // fn test_mbc3_get_hours() {
    //     let rom_file = FILE.to_vec();
    //     let mut mbc = Mbc3::new(rom_file);
    //
    //     mbc.set(0x01f5, 0x0a).unwrap();
    //     assert_eq!(mbc.ram_lock, true);
    //
    //     mbc.set(0x4f4f, 0x0a).unwrap();
    //     assert_eq!(mbc.ram_bank, 10);
    //
    //     let data = mbc.get(0x0aff);
    //     assert_eq!(data, 0x30);
    //
    //     mbc.set(0x01ff, 0x00).unwrap();
    //     assert_eq!(mbc.ram_lock, false);
    // }
    //
    // #[test]
    // fn test_mbc3_get_dc_lower() {
    //     let rom_file = FILE.to_vec();
    //     let mut mbc = Mbc3::new(rom_file);
    //
    //     mbc.set(0x01f5, 0x0a).unwrap();
    //     assert_eq!(mbc.ram_lock, true);
    //
    //     mbc.set(0x4f4f, 0x0b).unwrap();
    //     assert_eq!(mbc.ram_bank, 11);
    //
    //     let data = mbc.get(0x0bff);
    //     assert_eq!(data, 0xea);
    //
    //     mbc.set(0x01ff, 0x00).unwrap();
    //     assert_eq!(mbc.ram_lock, false);
    // }
    //
    // #[test]
    // fn test_mbc3_latch_rtc() {
    //     let rom_file = FILE.to_vec();
    //     let mut mbc = Mbc3::new(rom_file);
    // }
}
