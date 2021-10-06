use crate::blanks::{Blank, HBLANK, VBLANK};
use crate::oam::Oam;
use crate::registers::lcd::Register;
use crate::registers::Registers;
use crate::transfert::Pixel;
use crate::vram::Vram;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct Inner {
    vram: Vram,
    registers: Registers,
}

#[derive(Debug, Default, Clone)]
pub struct Ppu(Rc<RefCell<Inner>>);

impl AsRef<Vec<u8>> for Inner {
    fn as_ref(&self) -> &Vec<u8> {
        self.vram.as_ref()
    }
}

impl Ppu {
    pub async fn run(mut self) -> u8 {
        println!("Running the ppu!");
        while self.is_lower(Register::Ly, 144) {
            Oam::search(self.clone()).await;
            Pixel::transfert(self.clone()).await;
            Blank::new(HBLANK).await;
            self.increase(Register::Ly)
        }
        Blank::new(VBLANK).await;
        self.clear(Register::Ly);
        42
    }

    fn is_lower(&mut self, register: Register, nbr: u8) -> bool {
        let Ppu(ppu) = self;
        ppu.borrow_mut().registers.is_lower(register, nbr)
    }

    fn increase(&self, register: Register){
        let Ppu(ppu) = self;
        ppu.borrow_mut().registers.increase(register);
    }

    fn clear(&self, register: Register){
        let Ppu(ppu) = self;
        ppu.borrow_mut().registers.clear(register);
    }

}
