use std::convert::From;

use shared::Error;

use crate::{futures::Fetch, Ppu};

const TILE_SIZE: u16 = 16;

pub struct Bits([bool; 8]);

/// A Row of Pixels in a tile, encoded on two bytes
pub struct Row {
    byte0: u8,
    byte1: u8,
}

type Pixels = [u8; 8];

impl Bits {
    pub fn new(data: u8) -> Self {
        let mut bits = [false; 8];
        for i in (0..=7).rev() {
            bits[i] = data & (1 << i) != 0;
        }
        //println!("Data vec: {:?}", datavec);
        Bits(bits)
    }
}

impl From<Row> for Pixels {
    fn from(row: Row) -> Self {
        let mut pixels = [0; 8];
        let Bits(lower) = Bits::new(row.byte0);
        let Bits(higher) = Bits::new(row.byte1);
        let iterator = lower.iter().zip(higher.iter()).rev();

        for (i, (lower, higher)) in iterator.enumerate() {
            let higher = (*higher) as u8;
            let lower = (*lower) as u8;
            pixels[i] = (higher << 1) + lower;
        }
        pixels
    }
}

impl Row {
    /// Calculate the address of a row of pixels in a tile from Tile ID
    fn row_address(tile_line: usize, data_area: u16, id: u8) -> u16 {
        if data_area == 0x8000 {
            let offset = data_area + (id as u16 * TILE_SIZE);
            offset + (tile_line * 2) as u16
        } else {
            let relative = id as i16 * TILE_SIZE as i16;
            let address = data_area as i16 + relative;
            address as u16
        }
    }

    /// Get the data of a row in a tile from tile id
    pub async fn try_new(ppu: &'_ Ppu, id: u8) -> Result<Self, Error> {
        let p = ppu.borrow();
        let data_area = p.registers.control.data_area;
        let tile_line = p.registers.coordinates.tile_line();
        let address = Self::row_address(tile_line, data_area, id);

        drop(p);
        let (byte0, _) = Fetch::new(ppu, address).await?;
        //println!("[FETCHER] byte0 fetched");
        let (byte1, _) = Fetch::new(ppu, address + 1).await?;
        //println!("[FETCHER] byte1 fetched");

        Ok(Self { byte0, byte1 })
    }
}

#[cfg(test)]
mod test_tiles {
    use shared::{execute, Interrupts};

    use super::*;
    /// The Tile data is organiazed as follows:
    /// See https://blog.flozz.fr/2018/11/19/developpement-gameboy-5-creer-des-tilesets/
    /// [0, 0, 0, 0, 0, 3, 0, 0]);
    /// [0, 0, 0, 0, 0, 3, 0, 0]);
    /// [0, 0, 0, 0, 3, 0, 3, 0]);
    /// [0, 0, 0, 3, 0, 0, 3, 0]);
    /// [0, 1, 1, 0, 0, 1, 1, 0]);
    /// [1, 2, 2, 3, 1, 2, 2, 3]);
    /// [1, 2, 2, 3, 1, 2, 2, 3]);
    /// [0, 3, 3, 0, 0, 3, 3, 0]);
    fn setup_ppu(ly: u8) -> Ppu {
        let interrupts = Interrupts::default();
        let ppu = Ppu::new(interrupts);
        let tile = vec![
            0x04, 0x04, 0x04, 0x04, 0x0a, 0x0a, 0x12, 0x12, 0x66, 0x00, 0x99, 0x77, 0x99, 0x77,
            0x66, 0x66,
        ];
        let mut p = ppu.borrow_mut();
        p.vram_lock = true;
        for (index, data) in tile.iter().enumerate() {
            p.set_vram((0x8000 + index) as u16, *data).unwrap();
        }
        p.registers.coordinates.set(crate::Field::Ly, ly);
        drop(p);
        ppu
    }

    #[test]
    fn test_get_tile_row_zero_from_memory() {
        let ppu = setup_ppu(0);
        let expected = [0, 0, 0, 0, 0, 3, 0, 0];
        let row = execute::execute(Box::pin(Row::try_new(&ppu, 0))).unwrap();
        let result: Pixels = row.into();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_tile_row_one_from_memory() {
        let ppu = setup_ppu(1);
        let expected = [0, 0, 0, 0, 0, 3, 0, 0];
        let row = execute::execute(Box::pin(Row::try_new(&ppu, 0))).unwrap();
        let result: Pixels = row.into();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_tile_row_two_from_memory() {
        let ppu = setup_ppu(2);
        let expected = [0, 0, 0, 0, 3, 0, 3, 0];
        let row = execute::execute(Box::pin(Row::try_new(&ppu, 0))).unwrap();
        let result: Pixels = row.into();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_tile_row_three_from_memory() {
        let ppu = setup_ppu(3);
        let expected = [0, 0, 0, 3, 0, 0, 3, 0];
        let row = execute::execute(Box::pin(Row::try_new(&ppu, 0))).unwrap();
        let result: Pixels = row.into();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_tile_row_four_from_memory() {
        let ppu = setup_ppu(4);
        let expected = [0, 1, 1, 0, 0, 1, 1, 0];
        let row = execute::execute(Box::pin(Row::try_new(&ppu, 0))).unwrap();
        let result: Pixels = row.into();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_tile_row_five_from_memory() {
        let ppu = setup_ppu(5);
        let expected = [1, 2, 2, 3, 1, 2, 2, 3];
        let row = execute::execute(Box::pin(Row::try_new(&ppu, 0))).unwrap();
        let result: Pixels = row.into();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_tile_row_six_from_memory() {
        let ppu = setup_ppu(6);
        let expected = [1, 2, 2, 3, 1, 2, 2, 3];
        let row = execute::execute(Box::pin(Row::try_new(&ppu, 0))).unwrap();
        let result: Pixels = row.into();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_tile_row_seven_from_memory() {
        let ppu = setup_ppu(7);
        let expected = [0, 3, 3, 0, 0, 3, 3, 0];
        let row = execute::execute(Box::pin(Row::try_new(&ppu, 0))).unwrap();
        let result: Pixels = row.into();

        assert_eq!(result, expected);
    }
}
