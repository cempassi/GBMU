//! Graphics-related functionality.
//!
//! Contains an implementation of a PPU.

use std::fmt::{self, Debug};
use std::ops::{Index, IndexMut};

use byteorder::{ByteOrder, LittleEndian};
use derivative::Derivative;
use num_enum::IntoPrimitive;

use crate::bytes::ByteExt;
use shared::{Interrupt, Interrupts};

use crate::colors;

pub use colors::{BackgroundPalette, Shade, SpritePalette};

/// The width and height of the Game Boy screen.
pub const SCREEN_DIMENSIONS: (u32, u32) = (SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
/// The address that OAM starts at.
pub const SPRITE_START: u16 = 0xFE00;
pub const SPRITE_TILE_DATA_START: u16 = 0x8000;

/// Memory managed by the PPU.
struct Memory {
    /// Background data, split into two overlapping 1024 byte maps.
    ///
    /// Each byte in the map represents an 8x8 pixel space on the display, referring to tile data
    /// stored in the Character RAM. Each total map is 32x32 tiles.
    bg_map: Vec<u8>,

    /// Character RAM, storing 8x8 pixel tile data.
    ///
    /// Each pixel has two bits of color data, so each tile is 16 bytes long. This area is
    /// divided into signed and unsigned tiles: unsigned are numbered 0-255 at $8000-$9000.
    /// Signed tiles are numbered in two's complement from -127-128 at $87FF-$97FF.
    chram: Vec<u8>,

    /// Object attribute memory (OAM).
    oam: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Memory {
        Memory {
            bg_map: vec![0; 0x800],
            chram: vec![0; 0x1800],
            oam: vec![0; 0xA0],
        }
    }
}

/// Determines under which conditions the LCDC Status register (0xFF41) will fire.
#[derive(Debug, Default)]
pub struct LcdcStatusInterrupts {
    /// Fires during H-Blank.
    pub hblank: bool,

    /// Fires during V-Blank.
    pub vblank: bool,

    /// Fires when OAM is being transferred.
    pub oam: bool,

    /// Fires when LYC = LY (i.e., 0xFF45 = 0xFF44).
    pub ly_lyc_coincidence: bool,
}

/// The location of the window or background tile map.
#[derive(Debug, Copy, Clone, PartialEq, Eq, IntoPrimitive)]
#[repr(u16)]
pub enum TileMapStart {
    /// The low tile map (0x9800).
    Low = 0x9800,

    /// The high tile map (0x9C00).
    High = 0x9C00,
}

impl Default for TileMapStart {
    fn default() -> Self {
        TileMapStart::Low
    }
}

/// The location of the window or background tile data.
#[derive(Debug, Copy, Clone, PartialEq, Eq, IntoPrimitive)]
#[repr(u16)]
pub enum TileDataStart {
    /// The low address (0x8000). Offsets are unsigned.
    Low = 0x8000,

    /// The high address (0x8800). Offsets are signed.
    High = 0x8800,
}

impl Default for TileDataStart {
    fn default() -> Self {
        TileDataStart::High
    }
}

/// The available sizes of sprites.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SpriteSize {
    /// 8x8
    Small,

    /// 8x16
    Large,
}

impl Default for SpriteSize {
    fn default() -> Self {
        SpriteSize::Small
    }
}

/// Core LCD settings.
#[derive(Debug, Default)]
pub struct LcdControl {
    /// Whether the LCD is operating.
    pub display_enabled: bool,

    /// True if window memory should be displayed.
    pub window_enabled: bool,

    /// True if sprites should be displayed.
    pub sprites_enabled: bool,

    /// True if the background should be displayed.
    pub background_enabled: bool,

    /// The address of the start of the window tile map.
    pub window_map_start: TileMapStart,

    /// The address of the start of the background and window tile data.
    pub tile_data_start: TileDataStart,

    /// The address of the start of the background tile map.
    pub bg_map_start: TileMapStart,

    /// The size of the sprites being used.
    pub sprite_size: SpriteSize,
}

/// An X/Y coordinate pair.
#[derive(Debug, Default)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

#[derive(Clone, Derivative)]
#[derivative(Debug, Default)]
pub struct ScreenBuffer(
    #[derivative(Debug = "ignore")]
    #[derivative(Default(value = "[[Shade::default(); SCREEN_WIDTH]; SCREEN_HEIGHT]"))]
    [[Shade; SCREEN_WIDTH]; SCREEN_HEIGHT],
);

