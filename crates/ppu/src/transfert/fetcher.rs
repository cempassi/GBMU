use super::pixels::Row;

use crate::futures::Fetch;
use crate::interface::Push;

use crate::registers::coordinates::XRange;
use crate::Ppu;
use shared::Error;

/// The Fetcher will fetch all pixels in a lines of tiles.
/// The fetcher must work in parallel with the Fifo
/// Index: Index of the tile to read
pub struct Fetcher {
    ppu: Ppu,
    x_range: XRange,
}

impl Fetcher {
    pub fn new(ppu: Ppu) -> Self {
        let mut p = ppu.borrow_mut();
        // New line, so x is 0;
        let x_range = p.registers.coordinates.x_range();
        let x_discard = p.registers.coordinates.x_discard();

        p.fifo.clear();
        p.fifo.discard(x_discard);

        drop(p);
        Self { ppu, x_range }
    }

    //async fn fetch_sprite(&self)

    pub async fn fetch(self) -> Result<u8, Error> {
        let mut cycles = 0;

        // This loop fetches every pixels in a line.
        // Many checks have to opperate here as the line Fetcher is complex
        // (Background, Window, Sprites)
        // Carefull implemenation
        for x in self.x_range {
            // Checks if window we have to draw the window
            if self.ppu.borrow().registers.window_start(x) {
                self.ppu.borrow_mut().fifo.clear();
            }
            // Checks if we have to draw a sprite
            if self.ppu.borrow().fifo.is_sprite(x) {
                println!("[FETCHER] Sprite found. index: {}", x);
                continue;
            }
            // First get the adress of the Tile id
            // This may be refactored to handle background or window id
            let map_address = self.ppu.borrow().registers.map_address(x);
            println!("[FETCHER] map addres: {:#X}", map_address);
            let (tile_id, ticks) = Fetch::new(&self.ppu, map_address).await?;

            cycles += ticks;

            // Then we get the address of a row of pixels in that tile

            let row = Row::try_new(&self.ppu, tile_id).await?;
            // Finaly we convert that Row into a vector of pixels, and push
            // thoes in the ppu queue
            let ticks = self.ppu.push(row.into()).await;
            cycles += ticks;
        }
        //println!("Exited from Fetcher");
        Ok(cycles)
    }
}
