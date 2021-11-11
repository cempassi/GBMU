use crate::colors::Color;
use crate::fifo::Fifo;
use crate::registers::Registers;
use shared::Interrupts;
use shared::{Error, Interrupt};

pub const VRAM_START: u16 = 0x8000;
pub const WIDTH: usize = 160;
pub const HEIGHT: usize = 144;

#[derive(Debug)]
pub struct Ppu {
    vram: Vec<u8>,
    interrupts: Interrupts,
    screen: Vec<Color>,
    pub frame_ready: bool,
    pub vram_lock: bool,
    pub registers: Registers,
    pub(crate) fifo: Fifo,
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
        let frame_ready = true;
        Self {
            frame_ready,
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

    pub fn is_frame_ready(&mut self) -> bool {
        if self.frame_ready {
            self.frame_ready = false;
            true
        } else {
            false
        }
    }

    pub fn render(&mut self, frame: &mut [u8]) {
        println!("Outputing to screen");
        for (index, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let to_display: [u8; 4] = self.screen[index].into();

            pixel.copy_from_slice(&to_display);
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

    pub fn pop(&mut self) -> bool {
        if let Some(pixel) = self.fifo.try_pop() {
            let color = Color::from(pixel);
            let offset = self.registers.coordinates.offset(self.x);
            println!(
                "[FIFO] Poped data. x: {}, offset: {}, len {}",
                self.x,
                offset,
                self.fifo.len()
            );
            self.screen[offset] = color;
            self.x += 1;
            true
        } else {
            println!("Cannot write. Data in fifo: {}", self.fifo.len());
            false
        }
    }

    /// Get a reference to the ppu's registers.
    pub fn registers(&self) -> &Registers {
        &self.registers
    }
}