impl Index<(u8, u8)> for ScreenBuffer {
    type Output = Shade;
    fn index(&self, (y, x): (u8, u8)) -> &Self::Output {
        &self.0[usize::from(y)][usize::from(x)]
    }
}

impl IndexMut<(u8, u8)> for ScreenBuffer {
    fn index_mut(&mut self, (y, x): (u8, u8)) -> &mut Self::Output {
        &mut self.0[usize::from(y)][usize::from(x)]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Mode {
    /// Horizontal blank (HBLANK).
    HorizontalBlank = 0,

    /// Vertical blank (VBLANK).
    VerticalBlank = 1,

    /// Accessing OAM.
    ScanlineOam = 2,

    /// Accessing VRAM and drawing.
    ScanlineVram = 3,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::ScanlineOam
    }
}

/// The picture processing unit.
#[derive(Debug, Default)]
pub struct Ppu {
    mem: Memory,

    pub control: LcdControl,

    /// The current mode number of the PPU operation.
    mode: Mode,

    /// The number of PPU clock cycles that have been executed for the current
    /// PPU operation.
    modeclock: u32,

    /// The background palette.
    pub bg_palette: BackgroundPalette,

    /// The two object palettes.
    pub sprite_palette: [SpritePalette; 2],

    /// The current line position of the PPU. The last line is 143.
    pub line: u8,

    /// The position in the 256x256 background tile map that should be displayed from the upper
    /// left.
    pub bg_scroll: Position,

    /// The upper/left position of the window area. The window area is an alternate background
    /// area which can be displayed above the normal background. Sprites may be displayed above
    /// or behind the window.
    pub window: Position,

    /// A value that is compared against the current line.
    ///
    /// Used by the LCDC status and LYC I/O registers.
    pub line_compare: u8,

    /// Contains conditions under which the LCDC Status register will fire.
    pub lcd_status_interrupts: LcdcStatusInterrupts,

    /// The frame to be rendered.
    frame: ScreenBuffer,

    /// The pixels to be rendered on a frame.
    pixels: ScreenBuffer,
}

impl AsRef<Vec<u8>> for Ppu {
    fn as_ref(&self) -> &Vec<u8> {
        self.mem.chram.as_ref()
    }
}

impl Ppu {
    /// Creates a new picture processing unit.
    ///
    /// The initial contents of the memory are unspecified.
    pub fn new() -> Ppu {
        Ppu::default()
    }

    /// Render the current frame into a frame buffer.
    ///
    /// Assumes the default texture format of [`wgpu::TextureFormat::Rgba8UnormSrgb`].
    pub fn render(&self, frame: &mut [u8]) {
        if self.lcd_status_interrupts.vblank {
            for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                let x = i % SCREEN_WIDTH;
                let y = i / SCREEN_WIDTH;

                let shade = self.frame.0[y][x];
                pixel.copy_from_slice(shade.as_rgba());
            }
        }
    }

    /// Performs one clock step of the PPU.
    pub fn step(&mut self, interrupts: &Interrupts) {
        self.modeclock += 1;

        // Mode changes are a state machine. This match block returns an option indicating whether
        // there was a mode change, and if there was, the new mode.
        println!("modeclock: {}", self.modeclock);
        let new_mode = match self.mode {
            Mode::HorizontalBlank if self.modeclock >= 204 => {
                self.modeclock = 0;
                self.line += 1;

                if self.lcd_status_interrupts.ly_lyc_coincidence && self.line == self.line_compare {
                    interrupts.borrow_mut().request(Interrupt::Lcd);
                }

                if self.line > 143 {
                    // Push the pixels to a frame.
                    self.frame = self.pixels.clone();
                    Some(Mode::VerticalBlank)
                } else {
                    Some(Mode::ScanlineOam)
                }
            }

            Mode::VerticalBlank if self.modeclock >= 456 => {
                self.modeclock = 0;
                self.line += 1;

                if self.line > 153 {
                    self.line = 0;
                    Some(Mode::ScanlineOam)
                } else {
                    None
                }
            }

            Mode::ScanlineOam if self.modeclock >= 80 => {
                self.modeclock = 0;
                Some(Mode::ScanlineVram)
            }

            Mode::ScanlineVram if self.modeclock >= 172 => {
                self.modeclock = 0;

                // Write a scanline to the framebuffer
                self.renderscan();

                Some(Mode::HorizontalBlank)
            }

            _ => None,
        };

        if let Some(new_mode) = new_mode {
            self.mode = new_mode;

            match new_mode {
                Mode::HorizontalBlank => {
                    if self.lcd_status_interrupts.hblank {
                        interrupts.borrow_mut().request(Interrupt::Lcd);
                    }
                }
                Mode::VerticalBlank => {
                    interrupts.borrow_mut().request(Interrupt::VBlank);
                    if self.lcd_status_interrupts.vblank {
                        interrupts.borrow_mut().request(Interrupt::Lcd);
                    }
                }
                Mode::ScanlineOam => {
                    if self.lcd_status_interrupts.oam {
                        interrupts.borrow_mut().request(Interrupt::Lcd);
                    }
                }
                _ => (),
            }
        }
    }

