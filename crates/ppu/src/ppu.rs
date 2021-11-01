use crate::fifo::Fifo;
use crate::registers::Registers;
use shared::Interrupts;
use shared::{Error, Interrupt};

pub const VRAM_START: u16 = 0x8000;

#[derive(Debug)]
pub struct Ppu {
    vram: Vec<u8>,
    pub(crate) registers: Registers,
    interrupts: Interrupts,
    pub(crate) fifo: Fifo,
    screen: Vec<u8>,
    pub(crate) x: u8,
}

impl AsRef<Vec<u8>> for Ppu {
    fn as_ref(&self) -> &Vec<u8> {
        self.vram.as_ref()
    }
}

impl AsRef<Registers> for Ppu {
    fn as_ref(&self) -> &Registers {
        &self.registers
    }
}

impl From<Interrupts> for Ppu {
    fn from(interrupts: Interrupts) -> Self {
        let vram = vec![0; 8192];
        let registers = Registers::default();
        let fifo = Fifo::new();
        let screen = Vec::new();
        let x = 0;
        Self {
            vram,
            registers,
            interrupts,
            fifo,
            screen,
            x,
        }
    }
}

impl Ppu {
    pub fn get_vram(&self, address: u16) -> Result<u8, Error> {
        let address: usize = (address - VRAM_START) as usize;
        Ok(self.vram[address])
    }

    pub fn set_vram(&mut self, address: u16, data: u8) -> Result<(), Error> {
        let address: usize = (address - VRAM_START) as usize;
        self.vram[address] = data;
        Ok(())
    }

    pub fn reload_coordinates(&self, coordinates: &mut super::Coordinates) {
        self.registers.coordinates.update(coordinates)
    }

    pub fn get_registers(&self, address: u16) -> Result<u8, Error> {
        Ok(self.registers.get(address))
    }

    pub fn set_registers(&mut self, address: u16, data: u8) -> Result<(), Error> {
        println!(
            "CPU is Writing to PPU Registers at {:#X}, data: {:#b}",
            address, data
        );
        self.registers.set(address, data);
        Ok(())
    }

    pub fn raise_vblank(&self) {
        self.interrupts.borrow_mut().request(Interrupt::Lcd);
    }

    pub fn line_finished(&mut self) -> bool {
        if self.x == 160 {
            self.x = 0;
            true
        } else {
            false
        }
    }

    pub fn write(&mut self) -> Option<()> {
        if let Some(_pixel) = self.fifo.pop() {
            self.x += 1;
            Some(())
        } else {
            None
        }
    }
}
