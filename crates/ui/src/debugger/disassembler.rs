use crate::style::fonts;
use iced_wgpu::{Column, Renderer, Text};
use iced_winit::Element;

use cpu::Registers;
use memory::Memory;
mod instruction;
mod load;

use instruction::Instruction;
use shared::Error;

pub struct Disassembler {
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
        let mut disassembler = Self {
            registers,
            memory,
            instructions,
        };
        disassembler.update(DisassMsg::Step);
        disassembler
    }

    pub fn generate_instruction(&mut self) -> Result<Instruction, Error> {
        let address = self.registers.borrow().pc;
        Instruction::try_new(address, &self.memory, false)
    }

    pub fn update(&mut self, _message: DisassMsg) {
        match self.generate_instruction() {
            Ok(instruction) => self.instructions.push(Some(instruction)),
            Err(_) => self.instructions.push(None),
        }
    }

    pub fn view(&mut self) -> Element<DisassMsg, Renderer> {
        let mut column = Column::new();
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
