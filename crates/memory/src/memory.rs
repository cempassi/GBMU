use crate::area::Area;
use crate::consts;
use crate::interface::{Bios, BiosDefault, Rom, RomDefault, Wram};
use crate::state::State;
use shared::Error;

#[derive(Debug)]
pub struct Memory<'a> {
    pub(crate) state: State,
    pub(crate) bios: Bios<'a>,
    pub(crate) rom: Rom,
    pub(crate) wram: Wram,
}

impl Memory<'_> {
    pub fn get(&self, address: u16) -> Result<u8, Error> {
        match address {
            consts::BIOS_MIN..=consts::BIOS_MAX if self.state == State::Bios => {
                if let Some(data) = self.bios.borrow().get(Area::Rom.relative(address)) {
                    Ok(*data)
                } else {
                    Err(Error::SegmentationFault(address))
                }
            }
            consts::ROM_MIN..=consts::ROM_MAX => {
                Ok(self.rom.borrow().get(Area::Rom.relative(address)))
            }
            consts::WRAM_MIN..=consts::WRAM_MAX => {
                Ok(self.wram.borrow().get(Area::Wram.relative(address)))
            }
            _ => Err(Error::SegmentationFault(address)),
        }
    }

    pub fn set(&mut self, address: u16, data: u8) -> Result<(), Error> {
        match address {
            consts::WRAM_MIN..=consts::WRAM_MAX => {
                self.wram
                    .borrow_mut()
                    .set(Area::Wram.relative(address), data);
                Ok(())
            }
            consts::ROM_MIN..=consts::ROM_MAX => {
                self.wram
                    .borrow_mut()
                    .set(Area::Rom.relative(address), data);
                Ok(())
            }
            _ => Err(Error::SegmentationFault(address)),
        }
    }
}

impl Default for Memory<'_> {
    fn default() -> Self {
        Memory {
            state: State::Bios,
            bios: Bios::default(),
            wram: Wram::default(),
            rom: Rom::default(),
        }
    }
}

#[cfg(test)]
mod test_memory {
    #[test]
    fn test_invalid_read() {
        let memory = super::Memory::default();

        assert!(memory.get(0xfea1).is_err())
    }

    #[test]
    fn test_invalid_write() {
        let mut memory = super::Memory::default();

        assert!(memory.set(0xfea1, 42).is_err())
    }

    #[test]
    fn test_read_wram() {
        let memory = super::Memory::default();

        assert!(memory.get(0xc010).is_ok());
    }

    #[test]
    fn test_write_wram() {
        let mut memory = super::Memory::default();

        assert!(memory.set(0xc010, 42).is_ok());
    }

    #[test]
    fn test_write_read_wram() {
        let mut memory = super::Memory::default();

        assert!(memory.set(0xc010, 42).is_ok());

        let read = memory.get(0xc010);

        assert!(read.is_ok());
        assert_eq!(read.unwrap(), 42);
    }
}
