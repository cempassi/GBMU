use crate::colors::Color;
use crate::consts;
use crate::fifo::Fifo;
use crate::registers::{Mode, Registers};
use crate::transfert::Pixel;
use shared::Interrupts;
use shared::{Error, Interrupt};

#[derive(Debug)]
pub struct Ppu {
    vram: Vec<u8>,
    oam: Vec<u8>,
    interrupts: Interrupts,
    screen: Vec<Color>,
    pub vram_lock: bool,
    pub oam_lock: bool,
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
        let screen = vec![Color::Black; consts::FRAME_WIDTH * consts::FRAME_HEIGHT];
        let oam = vec![0; consts::OAM_TABLE];
        Self {
            vram_lock: false,
            oam_lock: false,
            vram,
            oam,
            registers,
            interrupts,
            fifo,
            screen,
        }
    }
}

impl Ppu {
    pub fn no_bios(interrupts: Interrupts) -> Self {
        let mut ppu = Self::from(interrupts);
        ppu.set_registers(0xFF40, 0x91).unwrap();
        ppu.set_registers(0xFF41, 0x02).unwrap();
        ppu
    }

    pub fn get_vram(&self, address: u16) -> Result<u8, Error> {
        if self.vram_lock {
            let address: usize = (address - consts::VRAM_START) as usize;
            Ok(self.vram[address])
        } else {
            println!("[PPU] VRAM Locked");
            Ok(0xFF)
        }
    }

    pub fn get_oam(&self, address: u16) -> Result<u8, Error> {
        if self.oam_lock {
            let address: usize = (address - consts::OAM_START) as usize;
            Ok(self.oam[address])
        } else {
            println!("[PPU] OAM Locked");
            Ok(0xFF)
        }
    }

    pub fn set_vram(&mut self, address: u16, data: u8) -> Result<(), Error> {
        if self.vram_lock {
            let address: usize = (address - consts::VRAM_START) as usize;
            self.vram[address] = data;
            Ok(())
        } else {
            println!("[PPU] VRAM Locked");
            Ok(())
        }
    }

    pub fn set_oam(&mut self, address: u16, data: u8) -> Result<(), Error> {
        if self.oam_lock {
            let address: usize = (address - consts::OAM_START) as usize;
            println!("[PPU] setting oam. Address: {}", address);
            self.oam[address] = data;
            Ok(())
        } else {
            println!("[PPU] OAM Locked");
            Ok(())
        }
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
            println!("[PPU] Outputing to screen");
            for (index, pixel) in frame.chunks_exact_mut(4).enumerate() {
                let to_display: [u8; 4] = self.screen[index].into();

                pixel.copy_from_slice(&to_display);
            }
        }
    }

    pub fn output(&mut self, x: usize, pixel: Pixel) {
        let offset = self.registers.coordinates.offset(x);
        let color = self.registers.background_p.color(pixel.color());
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

    pub fn raise_vblank(&self, interrupt: Interrupt) {
        self.interrupts.borrow_mut().request(interrupt);
    }

    /// Get a reference to the ppu's registers.
    pub fn registers(&self) -> &Registers {
        &self.registers
    }
}
