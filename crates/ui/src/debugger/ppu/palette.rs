//use super::View;
//use iced::{Alignment, Column, Element, Row};
// use ppu::registers::Monochrome;
//
// use super::PpuMsg;
// use crate::{
//     debugger::widgets::{Register, Text},
//     style::Theme,
// };
//
// impl View<PpuMsg> for Monochrome {
//     fn view(&self, _theme: Theme) -> Element<PpuMsg> {
//         let title = Text::new("Palette").medium_it(20);
//         let palette = Column::new().align_items(Alignment::Center).push(title);
//
//         let id0 = format!("{:?}", self.id0);
//         let id0 = Register::render("Index 0:".to_string(), id0);
//
//         let id1 = format!("{:?}", self.id1);
//         let id1 = Register::render("Index 1:".to_string(), id1);
//
//         let id2 = format!("{:?}", self.id2);
//         let id2 = Register::render("Index 2:".to_string(), id2);
//
//         let id3 = format!("{:?}", self.id3);
//         let id3 = Register::render("Index 3:".to_string(), id3);
//
//         let line1 = Row::new().push(id0).push(id1);
//         let line2 = Row::new().push(id2).push(id3);
//         palette.push(line1).push(line2).into()
//     }
// }
