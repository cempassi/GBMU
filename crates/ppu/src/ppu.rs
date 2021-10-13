use crate::blanks::{Blank, HBLANK, VBLANK};
use crate::interface::Registers;
use crate::oam::Oam;
use crate::registers::lcd::Register;
use crate::transfert::Pixel;
use std::cell::RefCell;
use std::rc::Rc;

const VRAM_SIZE: usize = 8192;

#[derive(Clone)]
pub struct Ppu {
    vram: Vec<u8>,
    registers: Registers,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            vram: vec![0; VRAM_SIZE],
            registers: Rc::new(RefCell::new(Default::default())),
        }
    }

    pub async fn run(self) -> u8 {
        println!("Running the ppu!");
        while self.registers.borrow_mut().is_lower(Register::Ly, 144) {
            Oam::search(self.registers.clone(), self.vram.clone()).await;
            Pixel::transfert(self.registers.clone(), self.vram.clone()).await;
            self.registers.borrow_mut().increase(Register::Ly);
            Blank::new(HBLANK).await;
        }
        Blank::new(VBLANK).await;
        self.registers.borrow_mut().clear(Register::Ly);
        42
    }
}

impl AsRef<Vec<u8>> for Ppu {
    fn as_ref(&self) -> &Vec<u8> {
        self.vram.as_ref()
    }
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}
