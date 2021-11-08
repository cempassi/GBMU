use super::{CpuMsg, View};
use cpu::registers::{self, Bus};
use iced_graphics::Alignment;
use iced_native::Element;
use iced_wgpu::{Column, Renderer, Row};
use itertools::Itertools;

use crate::{
    debugger::widgets::{Flag, Text},
    style::Theme,
};

pub struct Flags {
    flags: Vec<cpu::registers::Flag>,
}

impl Flags {
    pub fn new(flags: Vec<cpu::registers::Flag>) -> Self {
        Self { flags }
    }

    pub fn view(&self, registers: &cpu::Registers, theme: Theme) -> Element<CpuMsg, Renderer> {
        let title = Text::new("Flags").medium_it(20);
        let mut column = Column::new().push(title).align_items(Alignment::Center);
        for (left, right) in self.flags.iter().tuples() {
            let mut row = Row::new().spacing(15);
            row = row.push(left.view(registers, theme));
            row = row.push(right.view(registers, theme));
            column = column.push(row);
        }
        column.into()
    }
}

impl View<CpuMsg> for registers::Flag {
    fn get_data(&self, registers: &cpu::Registers) -> String {
        format!("{}", registers.get(*self))
    }

    fn get_name(&self) -> String {
        format!("{:?}", *self)
    }

    fn view(&self, registers: &cpu::Registers, _theme: Theme) -> Element<CpuMsg, Renderer> {
        let builder = Flag::new(20, 1, 5);
        builder
            .render(self.get_name(), self.get_data(registers))
            .into()
    }
}
