use super::disassembler::{Disassembler, DisassMsg};
use super::memory_map::{Memory, MemoryMsg};
use super::menu::{Menu, MenuMsg};
use super::registers::{CpuMsg, CpuRegisters};
use crate::style::Theme;
use iced_wgpu::{Column, Renderer, Row};
use iced_winit::{Command, Element, Program};
use soc::SOC;
use std::convert::From;

pub struct UserInterface {
    theme: Theme,
    cpu_registers: CpuRegisters,
    memory: Memory,
    menu: Menu,
    disassembler: Disassembler,
}

impl From<&SOC> for UserInterface {
    fn from(soc: &SOC) -> UserInterface {
        let cpu_registers = soc.get_cpu_registers();
        let memory = soc.get_memory();
        Self {
            theme: Theme::default(),
            cpu_registers: CpuRegisters::new(cpu_registers.clone()),
            memory: <Memory as From<memory::Memory>>::from(memory.clone()),
            menu: Menu::new(soc.get_runner()),
            disassembler: Disassembler::new(cpu_registers, memory),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Registers(CpuMsg),
    Memory(MemoryMsg),
    Menu(MenuMsg),
    Disassembler(DisassMsg),
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
            Message::Disassembler(message) => {
                self.disassembler.update(message);
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
        let disassembler = self
            .disassembler
            .view()
            .map(|message| Message::Disassembler(message));
        let cpu_registers = self
            .cpu_registers
            .view(self.theme)
            .map(|message| Message::Registers(message));
        let row = Row::new().push(cpu_registers).push(disassembler);
        let memory = self
            .memory
            .view(self.theme)
            .map(|message| Message::Memory(message));
        Column::new().push(menu).push(row).push(memory).into()
    }
}
