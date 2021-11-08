use crate::style::fonts;
use crate::style::{Register, Style, Theme};
use iced_wgpu::{Checkbox, Container, Renderer, Row, Text};
use iced_winit::{
    alignment::Alignment, alignment::Horizontal, alignment::Vertical, Element, Length, Space,
};

use super::View;

use cpu::registers::{Bits16, Bits8, Bus};

pub enum Registers {
    Splited(Bits8, Bits8),
    Merged(Bits16),
    NoSplit(Bits16),
}

impl Registers {
    pub fn swap(self) -> Self {
        match self {
            Registers::Splited(left, right) => Registers::Merged(left.merge(right)),
            Registers::Merged(register) => {
                let (left, right) = register.split();
                Registers::Splited(left, right)
            }
            Registers::NoSplit(register) => Registers::NoSplit(register),
        }
    }

    fn view_register(
        &self,
        registers: &cpu::Registers,
        theme: Theme,
    ) -> Element<RegisterMsg, Renderer> {
        let space = Space::new(Length::Units(10), Length::Units(5));
        match self {
            Registers::Splited(left, right) => Row::new()
                .push(left.view(registers, theme))
                .push(space)
                .push(right.view(registers, theme))
                .into(),
            Registers::Merged(register) | Registers::NoSplit(register) => {
                register.view(registers, theme)
            }
        }
    }

    fn is_merged(&self) -> bool {
        match self {
            Registers::Splited(_, _) => false,
            Registers::Merged(_) => true,
            _ => false,
        }
    }

    pub fn view(&self, registers: &cpu::Registers, theme: Theme) -> Element<RegisterMsg, Renderer> {
        let checkbox = Checkbox::new(self.is_merged(), "", |_| RegisterMsg::MergeToogle);
        let register = self.view_register(registers, theme);
        let space = Space::new(Length::Units(35), Length::Units(0));
        let row = Row::new().align_items(Alignment::Center).width(Length::Shrink);
        let row = match self {
            Registers::Splited(_, _) | Registers::Merged(_) => row.push(checkbox),
            Registers::NoSplit(_) => row.push(space),
        };
        row.push(register).into()
    }
}

#[derive(Debug, Clone)]
pub enum RegisterMsg {
    MergeToogle,
}

pub trait Merge {
    type Output;

    fn merge(self, right: Self) -> Self::Output;
}

impl Merge for Bits8 {
    type Output = Bits16;

    fn merge(self, right: Self) -> Self::Output {
        match self {
            Bits8::A if right == Bits8::F => Bits16::AF,
            Bits8::B if right == Bits8::C => Bits16::BC,
            Bits8::D if right == Bits8::E => Bits16::DE,
            Bits8::H if right == Bits8::L => Bits16::HL,
            _ => unreachable!(),
        }
    }
}

pub trait Split {
    type Output;

    fn split(self) -> Self::Output;
}

impl Split for Bits16 {
    type Output = (Bits8, Bits8);

    fn split(self) -> Self::Output {
        match self {
            Bits16::AF => (Bits8::A, Bits8::F),
            Bits16::BC => (Bits8::B, Bits8::C),
            Bits16::DE => (Bits8::D, Bits8::E),
            Bits16::HL => (Bits8::H, Bits8::L),
            _ => unreachable!(),
        }
    }
}


impl View<RegisterMsg> for Bits8 {
    fn get_data(&self, registers: &cpu::Registers) -> String {
        if *self == Bits8::F {
            format!("{:04b}", registers.get(*self))
        } else {
            format!("{:#x}", registers.get(*self))
        }
    }

    fn get_name(&self) -> String {
        format!("{:?}", self)
    }

    fn view(&self, registers: &cpu::Registers, theme: Theme) -> Element<RegisterMsg, Renderer> {
        let name = Text::new(self.get_name())
            .font(fonts::HASKLIG_BOLD)
            .size(20);
        let space = Space::new(Length::Units(20), Length::Units(5));
        let data = Text::new(self.get_data(registers))
            .font(fonts::HASKLIG_LIGHT)
            .width(Length::Fill)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center);
        let number = Container::new(data)
            .style(Register::style(theme))
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .width(Length::Units(50))
            .height(Length::Units(25));
        Row::new()
            .push(name)
            .push(space)
            .push(number)
            .align_items(Alignment::Center)
            .into()
    }
}

impl View<RegisterMsg> for Bits16 {
    fn get_data(&self, registers: &cpu::Registers) -> String {
        format!("{:#x}", registers.get(*self))
    }

    fn get_name(&self) -> String {
        format!("{:?}", self)
    }

    fn view(&self, registers: &cpu::Registers, theme: Theme) -> Element<RegisterMsg, Renderer> {
        let name = Text::new(self.get_name())
            .font(fonts::HASKLIG_BOLD)
            .size(20);
        let space = Space::new(Length::Units(10), Length::Units(5));
        let data = Text::new(self.get_data(registers))
            .font(fonts::HASKLIG_LIGHT)
            .width(Length::Fill)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center);
        let number = Container::new(data)
            .style(Register::style(theme))
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .width(Length::Units(140))
            .height(Length::Units(25));
        Row::new()
            .push(name)
            .push(space)
            .push(number)
            .align_items(Alignment::Center)
            .into()
    }
}
