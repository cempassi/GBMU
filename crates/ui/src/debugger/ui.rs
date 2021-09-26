mod memory;
mod registers;

use crate::style::Theme;

use self::memory::{Memory, MemoryMsg};
use iced_wgpu::{Column, Renderer};
use iced_winit::{Command, Element, Program};
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
    Registers(CpuMsg),
    Memory(MemoryMsg),
}

impl Program for UserInterface {
    type Message = Message;
    type Renderer = Renderer;

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        println!("Update of UserInterface reached");
        match message {
            Message::Registers(message) => {
                self.cpu_registers.update(message);
                Command::none()
            }
            Message::Memory(message) => {
                self.memory.update(message);
                Command::none()
            }
        }
    }

    #[allow(clippy::redundant_closure)]
    fn view(&mut self) -> Element<Message, Self::Renderer> {
        let cpu_registers = self
            .cpu_registers
            .view(self.theme)
            .map(|message| Message::Registers(message));
        let memory = self
            .memory
            .view(self.theme)
            .map(|message| Message::Memory(message));
        Column::new().push(cpu_registers).push(memory).into()
    }
}
