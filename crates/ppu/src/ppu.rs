use crate::colors::Color;
use crate::fifo::Fifo;
use crate::registers::{Mode, Registers};
use shared::Interrupts;
use shared::{Error, Interrupt};

pub const VRAM_START: u16 = 0x8000;
pub const OAM_TABLE: usize = 0xA0;
pub const OAM_START: u16 = 0xFE00;
pub const FRAME_WIDTH: usize = 160;
pub const FRAME_HEIGHT: usize = 144;

#[derive(Debug)]
pub struct Ppu {
    vram: Vec<u8>,
    oam: Vec<u8>,
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

impl Ppu {
    pub fn new(interrupts: Interrupts, bios: bool) -> Self {
        let vram = vec![0; 8192];

        let registers = match bios {
            true => Registers::default(),
            false => Registers::new()
        };
        let fifo = Fifo::new();
        let screen = vec![Color::Black; FRAME_WIDTH * FRAME_HEIGHT];
        let oam = vec![0; OAM_TABLE];
        Self {
            vram_lock: false,
            vram,
            oam,
            registers,
            interrupts,
            fifo,
            screen,
        }
    }

    pub fn get_vram(&self, address: u16) -> Result<u8, Error> {
        let address: usize = (address - VRAM_START) as usize;
        Ok(self.vram[address])
    }

    pub fn get_oam(&self, address: u16) -> Result<u8, Error> {
        let address: usize = (address - OAM_START) as usize;
        Ok(self.oam[address])
    }

    pub fn set_vram(&mut self, address: u16, data: u8) -> Result<(), Error> {
        let address: usize = (address - VRAM_START) as usize;
        self.vram[address] = data;
        Ok(())
    }

    pub fn set_oam(&mut self, address: u16, data: u8) -> Result<(), Error> {
        let address: usize = (address - OAM_START) as usize;
        //println!("[PPU] setting oam. Address: {}", address);
        self.oam[address] = data;
        Ok(())
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

    pub fn render(&mut self, frame: &mut [u8]) {
        if self.registers().mode == Mode::Vblank {
            //println!("[PPU] Outputing to screen");
            for (index, pixel) in frame.chunks_exact_mut(4).enumerate() {
                let to_display: [u8; 4] = self.screen[index].into();

                pixel.copy_from_slice(&to_display);
            }
        }
    }

    pub fn output(&mut self, x: usize, pixel: u8) {
        let offset = self.registers.coordinates.offset(x);
        let color = self.registers.bgp.color(pixel);
        //println!("[PPU] position Offset: {}", offset);
        // println!(
        //     "[FIFO] Poped data. offset: {}, len {}",
        //     offset,
        //     self.fifo.len()
        // );
        self.screen[offset] = color;
    }

    pub fn update_registers(&self, registers: &mut Registers) {
        registers.update(&self.registers)
    }
    pub fn reload_coordinates(&self, coordinates: &mut super::Coordinates) {
        self.registers.coordinates.update(coordinates)
    }

    pub fn raise_ly_lyc(&self) {
        if self.registers().lyc_ly_interupt && self.registers().lyc_ly {
            self.raise_lcd();
        }
    }

    pub fn raise_hblank(&self) {
            self.raise_lcd();
        if self.registers().hblank_interupt {
        }
    }

    pub fn raise_oam(&self) {
            self.raise_lcd();
        if self.registers().oam_interupt {
        }
    }

    pub fn raise_vblank(&self) {
            self.interrupts.borrow_mut().request(Interrupt::VBlank);
        if self.registers().vblank_interupt{
        }
    }

    pub fn raise_lcd(&self) {
        self.interrupts.borrow_mut().request(Interrupt::Lcd);
    }


    /// Get a reference to the ppu's registers.
    pub fn registers(&self) -> &Registers {
        &self.registers
    }
}
