use std::cell::RefCell;
use std::rc::Rc;

use super::mbc::{Cartridge, Mbc0, Mbc1, Mbc2, Mbc3, Mbc5};
use crate::area::Area;
use crate::bios::Bios;
use crate::bus::MemoryBus;
use crate::consts;
use crate::interface::{Bus, Rom};
use crate::interrupts::Interrupts;
use crate::io::IO;
use crate::mbc::default::RomDefault;
use crate::ram::Ram;
use crate::state::{self, State};
use ppu::Ppu;
use shared::Error;

#[derive(Debug)]
pub struct Memory {
    pub(crate) state: state::State,
    pub(crate) bios: Bus,
    pub(crate) rom: Rom,
    pub(crate) wram: Bus,
    pub(crate) ppu: Ppu,
    pub(crate) hram: Bus,
    pub(crate) io: IO,
    pub(crate) interrupts: Interrupts,
}

impl Default for Memory {
    fn default() -> Self {
        let interrupts = Interrupts::default();
        let raisable = interrupts.get_raisable();
        let io = IO::new(raisable.clone());
        let ppu = Ppu::new(raisable);

        Memory {
            state: state::State::Bios,
            bios: Rc::new(RefCell::new(Box::new(Bios::new()))),
            wram: Rc::new(RefCell::new(Box::new(Ram::default()))),
            ppu,
            rom: Rom::default(),
            io,
            hram: Rc::new(RefCell::new(Box::new(Ram::new(127)))),
            interrupts,
        }
    }
}

impl Memory {
    pub fn get_u8(&self, address: u16) -> Result<u8, Error> {
        match address {
            consts::BIOS_MIN..=consts::BIOS_MAX if self.state == state::State::Bios => {
                self.bios.borrow().get(Area::Bios.relative(address))
            }
            consts::ROM_MIN..=consts::ROM_MAX => self.rom.borrow().get(Area::Rom.relative(address)),
            consts::VRAM_MIN..=consts::VRAM_MAX => self.ppu.borrow().get(address.into()),
            consts::WRAM_MIN..=consts::WRAM_MAX => {
                self.wram.borrow().get(Area::Wram.relative(address))
            }
            consts::ECHO_MIN..=consts::ECHO_MAX => {
                self.wram.borrow().get(Area::EchoRam.relative(address))
            }
            consts::EXT_RAM_MIN..=consts::EXT_RAM_MAX => self.rom.borrow().get(address.into()),
            consts::OAM_MIN..=consts::OAM_MAX => self.ppu.borrow_mut().get(address.into()),
            consts::RESTRICTED_MIN..=consts::RESTRICTED_MAX => Ok(0x00),
            consts::IOREG_MIN..=consts::IOREM_MAX => self.get_io(address),
            consts::HRAM_MIN..=consts::HRAM_MAX => {
                self.hram.borrow().get(Area::Hram.relative(address))
            }
            consts::INTERRUPT_ENABLED => self.interrupts.get_enabled(),
        }
    }

    fn get_io(&self, address: u16) -> Result<u8, Error> {
        match address {
            consts::LCD_CONTROL..=consts::LY_COMPARE => self.ppu.borrow_mut().get(address.into()),
            consts::YWINDOW | consts::XWINDOW | consts::BGP => {
                self.ppu.borrow_mut().get(address.into())
            }
            consts::INTERRUPT_FLAGS => self.interrupts.get_requested(),
            _ => Ok(self.io.get(address)),
        }
    }

    pub fn set_u8(&mut self, address: u16, data: u8) -> Result<(), Error> {
        match address {
            consts::ROM_MIN..=consts::ROM_MAX => {
                self.rom.borrow_mut().set(Area::Rom.relative(address), data)
            }
            consts::VRAM_MIN..=consts::VRAM_MAX => self.ppu.borrow_mut().set(address.into(), data),
            consts::EXT_RAM_MIN..=consts::EXT_RAM_MAX => {
                self.rom.borrow_mut().set(address.into(), data)
            }
            consts::WRAM_MIN..=consts::WRAM_MAX => self
                .wram
                .borrow_mut()
                .set(Area::Wram.relative(address), data),
            consts::ECHO_MIN..=consts::ECHO_MAX => self
                .wram
                .borrow_mut()
                .set(Area::EchoRam.relative(address), data),
            consts::OAM_MIN..=consts::OAM_MAX => self.ppu.borrow_mut().set(address.into(), data),
            consts::RESTRICTED_MIN..=consts::RESTRICTED_MAX => Ok(()),
            consts::IOREG_MIN..=consts::IOREM_MAX => self.set_io(address, data),
            consts::HRAM_MIN..=consts::HRAM_MAX => self
                .hram
                .borrow_mut()
                .set(Area::Hram.relative(address), data),
            consts::INTERRUPT_ENABLED => self.interrupts.set_enabled(data),
        }
    }

