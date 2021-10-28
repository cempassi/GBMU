use super::disassembler::{DisassMsg, Disassembler};
use super::memory_map::{Memory, MemoryMsg};
use super::menu::{Menu, MenuMsg};
use super::registers::{Cpu, CpuMsg, Ppu, PpuMsg};
use crate::style::Theme;
use iced_wgpu::{Column, Renderer, Row};
use iced_winit::{Command, Element, Program};
use soc::SOC;
use std::convert::From;

pub struct UserInterface {
    soc: SOC,
    theme: Theme,
    cpu_registers: Cpu,
    ppu: Ppu,
    memory: Memory,
    menu: Menu,
    disassembler: Disassembler,
}

impl From<SOC> for UserInterface {
    fn from(soc: SOC) -> UserInterface {
        let cpu_registers = soc.get_cpu_registers();
        let memory = soc.get_memory();
        let runner = soc.get_runner();
        let ppu = soc.get_ppu();
        Self {
            soc,
            theme: Theme::default(),
            cpu_registers: Cpu::new(cpu_registers.clone()),
            memory: <Memory as From<memory::Memory>>::from(memory.clone()),
            menu: Menu::new(runner),
            disassembler: Disassembler::new(cpu_registers, memory),
            ppu: Ppu::new(ppu),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Registers(CpuMsg),
    Memory(MemoryMsg),
    Menu(MenuMsg),
    Disassembler(DisassMsg),
    Ppu(PpuMsg),
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

impl UserInterface {
    pub fn refresh(&self) {}
}

impl Program for UserInterface {
    type Message = Message;
    type Renderer = Renderer;

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::Registers(message) => {
                self.cpu_registers.update(message);
            }
            Message::Memory(message) => {
                self.memory.update(message);
            }
            Message::Menu(message) => {
                self.menu.update(message);
            }
            Message::Disassembler(message) => {
                let _ = self.disassembler.update(message);
            }
            Message::Ppu(message) => {
                self.ppu.update(message);
            }
        };
        self.soc.run();
        let _ = self.disassembler.update(DisassMsg::Refresh);
        Command::none()
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
