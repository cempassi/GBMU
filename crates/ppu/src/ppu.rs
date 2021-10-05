use crate::blanks::{Blank, HBLANK, VBLANK};
use crate::interface::{Registers, Vram};
use crate::oam::Oam;
use crate::registers::lcd::Register;
use crate::transfert::Pixel;

#[derive(Default, Clone)]
pub struct Ppu {
    vram: Vram,
    registers: Registers,
}

impl Ppu {
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