    fn set_io(&mut self, address: u16, data: u8) -> Result<(), Error> {
        match address {
            consts::LCD_CONTROL..=consts::LY_COMPARE => {
                self.ppu.borrow_mut().set(address.into(), data)
            }
            consts::YWINDOW | consts::XWINDOW | consts::BGP => {
                self.ppu.borrow_mut().set(address.into(), data)
            }
            consts::INTERRUPT_FLAGS => self.interrupts.set_requested(data),
            _ => {
                self.io.set(address, data);
                Ok(())
            }
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
            Area::IOReg => todo!(),
            Area::Hram => todo!(),
            Area::EchoRam => todo!(),
        }
    }

    pub fn get_ppu(&self) -> Ppu {
        self.ppu.clone()
    }

    pub fn get_rom(&self) -> Rom {
        self.rom.clone()
    }

    pub fn clock_tick(&mut self) {
        self.io.tick()
    }

    pub fn get_debug(&mut self) -> Option<char> {
        if self.io.get(consts::SERIAL_CONTROL) == 0x81 {
            let data = self.io.get(consts::SERIAL_DATA);
            self.io.set(consts::SERIAL_CONTROL, 0);
            char::from_u32(data as u32)
        } else {
            None
        }
    }
}

impl Memory {
    pub fn new(mbc: Cartridge, data: Vec<u8>, state: State) -> Rc<RefCell<Self>> {
        let rom: Rom = Rc::new(RefCell::new(match mbc {
            Cartridge::Mbc0 => Mbc0::new(data),
            Cartridge::Mbc1 => Mbc1::new(data),
            Cartridge::Mbc2 | Cartridge::Mbc2Battery => Mbc2::new(data),
            Cartridge::Mbc3 => Mbc3::new(data),
            Cartridge::Mbc5 => Mbc5::new(data),
            _ => unimplemented!(),
        }));
        // Init state
        let state = state;

        // Init Bios
        let bios: Box<dyn MemoryBus> = Box::new(Bios::new());
        let bios = Rc::new(RefCell::new(bios));

        // Init Wram
        let wram: Box<dyn MemoryBus> = Box::new(Ram::default());
        let wram = Rc::new(RefCell::new(wram));

        // Init Interrupts first as several IO need them
        let interrupts = Interrupts::default();

        // Get Requested memory spaces shared between componnents
        let requested = interrupts.get_raisable();

        // Create io registers (Timer)
        let io = IO::new(requested.clone());

        // Create memory spaces with fully-qualified syntax
        let ppu = match state {
            State::Bios => Ppu::new(requested),
            State::Rom => Ppu::no_bios(requested),
        };

        // Init Hram
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
            interrupts,
        }))
    }
}

// Interrupt interface
impl Memory {
    pub fn master_enabled(&self) -> bool {
        self.interrupts.master_enabled()
    }

    pub fn set_master_enabled(&mut self) {
        self.interrupts.set_master_enabled();
    }

    pub fn disable_master_enabled(&mut self) -> u8 {
        self.interrupts.disable_master_enabled();
        0
    }

    pub fn set_is_interrupted(&mut self) -> u8 {
        self.interrupts.set_is_interrupted();
        0
    }

    pub fn disable_is_interruped(&mut self) -> u8 {
        self.interrupts.disabled_is_interrupted();
        0
    }

    pub fn get_interrupt_address(&mut self) -> Option<u16> {
        self.interrupts.get_address()
    }

    pub fn is_requested(&self) -> bool {
        self.interrupts.is_requested()
    }

    /// Check if EI instruction was called, set interrupt if it was
    pub fn check_is_interrupted(&mut self) {
        self.interrupts.check()
    }
}

#[cfg(test)]
mod test_memory {
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
