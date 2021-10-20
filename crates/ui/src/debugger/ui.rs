use super::memory_map::{Memory, MemoryMsg};
use super::menu::{Menu, MenuMsg};
use super::registers::{CpuMsg, CpuRegisters};
use crate::style::Theme;
use iced_wgpu::{Column, Renderer};
use iced_winit::{Command, Element, Program};
use soc::SOC;
use std::convert::From;

pub struct UserInterface {
    theme: Theme,
    cpu_registers: CpuRegisters,
    memory: Memory,
    menu: Menu,
}

impl From<&SOC> for UserInterface {
    fn from(soc: &SOC) -> UserInterface {
        Self {
            theme: Theme::default(),
            cpu_registers: CpuRegisters::new(soc.get_cpu_registers()),
            memory: <Memory as From<memory::Memory>>::from(soc.get_memory()),
            menu: Menu::new(soc.get_runner()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Registers(CpuMsg),
    Memory(MemoryMsg),
    Menu(MenuMsg),
}

impl From<CpuMsg> for Message {
    fn from(msg: CpuMsg) -> Self {
        Message::Registers(msg)
    }
}

impl From<MemoryMsg> for Message {
    fn from(msg: MemoryMsg) -> Self {
        Message::Memory(msg)
    }
}
impl From<MenuMsg> for Message {
    fn from(msg: MenuMsg) -> Self {
        Message::Menu(msg)
    }
}

impl Program for UserInterface {
    type Message = Message;
    type Renderer = Renderer;

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::Registers(message) => {
                self.cpu_registers.update(message);
                Command::none()
            }
            Message::Memory(message) => {
                self.memory.update(message);
                Command::none()
            }
            Message::Menu(message) => {
                self.menu.update(message);
                Command::none()
            }
        }
    }

    #[allow(clippy::redundant_closure)]
    fn view(&mut self) -> Element<Message, Self::Renderer> {
        let menu = self
            .menu
            .view(self.theme)
            .map(|message| Message::Menu(message));
        let cpu_registers = self
            .cpu_registers
            .view(self.theme)
            .map(|message| Message::Registers(message));
        let memory = self
            .memory
            .view(self.theme)
            .map(|message| Message::Memory(message));
        Column::new()
            .push(menu)
            .push(cpu_registers)
            .push(memory)
            .into()
    }
}
