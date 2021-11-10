use super::PpuMsg;
use super::View;
use crate::debugger::widgets::Register;
use crate::debugger::widgets::Text;
use enum_iterator::IntoEnumIterator;
use iced::{Alignment, Element, Column, Row};
use itertools::Itertools;
use ppu::Coordinates;
use ppu::Field;

impl View<PpuMsg> for Coordinates {
    fn view(&self, _theme: crate::style::Theme) -> Element<PpuMsg> {
        let title = Text::new("Lcd").medium_it(20);
        let mut coordinates = Column::new().align_items(Alignment::Center).push(title);

        for (left, right) in Field::into_enum_iter().tuples() {
            let row = Row::new();

            let name = format!("{}", left);
            let data = self.get(left).to_string();
            let row = row.push(Register::render(name, data));

            let name = format!("{}", right);
            let data = self.get(right).to_string();
            let row = row.push(Register::render(name, data));

            coordinates = coordinates.push(row)
        }

        coordinates.into()
    }
}
