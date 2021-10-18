use super::{NextPc, Reader};
use crate::registers::{Absolute, Flag, Relative};
use crate::Registers;
use memory::Memory;
use shared::Error;
use std::future::Future;
use std::pin::Pin;

type Jumper = Pin<Box<dyn Future<Output = Result<(), Error>>>>;

pub enum Jump {
    Absolute,
    AbsCheck(Flag),
    AbsNot(Flag),
    Relative,
    RelCheck(Flag),
    RelNot(Flag),
}

impl Jump {
    pub fn jump(self, memory: Memory, register: Registers) -> Jumper {
        match self {
            Jump::Absolute => Box::pin(Reader::new(Box::pin(absolute(register, memory)))),
            Jump::Relative => Box::pin(Reader::new(Box::pin(relative(register, memory)))),
            Jump::AbsCheck(flag) => {
                Box::pin(Reader::new(Box::pin(abs_check(register, memory, flag))))
            }
            Jump::AbsNot(flag) => Box::pin(Reader::new(Box::pin(abs_not(register, memory, flag)))),
            Jump::RelCheck(flag) => {
                Box::pin(Reader::new(Box::pin(rel_check(register, memory, flag))))
            }
            Jump::RelNot(flag) => Box::pin(Reader::new(Box::pin(rel_not(register, memory, flag)))),
        }
    }
}

async fn absolute(registers: Registers, memory: Memory) -> Result<(), Error> {
    let address: u16 = registers.clone().next_pc(memory).await?;
    registers.borrow_mut().absolute(address);
    Ok(())
}

async fn relative(registers: Registers, memory: Memory) -> Result<(), Error> {
    let offset: u8 = registers.clone().next_pc(memory).await?;
    registers.borrow_mut().jump_relative(offset as i8);
    Ok(())
}

async fn abs_check(registers: Registers, memory: Memory, flag: Flag) -> Result<(), Error> {
    let address: u16 = registers.clone().next_pc(memory).await?;
    registers.borrow_mut().absolute_check(address, flag);
    Ok(())
}

async fn abs_not(registers: Registers, memory: Memory, flag: Flag) -> Result<(), Error> {
    let address: u16 = registers.clone().next_pc(memory).await?;
    registers.borrow_mut().absolute_check(address, flag);
    Ok(())
}

async fn rel_check(registers: Registers, memory: Memory, flag: Flag) -> Result<(), Error> {
    let offset: u8 = registers.clone().next_pc(memory).await?;
    registers
        .borrow_mut()
        .jump_relative_check(offset as i8, flag);
    Ok(())
}

async fn rel_not(registers: Registers, memory: Memory, flag: Flag) -> Result<(), Error> {
    let offset: u8 = registers.clone().next_pc(memory).await?;
    registers
        .borrow_mut()
        .jump_relative_check(offset as i8, flag);
    Ok(())
}
