use super::register::{Merge, RegisterMsg, Split, View};
use crate::theme::Theme;
use cpu::area::{Bits16, Bits8};
use enum_iterator::IntoEnumIterator;
use iced_wgpu::{Checkbox, Column, Renderer, Row};
use iced_winit::{Align, Element};
use itertools::Itertools;

#[derive(Default)]
pub struct CpuRegisters {
    theme: Theme,
    reg: cpu::Registers,
    registers: Vec<RegisterPair>,
}

#[derive(Debug, Clone)]
pub enum CpuMsg {
    Merge(usize),
}

impl CpuRegisters {
    pub fn new(theme: Theme) -> Self {
        let mut registers = Vec::new();
        for (left, right) in Bits8::into_enum_iter().tuples() {
            registers.push(RegisterPair::Splited(left, right));
        }
        registers.push(RegisterPair::NoSplit(Bits16::SP));
        registers.push(RegisterPair::NoSplit(Bits16::PC));
        let reg = cpu::Registers::default();
        Self {
            theme,
            reg,
            registers,
        }
    }

    pub fn update(&mut self, message: CpuMsg) {
        match message {
            CpuMsg::Merge(index) => {
                let register = self.registers.remove(index);
                self.registers.insert(index, register.swap());
            }
        }
    }

    pub fn view(&mut self) -> Element<CpuMsg, Renderer> {
        let column =
            self.registers
                .iter()
                .enumerate()
                .fold(Column::new(), |column, (index, register)| {
                    let element = register.view(&self.reg, self.theme);
                    column.push(element.map(move |_message| CpuMsg::Merge(index)))
                });
        column.into()
    }
}

enum RegisterPair {
    Splited(Bits8, Bits8),
    Merged(Bits16),
    NoSplit(Bits16)
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

    fn view_register(
        &self,
        registers: &cpu::Registers,
        theme: Theme,
    ) -> Element<RegisterMsg, Renderer> {
        match self {
            RegisterPair::Splited(left, right) => Row::new()
                .push(left.view(registers, theme))
                .push(right.view(registers, theme))
                .into(),
            RegisterPair::Merged(register) | RegisterPair::NoSplit(register)  => register.view(registers, theme),
        }
    }

    pub fn view(&self, registers: &cpu::Registers, theme: Theme) -> Element<RegisterMsg, Renderer> {
        let checkbox = Checkbox::new(self.is_merged(), "", |_| RegisterMsg::MergeToogle);
        let register = self.view_register(registers, theme);
        let row = Row::new().padding(10).spacing(2).align_items(Align::Center);
        let row = match self {
            RegisterPair::Splited(_, _) | RegisterPair::Merged(_) => {
                row.push(checkbox)
            }
            RegisterPair::NoSplit(_) => row,
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
