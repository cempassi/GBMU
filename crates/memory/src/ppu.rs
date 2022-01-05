use crate::consts;
use crate::MemoryBus;
use ppu::ppu::Ppu;
use shared::Error;

impl MemoryBus for Ppu {
    fn get(&self, address: usize) -> Result<u8, Error> {
        let map = address as u16;
        match map {
            consts::VRAM_MIN..=consts::VRAM_MAX => self.get(address),
            consts::OAM_MIN..=consts::OAM_MAX => self.get(address),
            consts::LCD_CONTROL..=consts::LY_COMPARE => self.get(address),
            consts::YWINDOW | consts::XWINDOW | consts::BGP => self.get(address),
            _ => unreachable!(),
        }
    }

    fn set(&mut self, address: usize, data: u8) -> Result<(), Error> {
        let map = address as u16;
        match map {
            consts::VRAM_MIN..=consts::VRAM_MAX => self.set(address, data),
            consts::OAM_MIN..=consts::OAM_MAX => self.set(address, data),
            consts::LCD_CONTROL..=consts::LY_COMPARE => self.set(address, data),
            consts::YWINDOW | consts::XWINDOW | consts::BGP => self.set(address, data),
            _ => unreachable!(),
        }
    }
}