    /// Returns the number of the current scanline.
    pub fn line(&self) -> u8 {
        if self.control.display_enabled {
            self.line
        } else {
            0
        }
    }

    /// Returns the number of the current graphics mode.
    pub fn mode(&self) -> u8 {
        if self.control.display_enabled {
            self.mode as u8
        } else {
            Mode::HorizontalBlank as u8
        }
    }

    /// Renders the screen one line at a time. Move tile-by-tile through the line until it is
    /// complete.
    pub fn renderscan(&mut self) {
        if !self.control.display_enabled {
            return;
        }

        if self.control.background_enabled {
            self.render_tiles();
        }

        if self.control.sprites_enabled {
            self.render_sprites();
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
            println!("Loop in render tiles");
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

            let tile_id = self.get(tile_id_address);
            let tile_address = self.tile_data_address(tile_id);

            // Find the correct vertical position within the tile. Multiply by two because each
            // row of the tile takes two bytes.
            let tile_line = (tile_y % TILE_SIZE) * 2;

            let shade_number =
                Self::shade_number(self.read_word(tile_address + tile_line as u16), tile_x % 8);

            self.pixels[(screen_y, screen_x)] = self.bg_palette.get(shade_number);
        }
    }

    /// Given a tile identifier, returns the starting address of the tile.
    ///
    /// The tile identifier may be interpreted as signed or unsigned depending on the tile map
    /// being used.
    fn tile_data_address(&self, tile_id: u8) -> u16 {
        const SIGNED_TILE_OFFSET: i16 = 128;
        const TILE_DATA_ROW_SIZE: u16 = 16;

        let start = self.control.tile_data_start;

        // Depending on which tile map we're using, the offset can be signed or unsigned.
        let offset = match start {
            TileDataStart::Low => tile_id.into(),
            TileDataStart::High => (i16::from(tile_id as i8) + SIGNED_TILE_OFFSET) as u16,
        };

        u16::from(start) + offset * TILE_DATA_ROW_SIZE
    }

    /// Gets the shade number for a particular pixel on the screen.
    fn shade_number(tile_row: u16, tile_x: u8) -> u8 {
        // Every two bytes represents one row of 8 pixels. The bits of each byte correspond to one
        // pixel. The first byte contains the lower order bit of the color number, while the second
        // byte contains the higher order bit.
        let mut bytes = [0; 2];
        LittleEndian::write_u16(&mut bytes, tile_row);

        // Convert x-position into bit position (bit 7 is leftmost bit).
        let color_bit = 7 - tile_x;

        let mut color_num = 0;
        color_num.set_bit(0, bytes[0].has_bit_set(color_bit));
        color_num.set_bit(1, bytes[1].has_bit_set(color_bit));
        color_num
    }

    /// Read the information for the nth sprite in OAM.
    fn read_sprite(&self, index: u8) -> Sprite {
        let oam_address = SPRITE_START + u16::from(index * 4);

        let y = self.get(oam_address).wrapping_sub(16);
        let x = self.get(oam_address + 1).wrapping_sub(8);
        let tile_number = {
            let number = self.get(oam_address + 2);
            match self.control.sprite_size {
                SpriteSize::Large => number & 0xFE,
                SpriteSize::Small => number,
            }
        };
        let attributes = self.get(oam_address + 3);

        Sprite {
            index,
            x,
            y,
            tile_number,
            attributes,
            size: self.control.sprite_size,
        }
    }

