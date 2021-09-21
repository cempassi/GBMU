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

impl Program for UserInterface {
    type Message = Message;
    type Renderer = Renderer;

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        println!("Update of UserInterface reached");
        match message {
            Message::ForRegister(message) => {
                self.registers.update(message);
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Message, Self::Renderer> {
        self.registers
            .view()
            .map(|message| Message::ForRegister(message))
    }
}
