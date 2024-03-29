use super::cpu::{Cpu, CpuMsg};
use super::disassembler::{DisassMsg, Disassembler};
use super::memory::{Memory, MemoryMsg};
use super::menu::{Menu, MenuMsg};
use super::ppu::{Ppu, PpuMsg};
use crate::style::Theme;
use iced_wgpu::{Column, Renderer, Row};
use iced_winit::{Command, Element, Program};
use soc::SOC;
use std::convert::From;

pub struct UserInterface {
    theme: Theme,
    cpu: Cpu,
    ppu: Ppu,
    memory: Memory,
    menu: Menu,
    disassembler: Disassembler,
}

impl From<SOC> for UserInterface {
    fn from(soc: SOC) -> UserInterface {
        let runner = soc.borrow().get_status();
        let ppu = soc.borrow().get_ppu();
        let cpu = soc.borrow().get_cpu();
        let memory = cpu.borrow().get_memory();
        let mut ui = Self {
            theme: Theme::default(),
            cpu: Cpu::new(cpu.clone()),
            memory: Memory::new(memory),
            menu: Menu::new(runner),
            disassembler: Disassembler::new(cpu),
            ppu: Ppu::new(ppu),
        };
        ui.refresh();
        ui
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Registers(CpuMsg),
    Memory(MemoryMsg),
    Menu(MenuMsg),
    Disassembler(DisassMsg),
    Ppu(PpuMsg),
    Refresh,
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
    pub fn refresh(&mut self) {
        let _ = self.disassembler.update(DisassMsg::Refresh);
        self.ppu.update(PpuMsg::Refresh);
        self.cpu.update(CpuMsg::Refresh)
    }
}

impl Program for UserInterface {
    type Message = Message;
    type Renderer = Renderer;

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::Registers(message) => {
                self.cpu.update(message);
            }
            Message::Memory(message) => {
                self.memory.update(message);
            }
            Message::Menu(message) => {
                self.menu.update(message);
            }
            Message::Disassembler(message) => {
                if let DisassMsg::SetBreakpoint(_, address) = message {
                    self.menu.add_breakpoints(address);
                }
                let _ = self.disassembler.update(message);
            }
            Message::Ppu(message) => {
                self.ppu.update(message);
            }
            Message::Refresh => {
                self.refresh();
            }
        };
        Command::none()
    }

    #[allow(clippy::redundant_closure)]
    fn view(&mut self) -> Element<Message, Self::Renderer> {
        let menu = self
            .menu
            .view(self.theme)
            .map(|message| Message::Menu(message));
        let main = Column::new().push(menu);
        let cpu = self
            .cpu
            .view(self.theme)
            .map(|message| Message::Registers(message));
        let disassembler = self
            .disassembler
            .view()
            .map(|message| Message::Disassembler(message));
        let cpu_disass = Row::new().push(cpu).push(disassembler);

        let memory = self
            .memory
            .view(self.theme)
            .map(|message| Message::Memory(message));
        let left = Column::new().push(cpu_disass).push(memory);

        let ppu = self
            .ppu
            .view(self.theme)
            .map(|message| Message::Ppu(message));
        let debugger = Row::new().push(left).push(ppu);
        main.push(debugger).into()
    }
}
