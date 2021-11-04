use crate::colors::Color;
use crate::fifo::Fifo;
use crate::registers::Registers;
use shared::Interrupts;
use shared::{Error, Interrupt};

pub const VRAM_START: u16 = 0x8000;
const WIDTH: usize = 160;
const HEIGHT: usize = 144;

#[derive(Debug)]
pub struct Ppu {
    pub vram_lock: bool,
    vram: Vec<u8>,
    pub(crate) registers: Registers,
    interrupts: Interrupts,
    pub(crate) fifo: Fifo,
    screen: Vec<Color>,
    pub(crate) x: usize,
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
        let screen = vec![Color::White; WIDTH * HEIGHT];
        let x = 0;
        Self {
            vram_lock: false,
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

    pub fn render(&mut self, frame: &mut [u8]) {
        println!("Outputing to screen");
        for (index, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let to_display: [u8; 4] = self.screen[index].into();

            pixel.copy_from_slice(&to_display);
        }
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
        if let Some(pixel) = self.fifo.pop() {
            let color = Color::from(pixel);
            self.screen[self.x] = color;
            self.x += 1;
            Some(())
        } else {
            None
        }
    }
}
