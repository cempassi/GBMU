use crate::style::fonts;
use iced_wgpu::{Column, Renderer, Text};
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
}

#[derive(Debug, Clone)]
pub enum DisassMsg {
    Step,
}

impl Disassembler {
    pub fn new(registers: Registers, memory: Memory) -> Self {
        let instructions = Vec::new();
        let header = Header::new();
        let mut disassembler = Self {
            header,
            registers,
            memory,
            instructions,
        };
        disassembler.update(DisassMsg::Step);
        disassembler
    }

    fn update_instructions(&mut self) {
        let mut pc = self.registers.borrow().pc;
        for _ in 0..5 {
            if let Ok(instruction) = Instruction::try_new(pc, &self.memory, false) {
                pc += instruction.fetched();
                self.instructions.push(Some(instruction));
            } else {
                self.instructions.push(None);
            };
        }
    }

    pub fn update(&mut self, _message: DisassMsg) {
        self.update_instructions();
    }

    pub fn view(&mut self) -> Element<DisassMsg, Renderer> {
        let mut column = Column::new();
        column = column.push(self.header.view());
        for instruction in &mut self.instructions {
            match instruction {
                Some(instruction) => column = column.push(instruction.view()),
                None => {
                    let unimplemented = Text::new("Unimplemented")
                        .font(fonts::HASKLIG_LIGHT)
                        .size(20);
                    column = column.push(unimplemented);
                }
            };
        }
        column.into()
    }
}