    /// Render sprites for the current scanline on the screen.
    pub fn render_sprites(&mut self) {
        let mut sprites_on_scanline = (0..40)
            .map(|index| self.read_sprite(index))
            .filter(|sprite| sprite.y <= self.line && self.line <= sprite.y + sprite.height())
            .collect::<Vec<_>>();

        // Sort sprites by priority. Sprites with lower X-coordinates are prioritized. If sprites
        // have the same X-coordinate, sprites earlier in OAM have priority.
        sprites_on_scanline.sort_by_key(|sprite| (sprite.x, sprite.index));

        // 10 sprites per line.
        sprites_on_scanline.truncate(10);

        for sprite in sprites_on_scanline.iter().rev() {
            // Determine the background priority of the sprite
            let behind_bg = sprite.attributes.has_bit_set(7);

            // Determine whether the sprite is flipped horizontally or vertically
            let y_flip = sprite.attributes.has_bit_set(6);
            let x_flip = sprite.attributes.has_bit_set(5);

            // Get the line of the sprite to be displayed
            let current_line = if y_flip {
                (i16::from(sprite.y) + i16::from(sprite.height()) - i16::from(self.line)) * 2
            } else {
                (i16::from(self.line) - i16::from(sprite.y)) * 2
            };

            // Get the address of the color information within the sprite tile data. The color
            // is stored as two bytes corresponding to an 8-pixel line, as with background
            // tiles.
            let data_address: u16 = (SPRITE_TILE_DATA_START + (u16::from(sprite.tile_number) * 16))
                + current_line as u16;
            let color_row = self.read_word(data_address);

            // Find the shade for each pixel in the line
            for tile_pixel in (0..8).rev() {
                // Get the bit that corresponds to the pixel within the line
                let color_bit = if x_flip {
                    tile_pixel as u8
                } else {
                    (7 - tile_pixel as i8) as u8
                };

                // Determine which sprite palette to use
                let sprite_palette = if sprite.attributes.has_bit_set(4) {
                    &self.sprite_palette[1]
                } else {
                    &self.sprite_palette[0]
                };

                // Find the horizontal position of the pixel on the screen
                let x_pixel: u8 = (7 - (tile_pixel as i8)) as u8;
                let pixel = sprite.x.wrapping_add(x_pixel);

                // Bail if the pixel isn't on the screen.
                if pixel >= SCREEN_WIDTH as u8 {
                    continue;
                }

                let shade_number = Self::shade_number(color_row, color_bit);

                if let Some(shade) = sprite_palette.get(shade_number) {
                    if !behind_bg || self.pixels[(self.line, pixel)] == Shade::White {
                        self.pixels[(self.line, pixel)] = shade;
                    }
                }
            }
        }
    }
}

impl Ppu {
    /// Reads a byte of graphics memory.
    ///
    /// # Panics
    ///
    /// Panics if reading memory that is not managed by the PPU.
    fn get(&self, address: u16) -> u8 {
        match address {
            0x8000..=0x97FF => {
                let index = address - 0x8000;
                self.mem.chram[index as usize]
            }

            0x9800..=0x9FFF => {
                let index = address - 0x9800;
                self.mem.bg_map[index as usize]
            }

            0xFE00..=0xFE9F => {
                let index = address - 0xFE00;
                self.mem.oam[index as usize]
            }

            // LCDC - LCD Control
            0xFF40 => {
                let mut register = 0u8;
                register.set_bit(7, self.control.display_enabled);
                register.set_bit(6, self.control.window_map_start != TileMapStart::default());
                register.set_bit(5, self.control.window_enabled);
                register.set_bit(4, self.control.tile_data_start != TileDataStart::default());
                register.set_bit(3, self.control.bg_map_start != TileMapStart::default());
                register.set_bit(2, self.control.sprite_size != SpriteSize::default());
                register.set_bit(1, self.control.sprites_enabled);
                register.set_bit(0, self.control.background_enabled);
                register
            }

            // STAT - LCDC Status
            0xFF41 => {
                let mut register = 0u8;

                // Set the lowest two bits to the mode.
                register |= self.mode();

                // Set bit 2 if LY == LYC
                register.set_bit(2, self.line == self.line_compare);

                // Other bits are set if the various interrupts are enabled.
                register.set_bit(3, self.lcd_status_interrupts.hblank);
                register.set_bit(4, self.lcd_status_interrupts.vblank);
                register.set_bit(5, self.lcd_status_interrupts.oam);
                register.set_bit(6, self.lcd_status_interrupts.ly_lyc_coincidence);

                // The highest bit is unspecified.

                register
            }

            // SCY - Scroll Y
            0xFF42 => self.bg_scroll.y,

            // SCX - Scroll X
            0xFF43 => self.bg_scroll.x,

            // LCDC Y-Coordinate
            0xFF44 => self.line(),

            // LYC - LY Compare
            0xFF45 => self.line_compare,

            // DMA Transfer
            0xFF46 => unreachable!("handled in bus"),

            // BGP - BG Palette Data
            0xFF47 => self.bg_palette.as_byte(),

            // OBP0 - Object Palette 0 Data
            0xFF48 => self.sprite_palette[0].as_byte(),

            // OBP1 - Object Palette 1 Data
            0xFF49 => self.sprite_palette[1].as_byte(),

            // WY - Window Y Position
            0xFF4A => self.window.y,

            // WX - Window X Position minus 7
            0xFF4B => self.window.x.wrapping_add(7),

            _ => panic!("read out-of-range address in PPU: {:#0x}", address),
        }
    }

