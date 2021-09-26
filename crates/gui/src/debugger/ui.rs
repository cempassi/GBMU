mod register;

use crate::style::Theme;

use iced_wgpu::{Column, Renderer};
use iced_winit::{Command, Element, Program};
use self::memory::{Memory, MemoryMsg};
use registers::{CpuMsg, CpuRegisters};
use soc::SOC;
use std::convert::From;

pub struct UserInterface {
    theme: Theme,
    cpu_registers: CpuRegisters,
    memory: Memory,
}

impl<'a> From<&SOC<'a>> for UserInterface {
    fn from(soc: &SOC) -> UserInterface {
        Self {
            theme: Theme::default(),
            cpu_registers: CpuRegisters::new(soc.get_cpu_registers()),
            memory: Memory::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ForRegister(register::Message),
}

#[derive(Default)]
pub struct UserInterface {
    registers: register::Registers,
}

impl Program for UserInterface {
    type Clipboard = Clipboard;
    type Message = Message;
    type Renderer = Renderer;

    fn update(
        &mut self,
        message: Message,
        _clipboard: &mut Self::Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::ForRegister(message) => {
                self.registers.update(message);
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Message, Self::Renderer> {
        let column = Column::new()
            .push(Text::new("Hello, world! Are we doing this or what?").color([0.0, 0.0, 1.0]));

        Element::new(column)
    }
}

impl UserInterface {
    // fn title(&self) -> String {
    //     String::from("Hello World")
    // }
}
