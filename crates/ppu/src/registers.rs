#![allow(dead_code, unused_attributes, unused_imports)]
pub mod control;
pub mod coordinates;

pub use coordinates::{Coordinates, Field};
use num_enum::TryFromPrimitive;
mod palette;

pub use control::Control;

// /// 1 LCD Control Register
// ///
// ///     1.1 FF40 - LCDC - LCD Control (R/W)
// ///
// /// 2 LCD Status Register
// ///
// ///     2.1 FF41 - STAT - LCDC Status (R/W)
// ///         Bit     Name                                    Usage notes
// ///         6       LYC=LY STAT Interrupt source            (1=Enable) (Read/Write)
// ///         5       Mode 2 OAM STAT Interrupt source        (1=Enable) (Read/Write)
// ///         4       Mode 1 VBlank STAT Interrupt source     (1=Enable) (Read/Write)
// ///         3       Mode 0 HBlank STAT Interrupt source     (1=Enable) (Read/Write)
// ///         2       LYC=LY Flag                             (0=Different, 1=Equal) (Read Only)
// ///         1-0     Mode Flag                               (Mode 0-3, see below) (Read Only)
// ///                 0: HBlank
// ///                 1: VBlank
// ///                 2: Searching OAM
// ///                 3: Transferring Data to LCD Controller
// ///
// /// 4 LCD Position and Scrolling
// ///
// ///     4.1 FF42 - SCY - Scroll Y (R/W), FF43 - SCX - Scroll X (R/W)
// ///     4.2 FF44 - LY - LCDC Y-Coordinate (R)
// ///     4.3 FF45 - LYC - LY Compare (R/W)
// ///     4.4 FF4A - WY - Window Y Position (R/W), FF4B - WX - Window X Position minus 7 (R/W)
// ///
// /// 5 LCD Monochrome Palettes
// ///
// ///     5.1 FF47 - BGP - BG Palette Data (R/W) - Non CGB Mode Only
// ///     5.2 FF48 - OBP0 - Object Palette 0 Data (R/W) - Non CGB Mode Only
// ///     5.3 FF49 - OBP1 - Object Palette 1 Data (R/W) - Non CGB Mode Only
// ///
// /// 6 LCD Color Palettes (CGB only)
// ///
// ///     6.1 FF68 - BCPS/BGPI - CGB Mode Only - Background Palette Index
// ///     6.2 FF69 - BCPD/BGPD - CGB Mode Only - Background Palette Data
// ///     6.3 FF6A - OCPS/OBPI - CGB Mode Only - Sprite Palette Index
// ///     6.4 FF6B - OCPD/OBPD - CGB Mode Only - Sprite Palette Data
// ///
// /// 7 LCD OAM DMA Transfers
// ///
// ///     7.1 FF46 - DMA - DMA Transfer and Start Address (R/W)
// ///
// /// 8 LCD VRAM DMA Transfers (CGB only)
// ///
// ///     8.1 FF51 - HDMA1 - CGB Mode Only - New DMA Source, High
// ///     8.2 FF52 - HDMA2 - CGB Mode Only - New DMA Source, Low
// ///     8.3 FF53 - HDMA3 - CGB Mode Only - New DMA Destination, High
// ///     8.4 FF54 - HDMA4 - CGB Mode Only - New DMA Destination, Low
// ///     8.5 FF55 - HDMA5 - CGB Mode Only - New DMA Length/Mode/Start
// ///         8.5.1 Bit7=0 - General Purpose DMA
// ///         8.5.2 Bit7=1 - H-Blank DMA

#[derive(Debug, Default)]
pub struct Registers {
    pub(super) control: Control,

    // Status
    pub(crate) mode: Mode,
    pub lyc_ly: bool,
    hblank_interupt: bool,
    vblank_interupt: bool,
    oam_interupt: bool,
    lyc_ly_interupt: bool,

    //Lcd Coordinates
    pub(crate) coordinates: Coordinates,
    // bgp: palette::Monochrome,
    // objp0: palette::Monochrome,
    // objp1: palette::Monochrome,
    // bcps: palette::Index,
    // bcpd: palette::Data,
    // ocps: palette::Index,
    // ocpd: palette::Data,

    // dma_transfer: u8,

    // hdma1: u8,
    // hdma2: u8,
    // hdma3: u8,
    // hdma4: u8,
    // hdma5: u8
}

impl Registers {
    fn check_ly(&mut self) {
        self.lyc_ly = self.coordinates.ly_compare();
    }

    pub fn increase(&mut self, field: Field) {
        self.coordinates.increase(field);
        self.check_ly();
    }

    pub fn clear(&mut self, field: Field) {
        self.coordinates.clear(field);
        self.check_ly();
    }

    pub fn is_equal(&mut self, field: Field, data: u8) -> bool {
        self.check_ly();
        self.coordinates.is_equal(field, data)
    }

    pub fn is_lower(&mut self, field: Field, data: u8) -> bool {
        self.check_ly();
        self.coordinates.is_lower(field, data)
    }

    pub fn get(&self, address: u16) -> u8 {
        match address {
            0xFF40 => self.control.get(),
            0xFF41 => {
                let mut byte: u8 = 0;
                byte |= if self.lyc_ly_interupt { 0x40 } else { 0 };
                byte |= if self.oam_interupt { 0x20 } else { 0 };
                byte |= if self.vblank_interupt { 0x10 } else { 0 };
                byte |= if self.hblank_interupt { 0x08 } else { 0 };
                byte |= if self.coordinates.ly_compare() {
                    0x04
                } else {
                    0
                };
                byte |= self.mode.get();
                byte
            }
            0xFF42..=0xFF45 | 0xFF4A | 0xFF4B => {
                let field = Field::try_from_primitive(address).unwrap();
                self.coordinates.get(field)
            }
            _ => unreachable!(),
        }
    }

    pub fn set(&mut self, address: u16, data: u8) {
        match address {
            0xFF40 => {
                let old_lcd = self.control.lcd_enabled;
                self.control.set(data);
                if old_lcd && !self.control.lcd_enabled {
                    self.coordinates.set(Field::Ly, 0);
                    self.mode = Mode::Hblank;
                    // Clear screen and reset clock
                }
            }
            0xFF41 => {
                self.lyc_ly_interupt = data & 0x40 == 0x40;
                self.oam_interupt = data & 0x20 == 0x20;
                self.vblank_interupt = data & 0x10 == 0x10;
                self.hblank_interupt = data & 0x08 == 0x08;
            }
            0xFF42..=0xFF45 | 0xFF4A | 0xFF4B => {
                let field = Field::try_from_primitive(address).unwrap();
                self.coordinates.set(field, data);
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mode {
    Hblank,
    Vblank,
    Oam,
    Transfert,
}

impl Mode {
    fn get(&self) -> u8 {
        match self {
            Mode::Hblank => 0,
            Mode::Vblank => 1,
            Mode::Oam => 2,
            Mode::Transfert => 3,
        }
    }

    fn set(&mut self, byte: u8) {
        *self = match byte {
            0 => Mode::Hblank,
            1 => Mode::Vblank,
            2 => Mode::Oam,
            3 => Mode::Transfert,
            _ => unreachable!(),
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Hblank
    }
}
