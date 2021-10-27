pub(crate) mod lcd;
mod palette;

pub use lcd::Lcd;

// /// 1 LCD Control Register
// ///
// ///     1.1 FF40 - LCDC - LCD Control (R/W)
// /// 2 LCD Status Register
// ///
// ///     2.1 FF41 - STAT - LCDC Status (R/W)
// /// 4 LCD Position and Scrolling
// ///
// ///     4.1 FF42 - SCY - Scroll Y (R/W), FF43 - SCX - Scroll X (R/W)
// ///     4.2 FF44 - LY - LCDC Y-Coordinate (R)
// ///     4.3 FF45 - LYC - LY Compare (R/W)
// ///     4.4 FF4A - WY - Window Y Position (R/W), FF4B - WX - Window X Position minus 7 (R/W)
// /// 5 LCD Monochrome Palettes
// ///
// ///     5.1 FF47 - BGP - BG Palette Data (R/W) - Non CGB Mode Only
// ///     5.2 FF48 - OBP0 - Object Palette 0 Data (R/W) - Non CGB Mode Only
// ///     5.3 FF49 - OBP1 - Object Palette 1 Data (R/W) - Non CGB Mode Only
// /// 6 LCD Color Palettes (CGB only)
// ///
// ///     6.1 FF68 - BCPS/BGPI - CGB Mode Only - Background Palette Index
// ///     6.2 FF69 - BCPD/BGPD - CGB Mode Only - Background Palette Data
// ///     6.3 FF6A - OCPS/OBPI - CGB Mode Only - Sprite Palette Index
// ///     6.4 FF6B - OCPD/OBPD - CGB Mode Only - Sprite Palette Data
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
    pub lcd: Lcd, // // bgp: palette::Monochrome,
                  // // objp0: palette::Monochrome,
                  // // objp1: palette::Monochrome,

                  // // bcps: palette::Index,
                  // // bcpd: palette::Data,
                  // // ocps: palette::Index,
                  // // ocpd: palette::Data,

                  //dma_transfer: u8,

                  // // hdma1: u8,
                  // hdma2: u8,
                  // hdma3: u8,
                  // hdma4: u8,
                  // hdma5: u8
}
