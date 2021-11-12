use std::usize;

use crate::futures::Fetch;
use crate::interface::Push;

use crate::Ppu;
use shared::Error;

pub struct DataVec(Vec<bool>);

struct PixelData {
    byte0: u8,
    byte1: u8,
}

impl DataVec {
    pub fn new(data: u8) -> Self {
        let mut datavec = Vec::new();
        for i in 0..=7 {
            datavec.insert(0, data & (1 << i) != 0);
        }
        //println!("Data vec: {:?}", datavec);
        DataVec(datavec)
    }
}

impl PixelData {
    pub async fn try_new(ppu: &'_ Ppu, address: u16) -> Result<Self, Error> {
        let (byte0, _) = Fetch::new(ppu, address).await?;
        //println!("[FETCHER] byte0 fetched");
        let (byte1, _) = Fetch::new(ppu, address + 1).await?;
        //println!("[FETCHER] byte1 fetched");

        Ok(Self { byte0, byte1 })
    }

    fn get_pixels(&self) -> Vec<u8> {
        let mut pixels = Vec::new();
        let DataVec(lower) = DataVec::new(self.byte0);
        let DataVec(higher) = DataVec::new(self.byte1);
        let iterator = lower.iter().zip(higher.iter()).enumerate();

        for (index, (lower, higher)) in iterator {
            let higher = (*higher) as u8;
            let lower = (*lower) as u8;
            pixels.insert(index, (higher << 1) + lower);
        }
        pixels
    }
}

/// The Fetcher will fetch all pixels in a lines of tiles.
/// The fetcher must work in parallel with the Fifo
/// Index: Index of the tile to read
pub struct Fetcher {
    ppu: Ppu,
    line: usize,
    bg_area: u16,
    data_area: u16,
    row: usize,
}

impl Fetcher {
    pub fn new(ppu: Ppu) -> Self {
        let mut p = ppu.borrow_mut();
        // New line, so x is 0;
        let line = p.registers.coordinates.line();
        let bg_area = p.registers.control.bg_area;
        let data_area = p.registers.control.data_area;
        let xscroll = p.registers.coordinates.xscroll();
        p.fifo.scroll(xscroll);
        p.fifo.clear();

        let row = p.registers.coordinates.row();
        drop(p);
        Self {
            ppu,
            line,
            bg_area,
            data_area,
            row,
        }
    }

    fn tile_address(&self, id: u8) -> u16 {
        if self.data_area == 0x8000 {
            let offset = self.data_area + (id as u16 * 16);
            offset + (self.line * 2) as u16
        } else {
            let relative = id as i16 * 16;
            let address = self.data_area as i16 + relative;
            address as u16
        }
    }

    pub async fn fetch(mut self) -> Result<u8, Error> {
        let mut cycles = 0;

        // This loop fetches every pixels in a line.
        // Many checks have to opperate here as the line Fetcher is complex
        // (Background, Window, Sprite)
        // Carefull implemenation
        for i in 0..crate::ppu::FRAME_WIDTH {
            // First get the adress of the Tile id
            // This may be refactored to handle background or window id
            //println!("[FETCHER] Fetching tile id");
            self.line = self.ppu.borrow().registers.coordinates.line();
            //let x = self.ppu.borrow().registers.coordinates.x(i);
            //let column = i / 8;
            let tile_map_index = (self.row as u16 * 32) + i as u16; //
            let id_address = self.bg_area + tile_map_index;
            let (tile_id, ticks) = Fetch::new(&self.ppu, id_address).await?;

            cycles += ticks;

            //println!("[FETCHER] Processing tile address");
            // Then we get the actual tile address
            let tile_address = self.tile_address(tile_id);

            let data = PixelData::try_new(&self.ppu, tile_address).await?;
            // Try to push pixels in ppu queue
            let ticks = self.ppu.push(data.get_pixels()).await;
            cycles += ticks;
        }
        //println!("Exited from Fetcher");
        Ok(cycles)
    }
}
