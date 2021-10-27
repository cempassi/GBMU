use std::cell::RefCell;
use std::rc::Rc;

use super::mbc::{Cartridge, Mbc0, Mbc1, Mbc2, Mbc3, Mbc5};
use crate::area::Area;
use crate::bios::Bios;
use crate::bus::MemoryBus;
use crate::consts;
use crate::interface::{Bus, Rom};
use crate::mbc::default::RomDefault;
use crate::state;
use crate::ram::Ram;
use ppu::Ppu;
use shared::Error;
use crate::io::IO;

#[derive(Debug)]
pub struct Memory {
    pub(crate) state: state::Rom,
    pub(crate) bios: Bus,
    pub(crate) rom: Rom,
    pub(crate) wram: Bus,
    pub(crate) ppu: Ppu,
    pub(crate) hram: Bus,
    pub(crate) io: IO,
    pub(crate) ie: u8
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            state: state::Rom::Bios,
            bios: Rc::new(RefCell::new(Box::new(Bios::new()))),
            wram: Rc::new(RefCell::new(Box::new(Ram::default()))),
            ppu: Ppu::default(),
            rom: Rom::default(),
            io: IO::default(),
            hram: Rc::new(RefCell::new(Box::new(Ram::new(127)))),
            ie: 0
        }
    }
}

impl Memory {
    pub fn get_u8(&self, address: u16) -> Result<u8, Error> {
        match address {
            consts::BIOS_MIN..=consts::BIOS_MAX if self.state == state::Rom::Bios => {
                Ok(self.bios.borrow().get(Area::Bios.relative(address)))
            }
            consts::ROM_MIN..=consts::ROM_MAX => {
                Ok(self.rom.borrow().get(Area::Rom.relative(address)))
            }
            consts::VRAM_MIN..=consts::VRAM_MAX => Ok(self.ppu.borrow().get(address.into())),
            consts::WRAM_MIN..=consts::WRAM_MAX => {
                Ok(self.wram.borrow().get(Area::Wram.relative(address)))
            }
            consts::IOREG_MIN..=consts::IOREM_MAX => Ok(self.io.get(address)),
            consts::HRAM_MIN..=consts::HRAM_MAX => {
                Ok(self.hram.borrow().get(Area::Hram.relative(address)))
            }
            consts::INTERUPT_ENABLE => {
                Ok(self.ie)
            }
            _ => Err(Error::InvalidGet(address)),
        }
    }

    pub fn set_u8(&mut self, address: u16, data: u8) -> Result<(), Error> {
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
            consts::VRAM_MIN..=consts::VRAM_MAX => {
                self.ppu.borrow_mut().set(address.into(), data);
                Ok(())
            }
            consts::IOREG_MIN..=consts::IOREM_MAX => {
                self.io.set(address, data);
                Ok(())
            }
            consts::HRAM_MIN..=consts::HRAM_MAX => {
                self.hram
                    .borrow_mut()
                    .set(Area::Hram.relative(address), data);
                Ok(())
            }
            consts::INTERUPT_ENABLE => {
                self.ie = data;
                Ok(())
            }
            _ => Err(Error::InvalidSet(address, data)),
        }
    }

    pub fn get_u16(&self, address: u16) -> Result<u16, Error> {
        match self.get_u8(address) {
            Ok(left) => match self.get_u8(address + 1) {
                Ok(right) => Ok((right as u16) << 8 | left as u16),
                Err(error) => Err(error),
            },
            Err(error) => Err(error),
        }
    }

    pub fn set_u16(&mut self, address: u16, data: u16) -> Result<(), Error> {
        match self.set_u8(address, data as u8) {
            Ok(_) => match self.set_u8(address + 1, (data >> 8) as u8) {
                Ok(_) => Ok(()),
                Err(error) => Err(error),
            },
            Err(error) => Err(error),
        }
    }

    pub fn get_area(&self, area: Area) -> Bus {
        match area {
            Area::Bios => self.bios.clone(),
            Area::Rom => todo!(),
            Area::Vram | Area::_ExtRam => todo!(),
            Area::Wram => self.wram.clone(),
            Area::_EchoRam => todo!(),
            Area::_Oam => todo!(),
            Area::IOReg => todo!(),
            Area::Hram => todo!(),
        }
    }

    pub fn get_ppu(&self) -> Ppu {
        self.ppu.clone()
    }
}

impl Memory {
    pub fn new(mbc: Cartridge, data: Vec<u8>) -> Rc<RefCell<Self>> {
        let rom: Rom = Rc::new(RefCell::new(match mbc {
            Cartridge::Mbc0 => Mbc0::new(data),
            Cartridge::Mbc1 => Mbc1::new(data),
            Cartridge::Mbc2 | Cartridge::Mbc2Battery => Mbc2::new(data),
            Cartridge::Mbc3 => Mbc3::new(data),
            Cartridge::Mbc5 => Mbc5::new(data),
            _ => unimplemented!(),
        }));
        let state = state::Rom::Bios;
        let bios: Box<dyn MemoryBus> = Box::new(Bios::new());
        let bios = Rc::new(RefCell::new(bios));
        let wram: Box<dyn MemoryBus> = Box::new(Ram::default());
        let wram = Rc::new(RefCell::new(wram));
        let ppu = Ppu::default();
        let io = IO::default();
        let hram: Box<dyn MemoryBus> = Box::new(Ram::new(consts::HIGH_RAM_SIZE));
        let hram = Rc::new(RefCell::new(hram));
        Rc::new(RefCell::new(Self {
            state,
            bios,
            rom,
            wram,
            ppu,
            io,
            hram,
            ie: 0
        }))
    }
}

#[cfg(test)]
mod test_memory {
    #[test]
    fn test_invalid_read() {
        let memory = super::Memory::default();

        assert!(memory.get_u8(0xfea1).is_err())
    }

    #[test]
    fn test_invalid_write() {
        let mut memory = super::Memory::default();

        assert!(memory.set_u8(0xfea1, 42).is_err())
    }

    #[test]
    fn test_read_wram() {
        let memory = super::Memory::default();

        assert!(memory.get_u8(0xc010).is_ok());
    }

    #[test]
    fn test_write_wram() {
        let mut memory = super::Memory::default();

        assert!(memory.set_u8(0xc010, 42).is_ok());
    }

    #[test]
    fn test_write_read_wram() {
        let mut memory = super::Memory::default();

        assert!(memory.set_u8(0xc010, 42).is_ok());

        let read = memory.get_u8(0xc010);

        assert!(read.is_ok());
        assert_eq!(read.unwrap(), 42);
    }

    #[test]
    fn test_write_read_u16() {
        let mut memory = super::Memory::default();

        assert!(memory.set_u16(0xc010, 0x2242).is_ok());

        let read = memory.get_u16(0xc010);

        assert!(read.is_ok());
        assert_eq!(read.unwrap(), 0x2242);
    }
}
