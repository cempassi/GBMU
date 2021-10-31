use crate::consts;
use crate::Area;
use crate::MemoryBus;
use ppu::ppu::Ppu;
use shared::Error;

impl MemoryBus for Ppu {
    fn get(&self, address: usize) -> Result<u8, Error> {
        let address = address as u16;
        match address {
            consts::VRAM_MIN..=consts::VRAM_MAX => self.get_vram(Area::Vram.relative(address)),
            consts::LCD_CONTROL..=consts::LY_COMPARE => self.get_registers(address),
            consts::YWINDOW | consts::XWINDOW => self.get_registers(address),
            _ => unreachable!(),
        }
    }

    fn set(&mut self, address: usize, data: u8) -> Result<(), Error> {
        let address = address as u16;
        match address {
            consts::VRAM_MIN..=consts::VRAM_MAX => {
                self.set_vram(Area::Vram.relative(address), data)
            }
            consts::LCD_CONTROL..=consts::LY_COMPARE => self.set_registers(address, data),
            consts::YWINDOW | consts::XWINDOW => self.set_registers(address, data),
            _ => unreachable!(),
        }
    }
}
