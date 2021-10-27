use crate::style::fonts;
use crate::style::{Register, Style, Theme};
use cpu::Registers;
use iced_wgpu::{Container, Renderer, Row, Text};
use iced_winit::{
    alignment::Alignment, alignment::Horizontal, alignment::Vertical, Element, Length, Space,
};

use cpu::registers::{Bits16, Bits8, Bus};

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

pub trait View {
    fn get_data(&self, registers: Registers) -> String;
    fn get_name(&self) -> String;

    fn view(&self, registers: Registers, theme: Theme) -> Element<RegisterMsg, Renderer>;
}

impl View for Bits8 {
    fn get_data(&self, registers: Registers) -> String {
        if *self == Bits8::F {
            format!("{:04b}", registers.borrow().get(*self))
        } else {
            format!("{:#x}", registers.borrow().get(*self))
        }
    }

    fn get_name(&self) -> String {
        format!("{:?}", self)
    }

    fn view(&self, registers: Registers, theme: Theme) -> Element<RegisterMsg, Renderer> {
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

impl View for Bits16 {
    fn get_data(&self, registers: Registers) -> String {
        format!("{:#x}", registers.borrow().get(*self))
    }

    fn get_name(&self) -> String {
        format!("{:?}", self)
    }

    fn view(&self, registers: Registers, theme: Theme) -> Element<RegisterMsg, Renderer> {
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