    /// Writes a byte of graphics memory.
    ///
    /// # Panics
    ///
    /// Panics if writing memory that is not managed by the PPU.
    fn set(&mut self, address: u16, byte: u8) {
        match address {
            0x8000..=0x97FF => {
                let index = address - 0x8000;
                self.mem.chram[index as usize] = byte;
            }

            0x9800..=0x9FFF => {
                let index = address - 0x9800;
                self.mem.bg_map[index as usize] = byte;
            }

            0xFE00..=0xFE9F => {
                let index = address & 0xFF;
                self.mem.oam[index as usize] = byte;
            }

            // LCDC - LCD Control
            0xFF40 => {
                self.control.display_enabled = byte.has_bit_set(7);
                self.control.window_map_start = if byte.has_bit_set(6) {
                    TileMapStart::High
                } else {
                    TileMapStart::Low
                };
                self.control.window_enabled = byte.has_bit_set(5);
                self.control.tile_data_start = if byte.has_bit_set(4) {
                    TileDataStart::Low
                } else {
                    TileDataStart::High
                };
                self.control.bg_map_start = if byte.has_bit_set(3) {
                    TileMapStart::High
                } else {
                    TileMapStart::Low
                };
                self.control.sprite_size = if byte.has_bit_set(2) {
                    SpriteSize::Large
                } else {
                    SpriteSize::Small
                };
                self.control.sprites_enabled = byte.has_bit_set(1);
                self.control.background_enabled = byte.has_bit_set(0);
            }

            // STAT - LCDC Status
            0xFF41 => {
                self.lcd_status_interrupts.hblank = byte.has_bit_set(3);
                self.lcd_status_interrupts.vblank = byte.has_bit_set(4);
                self.lcd_status_interrupts.oam = byte.has_bit_set(5);
                self.lcd_status_interrupts.ly_lyc_coincidence = byte.has_bit_set(6);
            }

            // SCY - Scroll Y
            0xFF42 => self.bg_scroll.y = byte,

            // SCX - Scroll X
            0xFF43 => self.bg_scroll.x = byte,

            // LY - LCDC Y-Coordinate (Read-only),
            0xFF44 => (),

            // LYC - LY Compare
            0xFF45 => self.line_compare = byte,

            // DMA Transfer
            0xFF46 => unreachable!("handled in bus"),

            // BGP - BG Palette Data
            0xFF47 => self.bg_palette = byte.into(),

            // OBP0 - Object Palette 0 Data
            0xFF48 => self.sprite_palette[0] = byte.into(),

            // OBP1 - Object Palette 1 Data
            0xFF49 => self.sprite_palette[1] = byte.into(),

            // WY - Window Y position
            0xFF4A => self.window.y = byte,

            // WB - Window X position minus 7
            0xFF4B => self.window.x = byte.wrapping_sub(7),

            _ => panic!("write out-of-range address in PPU"),
        }
    }

    fn read_word(&self, address: u16) -> u16 {
        LittleEndian::read_u16(&[self.get(address), self.get(address + 1)])
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let chram: &[u8] = &self.chram;
        let bg_map: &[u8] = &self.bg_map;
        let oam: &[u8] = &self.oam;

        f.debug_struct("Memory")
            .field("chram", &chram)
            .field("bg_map", &bg_map)
            .field("oam", &oam)
            .finish()
    }
}

/// A sprite read from memory.
#[derive(Debug)]
struct Sprite {
    /// The index of the sprite in OAM.
    index: u8,

    /// The Y-position of the sprite on the screen.
    y: u8,

    /// The X-position of the sprite on the screen.
    x: u8,

    /// Which tile number should be used for the sprite.
    tile_number: u8,

    /// Sprite attributes.
    attributes: u8,

    /// The sprite size at the time of reading the sprite.
    size: SpriteSize,
}

impl Sprite {
    /// The height of the sprite, in pixels.
    fn height(&self) -> u8 {
        match self.size {
            SpriteSize::Small => 7,
            SpriteSize::Large => 15,
        }
    }
}
