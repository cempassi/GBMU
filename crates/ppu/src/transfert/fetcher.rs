use super::pixels::Row;

use crate::futures::Fetch;
use crate::interface::Push;

use crate::Ppu;
use shared::Error;

/// The Fetcher will fetch all pixels in a lines of tiles.
/// The fetcher must work in parallel with the Fifo
/// Index: Index of the tile to read
pub struct Fetcher {
    ppu: Ppu,
    map_row: u16,
}

impl Fetcher {
    pub fn new(ppu: Ppu) -> Self {
        let mut p = ppu.borrow_mut();
        // New line, so x is 0;
        let xscroll = p.registers.coordinates.xscroll();
        let map_row = p.registers.tile_map_row_address();

        p.fifo.clear();
        p.fifo.scroll(xscroll);

        drop(p);
        Self {
            ppu,
            map_row,
        }
    }


    pub async fn fetch(self) -> Result<u8, Error> {
        let mut cycles = 0;

        // This loop fetches every pixels in a line.
        // Many checks have to opperate here as the line Fetcher is complex
        // (Background, Window, Sprite)
        // Carefull implemenation
        for i in 0..=21 {
            // First get the adress of the Tile id
            // This may be refactored to handle background or window id
            //println!("[FETCHER] Fetching tile id");

            let map_address = self.map_row + i as u16;
            let (tile_id, ticks) = Fetch::new(&self.ppu, map_address).await?;

            cycles += ticks;

            //println!("[FETCHER] Processing tile address");
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
