use crate::fonts;
use crate::theme::Theme;
use iced_wgpu::{Column, Container, Renderer, Row, Text};
use iced_winit::{Align, Element};

#[derive(Default)]
pub struct CpuRegisters {
    theme: Theme,
    registers: Vec<RegisterPair>,
}

enum RegisterPair {
    Splited(Register, Register),
    Merged(Register)
}

impl RegisterPair {
    pub fn view(&self, theme: Theme) -> Element<Message, Renderer> {
        match self {
            RegisterPair::Splited(left, right) => {
                Row::new().push(left.view(theme)).push(right.view(theme)).into()
            },
            RegisterPair::Merged(register) => register.view(theme),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {}

pub struct Register {
    name: String,
    data: u16,
}

impl Register {
    pub fn new(name: &str, data: u16) -> Self {
        Self {
            name: name.to_string(),
            data,
        }
    }

    pub fn view(&self, theme: Theme) -> Element<Message, Renderer> {
        let data = Text::new(self.get_data()).font(fonts::HASKLIG_LIGHT);
        let name = Text::new(self.name.clone()).font(fonts::HASKLIG_BOLD).size(30);
        let number = Container::new(data).style(theme).padding(5);
        Row::new()
            .push(name)
            .push(number)
            .padding(5)
            .spacing(5)
            .align_items(Align::Center)
            .into()
    }

    fn get_data(&self) -> String {
        format!("{:#x}", self.data)
    }
}

impl CpuRegisters {
    pub fn new(theme: Theme) -> Self {
        let a = Register::new("A", 2456);
        let f = Register::new("F", 2456);
        let af = RegisterPair::Splited(a, f);
        let registers = vec![af];
        Self { theme, registers }
    }

    pub fn update(&mut self, _message: Message) {}

    pub fn view(&mut self) -> Element<Message, Renderer> {
        let column = self
            .registers
            .iter()
            .fold(Column::new(), |column, register| {
                column.push(register.view(self.theme))
            });
        column.into()
    }
}
