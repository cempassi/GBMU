mod flags;
mod registers;

use crate::debugger::widgets::Text;
use crate::style::Theme;
use cpu::registers::{Bits16, Bits8, Flag};
use enum_iterator::IntoEnumIterator;
use iced::{alignment::Alignment, Column, Element};
use itertools::Itertools;
use registers::Registers;

use flags::Flags;

pub trait View<T> {
    fn get_data(&self, registers: &cpu::Registers) -> String;
    fn get_name(&self) -> String;

    fn view(&self, registers: &cpu::Registers, theme: Theme) -> Element<T>;
}

pub struct Cpu {
    cpu: cpu::Cpu,
    data: cpu::Registers,
    registers: Vec<Registers>,
    flags: Flags,
}

#[derive(Debug, Clone)]
pub enum CpuMsg {
    Merge(usize),
    Refresh,
}

impl Cpu {
    pub fn new(cpu: cpu::Cpu) -> Self {
        let mut data = cpu::Registers::default();
        let mut registers = Vec::new();
        cpu.borrow().registers.update(&mut data);
        for (left, right) in Bits8::into_enum_iter().tuples() {
            if left == Bits8::H {
                registers.push(Registers::Merged(Bits16::HL));
            } else {
                registers.push(Registers::Splited(left, right));
            }
        }
        let mut flags = Vec::new();
        for flag in Flag::into_enum_iter() {
            flags.push(flag);
        }
        let flags = Flags::new(flags);
        registers.push(Registers::NoSplit(Bits16::SP));
        registers.push(Registers::NoSplit(Bits16::PC));
        Self {
            cpu,
            data,
            registers,
            flags,
        }
    }

    pub fn update(&mut self, message: CpuMsg) {
        self.cpu.borrow().registers.update(&mut self.data);
        match message {
            CpuMsg::Merge(index) => {
                let ui = self.registers.remove(index);
                self.registers.insert(index, ui.swap());
            }
            CpuMsg::Refresh => (),
        }
    }

    pub fn view(&mut self, theme: Theme) -> Element<CpuMsg> {
        let title = Text::new("Cpu").medium_it(20);
        let cpu = Column::new().push(title).align_items(Alignment::Center);
        let registers = self.registers.iter().enumerate().fold(
            Column::new().padding(15).spacing(5),
            |column, (index, ui)| {
                let element = ui.view(&self.data, theme);
                column.push(element.map(move |_message| CpuMsg::Merge(index)))
            },
        );
        let flags = self.flags.view(&self.data, theme);
        cpu.push(registers).push(flags).into()
    }
}
