use super::register::{Merge, RegisterMsg, Split, View};
use crate::style::Theme;
use cpu::area::{Bits16, Bits8};
use cpu::Registers;
use enum_iterator::IntoEnumIterator;
use iced_wgpu::{Checkbox, Column, Renderer, Row};
use iced_winit::{alignment::Alignment, Element, Length, Space};
use itertools::Itertools;

pub struct CpuRegisters {
    registers: Registers,
    ui: Vec<RegisterPair>,
}

#[derive(Debug, Clone)]
pub enum CpuMsg {
    Merge(usize),
}

impl CpuRegisters {
    pub fn new(registers: Registers) -> Self {
        let mut ui = Vec::new();
        for (left, right) in Bits8::into_enum_iter().tuples() {
            ui.push(RegisterPair::Splited(left, right));
        }
        ui.push(RegisterPair::NoSplit(Bits16::SP));
        ui.push(RegisterPair::NoSplit(Bits16::PC));
        Self { registers, ui }
    }

    pub fn update(&mut self, message: CpuMsg) {
        match message {
            CpuMsg::Merge(index) => {
                let ui = self.ui.remove(index);
                self.ui.insert(index, ui.swap());
            }
        }
    }

    pub fn view(&mut self, theme: Theme) -> Element<CpuMsg, Renderer> {
        let column =
            self.ui
                .iter()
                .enumerate()
                .fold(Column::new(), |column, (index, register_ui)| {
                    let element = register_ui.view(self.registers.clone(), theme);
                    column.push(element.map(move |_message| CpuMsg::Merge(index)))
                });
        column.padding(15).spacing(5).into()
    }
}

enum RegisterPair {
    Splited(Bits8, Bits8),
    Merged(Bits16),
    NoSplit(Bits16),
}

impl RegisterPair {
    pub fn swap(self) -> Self {
        match self {
            RegisterPair::Splited(left, right) => RegisterPair::Merged(left.merge(right)),
            RegisterPair::Merged(register) => {
                let (left, right) = register.split();
                RegisterPair::Splited(left, right)
            }
            RegisterPair::NoSplit(register) => RegisterPair::NoSplit(register),
        }
    }

    fn view_register(&self, registers: Registers, theme: Theme) -> Element<RegisterMsg, Renderer> {
        let space = Space::new(Length::Units(10), Length::Units(5));
        match self {
            RegisterPair::Splited(left, right) => Row::new()
                .push(left.view(registers.clone(), theme))
                .push(space)
                .push(right.view(registers, theme))
                .into(),
            RegisterPair::Merged(register) | RegisterPair::NoSplit(register) => {
                register.view(registers, theme)
            }
        }
    }

    pub fn view(&self, registers: Registers, theme: Theme) -> Element<RegisterMsg, Renderer> {
        let checkbox = Checkbox::new(self.is_merged(), "", |_| RegisterMsg::MergeToogle);
        let register = self.view_register(registers, theme);
        let space = Space::new(Length::Units(35), Length::Units(0));
        let row = Row::new().align_items(Alignment::Center);
        let row = match self {
            RegisterPair::Splited(_, _) | RegisterPair::Merged(_) => row.push(checkbox),
            RegisterPair::NoSplit(_) => row.push(space),
        };
        row.push(register).into()
    }

    pub fn is_merged(&self) -> bool {
        match self {
            RegisterPair::Splited(_, _) => false,
            RegisterPair::Merged(_) => true,
            _ => false,
        }
    }
}
