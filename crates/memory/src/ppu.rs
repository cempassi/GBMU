use crate::consts;
use crate::Area;
use crate::MemoryBus;
use ppu::ppu::Ppu;

impl MemoryBus for Ppu {
    fn get(&self, address: usize) -> u8 {
        match address as u16 {
            consts::VRAM_MIN..=consts::VRAM_MAX => {
                self.get_vram(Area::Vram.relative(address as u16))
            }
            _ => unreachable!(),
        }
    }

    fn set(&mut self, address: usize, data: u8) {
        match address as u16 {
            consts::VRAM_MIN..=consts::VRAM_MAX => {
                self.set_vram(Area::Vram.relative(address as u16), data)
            }
            _ => unreachable!(),
        }
    }
}
