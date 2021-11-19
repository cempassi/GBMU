use shared::Error;

use crate::registers::Mode;
use crate::sprite::Sprite;
use crate::Ppu;

use crate::consts;

pub struct Oam {}

impl Oam {
    pub async fn search(ppu: Ppu) -> Result<u16, Error> {
        ppu.borrow_mut().registers.mode.update(Mode::Oam);
        ppu.borrow_mut().oam_lock = true;
        let mut address = consts::OAM_START;
        let mut cycles = 0;
        let mut sprites = 0;
        while address < consts::OAM_END && sprites <= 9 {
            let (sprite, cycle) = Sprite::try_new(&ppu, address).await?;
            let size = ppu.borrow().registers.control.sprite_size;
            let ly = ppu.borrow().registers.coordinates.ly;
            if sprite.is_line(ly, size) {
                ppu.borrow_mut().fifo.push_sprite(sprite);
                sprites += 1;
            }
            address += 4;
            cycles += cycle as u16;
        }
        ppu.borrow_mut().oam_lock = false;
        Ok(cycles)
    }
}
