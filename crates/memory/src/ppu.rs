use crate::consts;
use crate::MemoryBus;
use ppu::ppu::Ppu;
use shared::Error;

impl MemoryBus for Ppu {
    fn get(&self, address: usize) -> Result<u8, Error> {
        let address = address as u16;
        match address {
            consts::VRAM_MIN..=consts::VRAM_MAX => {
                if self.vram_lock {
                    println!("[CPU] Vram Locked");
                    Ok(0xFF)
                } else {
                    self.get_vram(address)
                }
            }
            consts::OAM_MIN..=consts::OAM_MAX => self.get_oam(address),
            consts::LCD_CONTROL..=consts::LY_COMPARE => self.get_registers(address),
            consts::YWINDOW | consts::XWINDOW => self.get_registers(address),
            _ => unreachable!(),
        }
    }

    fn set(&mut self, address: usize, data: u8) -> Result<(), Error> {
        let address = address as u16;
        match address {
            consts::VRAM_MIN..=consts::VRAM_MAX => {
                if self.vram_lock {
                    println!("[CPU] Vram Locked");
                    Ok(())
                } else {
                    self.set_vram(address, data)
                }
            }
            consts::OAM_MIN..=consts::OAM_MAX => self.set_oam(address, data),
            consts::LCD_CONTROL..=consts::LY_COMPARE => self.set_registers(address, data),
            consts::YWINDOW | consts::XWINDOW => self.set_registers(address, data),
            _ => unreachable!(),
        }
    }
}
