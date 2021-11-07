use crate::debugger::widgets::Text;
use crate::error::Error;
use iced_graphics::Alignment;
use iced_wgpu::{Column, Renderer};
use iced_winit::Element;

mod conversion;
mod disass;
mod header;
mod instruction;

use header::Header;
use instruction::Instruction;

pub struct Disassembler {
    header: Header,
    cpu: cpu::Cpu,
    instructions: Vec<Option<Instruction>>,
    next: u16,
    is_jump: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisassMsg {
    Refresh,
    Reload,
}

impl Disassembler {
    pub fn new(cpu: cpu::Cpu) -> Self {
        let instructions = Vec::new();
        let header = Header::new();
        let next = 0;
        let is_jump = false;

        let mut disassembler = Self {
            cpu,
            header,
            instructions,
            next,
            is_jump,
        };
        let _ = disassembler.update(DisassMsg::Reload);
        disassembler
    }

    fn check_pc(&mut self, message: DisassMsg) -> Result<u16, Error> {
        let pc = self.cpu.borrow().registers.pc;
        if self.is_jump {
            self.is_jump = false;
            Ok(pc)
        } else if message == DisassMsg::Reload {
            Ok(pc)
        } else if pc != self.next {
            Err(Error::NoUpdate)
        } else {
            Ok(self.next)
        }
    }

    pub fn update(&mut self, message: DisassMsg) -> Result<(), Error> {
        let mut pc = self.check_pc(message)?;
        self.instructions.clear();
        for id in 0..5 {
            if let Ok(instruction) =
                Instruction::try_new(pc, &self.cpu.borrow().get_memory(), false)
            {
                pc += instruction.fetched();
                if id == 0 {
                    //instruction.get_cycle();
                    if instruction.is_jump {
                        self.is_jump = true
                    };
                    self.next = pc;
                }
                self.instructions.push(Some(instruction));
            } else {
                self.instructions.push(None);
            };
        }
        Ok(())
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
