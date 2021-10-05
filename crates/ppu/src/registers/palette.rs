// use modular_bitfield::{
//     bitfield,
//     specifiers::{B2, B4, B6},
// };

// /// BGP - BG Palette Data (R/W) - Non CGB Mode Only
// /// Bit   Name
// /// 7-6 - Color for index 3
// /// 5-4 - Color for index 2
// /// 3-2 - Color for index 1
// /// 1-0 - Color for index 0
//
// /// Value	Color
// /// 0       White
// /// 1       Light gray
// /// 2       Dark gray
// /// 3       Black
//
// #[allow(dead_code)]
// #[bitfield]
// #[derive(Debug, Default, Copy, Clone, PartialEq)]
// pub struct Monochrome {
//     id0: B2,
//     id1: B2,
//     id2: B2,
//     id3: B2,
// }
//
// /// Palette Index - CGB Mode Only
// /// Bit     Value
// /// 0-5     Index (00-3F)
// /// 7       Auto Increment  (0=Disabled, 1=Increment after Writing)
//
// #[allow(dead_code)]
// #[bitfield]
// #[derive(Debug, Default, Copy, Clone, PartialEq)]
// pub struct Index {
//     index: B6,
//     #[skip]
//     _unused: bool,
//     auto_increment: bool,
// }
//
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
