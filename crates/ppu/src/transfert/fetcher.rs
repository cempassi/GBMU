use crate::futures::Fetch;
use crate::interface::Push;

use crate::Ppu;
use shared::Error;

pub struct DataVec(Vec<bool>);

struct PixelData {
    byte0: u8,
    byte1: u8,
    cycles: u8,
}

impl DataVec {
    pub fn new(data: u8) -> Self {
        let mut datavec = Vec::new();
        for i in 0..=7 {
            datavec.insert(0, data & (1 << i) != 0);
        }
        DataVec(datavec)
    }
}

impl PixelData {
    pub async fn try_new(ppu: &'_ Ppu, address: u16) -> Result<Self, Error> {
        let (byte0, cycles_0) = Fetch::new(ppu, address).await?;
        //println!("[FETCHER] byte0 fetched, cycles: {}", cycles_0);
        let (byte1, cycles_1) = Fetch::new(ppu, address + 1).await?;
        //println!("[FETCHER] byte1 fetched, cycles: {}", cycles_1);

        Ok(Self {
            byte0,
            byte1,
            cycles: cycles_0 + cycles_1,
        })
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
    index: u8,
    id_address: u16,
}

impl Fetcher {
    pub fn new(ppu: Ppu, id_address: u16, index: u8) -> Self {
        Self {
            ppu,
            id_address,
            index,
        }
    }

    fn get_data_addr(&self, id: u8, line: u8) -> u16 {
        let offset = 0x8000 + (id as u16 * 16);
        offset + (line * 2) as u16
    }

    pub async fn fetch(self, line: u8) -> Result<u8, Error> {
        let mut cycles = 0;

        //println!("[FETCHER] Starting to fetch. id: {}", self.index);
        let id_address = self.id_address + self.index as u16;
        let (id, ticks) = Fetch::new(&self.ppu, id_address).await?;
        cycles += ticks;
        //println!("[FETCHER] Index fetched, cycles: {}", cycles);

        let data_addr = self.get_data_addr(id, line);
        let data = PixelData::try_new(&self.ppu, data_addr).await?;
        cycles += data.cycles;

        // Try to push pixels in ppu queue
        let ticks = self.ppu.push(data.get_pixels()).await;
        cycles += ticks;
        Ok(cycles)
    }
}
