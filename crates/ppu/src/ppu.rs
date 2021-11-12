use crate::colors::Color;
use crate::fifo::Fifo;
use crate::registers::{Mode, Registers};
use shared::Interrupts;
use shared::{Error, Interrupt};

pub const VRAM_START: u16 = 0x8000;
pub const FRAME_WIDTH: usize = 160;
pub const FRAME_HEIGHT: usize = 144;

#[derive(Debug)]
pub struct Ppu {
    vram: Vec<u8>,
    interrupts: Interrupts,
    screen: Vec<Color>,
    pub vram_lock: bool,
    pub registers: Registers,
    pub(crate) fifo: Fifo,
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
        let screen = vec![Color::White; FRAME_WIDTH * FRAME_HEIGHT];
        Self {
            vram_lock: false,
            vram,
            registers,
            interrupts,
            fifo,
            screen,
        }
    }
}

impl Ppu {
    pub fn get_vram(&self, address: u16) -> Result<u8, Error> {
        let address: usize = (address - VRAM_START) as usize;
        Ok(self.vram[address])
    }

    pub fn render(&mut self, frame: &mut [u8]) {
        println!("[PPU] Outputing to screen");
        if self.registers().mode == Mode::Vblank {
            for (index, pixel) in frame.chunks_exact_mut(4).enumerate() {
                let to_display: [u8; 4] = self.screen[index].into();

                pixel.copy_from_slice(&to_display);
            }
        }
    }

    pub fn update_registers(&self, registers: &mut Registers) {
        registers.update(&self.registers)
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
        // println!(
        //     "CPU is Writing to PPU Registers at {:#X}, data: {:#b}",
        //     address, data
        // );
        self.registers.set(address, data);
        Ok(())
    }

    pub fn raise_vblank(&self) {
        self.interrupts.borrow_mut().request(Interrupt::Lcd);
    }

    pub fn output(&mut self, x: usize, pixel: u8) {
        let offset = self.registers.coordinates.offset(x);
        // println!(
        //     "[FIFO] Poped data. offset: {}, len {}",
        //     offset,
        //     self.fifo.len()
        // );
        self.screen[offset] = pixel.into();
    }

    /// Get a reference to the ppu's registers.
    pub fn registers(&self) -> &Registers {
        &self.registers
    }
}
