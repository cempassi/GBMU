use crate::debugger::widgets::Text;
use iced_graphics::Alignment;
use iced_wgpu::{Column, Renderer};
use iced_winit::Element;

use cpu::Registers;
use memory::Memory;
mod conversion;
mod header;
mod instruction;

use header::Header;
use instruction::Instruction;

pub struct Disassembler {
    header: Header,
    registers: Registers,
    memory: Memory,
    instructions: Vec<Option<Instruction>>,
    next: u16,
}

#[derive(Debug, Clone)]
pub enum DisassMsg {
    Refresh,
}

impl Disassembler {
    pub fn new(registers: Registers, memory: Memory) -> Self {
        let instructions = Vec::new();
        let header = Header::new();
        let next = 0;

        let mut disassembler = Self {
            header,
            registers,
            memory,
            instructions,
            next,
        };
        disassembler.update(DisassMsg::Refresh);
        disassembler
    }

    fn update_instructions(&mut self) {
        let mut pc = self.registers.borrow().pc;
        if pc != self.next {
            return;
        }
        self.instructions.clear();
        for id in 0..5 {
            if let Ok(instruction) = Instruction::try_new(pc, &self.memory, false) {
                pc += instruction.fetched();
                if id == 0 {
                    self.next = pc;
                }
                self.instructions.push(Some(instruction));
            } else {
                self.instructions.push(None);
            };
        }
    }

    pub fn update(&mut self, _message: DisassMsg) {
        self.update_instructions();
    }

    pub fn reload(&mut self) {
        let mut pc = self.next;
        self.instructions.clear();
        for id in 0..5 {
            if let Ok(instruction) = Instruction::try_new(pc, &self.memory, false) {
                pc += instruction.fetched();
                if id == 0 {
                    self.next = pc;
                }
                self.instructions.push(Some(instruction));
            } else {
                self.instructions.push(None);
            };
        }
    }

    pub fn view(&mut self) -> Element<DisassMsg, Renderer> {
        let title = Text::new("Disassembler").medium_it(20);
        let disassembler = Column::new().push(title).align_items(Alignment::Center);
        let mut column = Column::new();
        column = column.push(self.header.view());
        for instruction in &mut self.instructions {
            match instruction {
                Some(instruction) => column = column.push(instruction.view()),
                None => {
                    let unimplemented = Text::new("Unimplemented").light(20);
                    column = column.push(unimplemented);
                }
            };
        }
        disassembler.push(column).into()
    }
}
