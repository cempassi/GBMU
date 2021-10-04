use crate::blanks::{Blank, HBLANK, VBLANK};
use crate::oam::Oam;
use crate::transfert::Pixel;
use memory::Memory;

#[derive(Default, Clone)]
pub struct Ppu {
    memory: Memory,
    ly: u8,
}

impl Ppu {
    pub fn new(memory: Memory) -> Self {
        Self { memory, ly: 0 }
    }

    pub async fn run(mut self) -> u8 {
        println!("Running the ppu!");
        while self.ly < 144 {
            Oam::search(self.memory.clone()).await;
            Pixel::transfert(self.memory.clone()).await;
            self.ly += 1;
            Blank::new(HBLANK).await;
        }
        Blank::new(VBLANK).await;
        self.ly = 0;
        42
    }
}
