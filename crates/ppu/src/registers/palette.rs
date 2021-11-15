use crate::colors::Color;

/// BGP - BG Palette Data (R/W) - Non CGB Mode Only
/// Bit   Name
/// 7-6 - Color for index 3
/// 5-4 - Color for index 2
/// 3-2 - Color for index 1
/// 1-0 - Color for index 0
//
/// Value   Color
/// 0       White
/// 1       Light gray
/// 2       Dark gray
/// 3       Black

#[allow(dead_code)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Monochrome {
    pub id0: Color,
    pub id1: Color,
    pub id2: Color,
    pub id3: Color,
}

impl Monochrome {
    pub fn get(&self) -> u8 {
        let id3: u8 = u8::from(self.id3) << 6;
        let id2: u8 = u8::from(self.id2) << 4;
        let id1: u8 = u8::from(self.id1) << 2;
        id3 | id2 | id1 | u8::from(self.id0)
    }

    pub fn set(&mut self, data: u8) {
        let id3 = (data >> 6) & 0x03;
        let id2 = (data >> 4) & 0x03;
        let id1 = (data >> 2) & 0x03;
        let id0 = data & 0x03;
        self.id3 = Color::from(id3);
        self.id2 = Color::from(id2);
        self.id1 = Color::from(id1);
        self.id0 = Color::from(id0);
    }
}

#[cfg(test)]
mod test_tiles {
    use super::*;
    use shared::{execute, Interrupts};

    #[test]
    fn test_palette_to_u8_black_and_white() {
        let mut palette = Monochrome::default();
        let expected: u8 = 0b11001100;
        // Black - White - Black - White
        palette.set(expected);

        let result = palette.get();
        assert_eq!(
            result, expected,
            "result: {:#b}, expected: {:#b}, palette: {:?}",
            result, expected, palette
        );
    }

    #[test]
    fn test_palette_to_u8_with_light_gray() {
        let mut palette = Monochrome::default();
        let expected: u8 = 0b11101100;
        // Black - LightGray - Black - White
        palette.set(expected);

        let result = palette.get();
        assert_eq!(
            result, expected,
            "result: {:#b}, expected: {:#b}, palette: {:?}",
            result, expected, palette
        );
    }
}
// //
// /// Palette Index - CGB Mode Only
// /// Bit     Value
// /// 0-5     Index (00-3F)
// /// 7       Auto Increment  (0=Disabled, 1=Increment after Writing)
// //
// #[allow(dead_code)]
// #[bitfield]
// #[derive(Debug, Default, Copy, Clone, PartialEq)]
// pub struct Index {
//     index: B6,
//     #[skip]
//     _unused: bool,
//     auto_increment: bool,
// }
// //
// /// Palette Data - CGB Mode Only
// /// Each color is stored as little-endian RGB555.
// /// Bit     Value
// /// 0-4     Red Intensity   (00-1F)
// /// 5-9     Green Intensity (00-1F)
// /// 10-14   Blue Intensity  (00-1F)
// #[allow(dead_code)]
// #[bitfield]
// #[derive(Debug, Default, Copy, Clone, PartialEq)]
// pub struct Data {
//     red: B4,
//     green: B4,
//     blue: B4,
//     #[skip]
//     _unused: B4,
// }
