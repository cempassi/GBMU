#![allow(dead_code)]
use crate::futures::Fetch;
use crate::Ppu;
use modular_bitfield::{bitfield, specifiers::B3};
use shared::Error;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sprite {
    y: u8,
    pub x: u8,
    index: u8,
    attributes: Attributes,
}

/// Bit7   BG and Window over OBJ (0=No, 1=BG and Window colors 1-3 over the OBJ)
/// Bit6   Y flip          (0=Normal, 1=Vertically mirrored)
/// Bit5   X flip          (0=Normal, 1=Horizontally mirrored)
/// Bit4   Palette number  **Non CGB Mode Only** (0=OBP0, 1=OBP1)
/// Bit3   Tile VRAM-Bank  **CGB Mode Only**     (0=Bank 0, 1=Bank 1)
/// Bit2-0 Palette number  **CGB Mode Only**     (OBP0-7)

#[bitfield]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Attributes {
    cgb_palette: B3,
    cgb_bank: bool,
    palette: bool,
    x_flip: bool,
    y_flip: bool,
    priority: bool,
}

impl Sprite {
    pub async fn try_new(ppu: &Ppu, address: u16) -> Result<(Self, u8), Error> {
        let y = ppu.borrow().get_oam(address)?;
        let (x, x_cycles) = Fetch::new(ppu, address + 1).await?;
        let index = ppu.borrow().get_oam(address + 2)?;
        let (attributes, attributes_cycles) = Fetch::new(ppu, address + 3).await?;
        let attributes: Attributes = Attributes::from_bytes([attributes]);
        let sprite = Self {
            y: y - 16,
            x: x - 8,
            index,
            attributes,
        };
        let cycles = x_cycles + attributes_cycles;
        println!("[OAM] cycles: {}", cycles);
        Ok((sprite, cycles))
    }

    pub fn is_line(&self, ly: u8, size: u8) -> bool {
        self.y > 0 && self.y < 160 && ly > self.y.wrapping_sub(size) && ly <= self.y
    }
}
