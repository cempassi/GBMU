use crate::debugger::widgets::Text;
use crate::error::Error;
use iced::{Alignment, Column, Element, Length};

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
    Breakpoint(u16),
    SetBreakpoint(usize, u16),
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
        let _ = disassembler.update(DisassMsg::Refresh);
        disassembler
    }

    fn check_pc(&mut self, message: DisassMsg) -> Result<u16, Error> {
        let pc = self.cpu.borrow().registers.pc;
        if self.is_jump {
            self.is_jump = false;
            Ok(pc)
        } else if message == DisassMsg::Refresh {
            Ok(pc)
        } else if pc != self.next {
            Err(Error::NoUpdate)
        } else {
            Ok(self.next)
        }
    }

    pub fn update(&mut self, message: DisassMsg) -> Result<(), Error> {
        match message {
            DisassMsg::Refresh => {
                //println!("Refreshed!");
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
            DisassMsg::Breakpoint(_) => Ok(()),
            DisassMsg::SetBreakpoint(index, _) => match self.instructions.get_mut(index as usize) {
                Some(instruction) => {
                    if let Some(instruction) = instruction {
                        instruction.bp_toogle();
                    };
                    Ok(())
                }
                _ => Ok(()),
            },
        }
    }

    pub fn view(&mut self) -> Element<DisassMsg> {
        let title = Text::new("Disassembler").medium_it(20);
        let disassembler = Column::new()
            .push(title)
            .align_items(Alignment::Center)
            .width(Length::Shrink);
        let mut column = Column::new();
        column = column.push(self.header.view());
        let column = self.instructions.iter_mut().enumerate().fold(
            column,
            |column, (index, instruction)| match instruction {
                Some(instruction) => {
                    let element: Element<DisassMsg> = instruction.view();
                    column.push(element.map(move |message| {
                        if let DisassMsg::Breakpoint(address) = message {
                            DisassMsg::SetBreakpoint(index, address)
                        } else {
                            message
                        }
                    }))
                }
                None => {
                    let unimplemented = Text::new("Unimplemented").light(20);
                    column.push(unimplemented)
                }
            },
        );
        disassembler.push(column).into()
    }
}
