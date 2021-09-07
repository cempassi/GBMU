pub mod area;
mod bios;
mod consts;
mod state;

use area::rom::Cartridge;
use area::rom::NoMbc;
use area::{Area, Wram};
use bios::Bios;
use shared::{traits::Bus, Error};
use state::State;

type Rom = Box<dyn Bus<usize, Item = u8, Result = Result<(), Error>, Data = u8>>;

pub struct Memory {
    state: State,
    bios: Bios<'static>,
    wram: Wram,
    rom: Rom,
}

impl Bus<u16> for Memory {
    type Item = Result<u8, Error>;
    type Result = Result<(), Error>;
    type Data = u8;

    fn get(&self, address: u16) -> Self::Item {
        match address {
            consts::BIOS_MIN..=consts::BIOS_MAX if self.state == State::Bios => {
                if let Some(data) = self.bios.get(Area::Rom.relative(address)) {
                    Ok(*data)
                } else {
                    Err(Error::SegmentationFault(address))
                }
            }
            consts::ROM_MIN..=consts::ROM_MAX => Ok(self.rom.get(Area::Rom.relative(address))),
            consts::WRAM_MIN..=consts::WRAM_MAX => Ok(self.wram.get(Area::Wram.relative(address))),
            _ => Err(Error::SegmentationFault(address)),
        }
    }

    fn set(&mut self, address: u16, data: Self::Data) -> Self::Result {
        match address {
            consts::WRAM_MIN..=consts::WRAM_MAX => {
                self.wram.set(Area::Wram.relative(address), data);
                Ok(())
            }
            consts::ROM_MIN..=consts::ROM_MAX => {
                self.wram.set(Area::Rom.relative(address), data);
                Ok(())
            }
            _ => Err(Error::SegmentationFault(address)),
        }
    }
}

impl Memory {
    pub fn new(mbc: Cartridge, data: Vec<u8>) -> Self {
        let rom: Rom = match mbc {
            Cartridge::NoMbc => Box::new(NoMbc::new(data)),
            _ => unimplemented!(),
        };

        Memory {
            state: State::Bios,
            bios: bios::DMG,
            rom,
            wram: Wram::default(),
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            state: State::Bios,
            bios: bios::DMG,
            wram: Wram::default(),
            rom: Box::new(NoMbc::default()),
        }
    }
}

#[cfg(test)]
mod test_memory {
    use super::Memory;
    use shared::traits::Bus;

    #[test]
    fn test_invalid_read() {
        let memory = Memory::default();

        assert!(memory.get(0xfea1).is_err())
    }

    #[test]
    fn test_invalid_write() {
        let mut memory = Memory::default();

        assert!(memory.set(0xfea1, 42).is_err())
    }

    #[test]
    fn test_read_wram() {
        let memory = Memory::default();

        assert!(memory.get(0xc010).is_ok());
    }

    #[test]
    fn test_write_wram() {
        let mut memory = Memory::default();

        assert!(memory.set(0xc010, 42).is_ok());
    }

    #[test]
    fn test_write_read_wram() {
        let mut memory = Memory::default();

        assert!(memory.set(0xc010, 42).is_ok());

        let read = memory.get(0xc010);

        assert!(read.is_ok());
        assert_eq!(read.unwrap(), 42);
    }
}
