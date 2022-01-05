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
    map_row: u16,
    x_range: XRange,
}

impl Fetcher {
    pub fn new(ppu: Ppu) -> Self {
        let mut p = ppu.borrow_mut();
        // New line, so x is 0;
        let map_row = p.registers.tile_map_row_address();
        let x_range = p.registers.coordinates.x_range();

        p.fifo.clear();

        drop(p);
        Self {
            ppu,
            map_row,
            x_range,
        }
    }
    fn render_tiles(&mut self) {
        /// Width or height of a tile, in pixels.
        const TILE_SIZE: u16 = 8;

        /// Width of the tile map, in bytes.
        const TILE_MAP_WIDTH: u16 = 32;

        debug_assert!(self.line <= 143, "scanline out of range");

        // Draw the line.
        for screen_x in 0..SCREEN_WIDTH as u8 {
            let screen_y = self.line;

            let use_window = self.control.window_enabled
                && screen_y >= self.window.y
                && screen_x >= self.window.x;

            let (tile_y, tile_x) = if use_window {
                let y = screen_y.wrapping_sub(self.window.y);
                let x = screen_x.wrapping_sub(self.window.x);
                (u16::from(y), x)
            } else {
                let y = screen_y.wrapping_add(self.bg_scroll.y);
                let x = screen_x.wrapping_add(self.bg_scroll.x);
                (u16::from(y), x)
            };

            // Get the address of the tile in memory.
            let tile_id_address = {
                let tile_map_row = tile_y / TILE_SIZE;
                let tile_map_col = u16::from(tile_x) / TILE_SIZE;

                let tile_start_address: u16 = if use_window {
                    self.control.window_map_start.into()
                } else {
                    self.control.bg_map_start.into()
                };

                tile_start_address + tile_map_row * TILE_MAP_WIDTH + tile_map_col
            };

            let tile_id = self.read_byte(tile_id_address);
            let tile_address = self.tile_data_address(tile_id);

            // Find the correct vertical position within the tile. Multiply by two because each
            // row of the tile takes two bytes.
            let tile_line = (tile_y % TILE_SIZE) * 2;

            let shade_number =
                Self::shade_number(self.read_word(tile_address + tile_line as u16), tile_x % 8);

            self.pixels[(screen_y, screen_x)] = self.bg_palette.get(shade_number);
        }
    }
}
