use super::{reader::Reader, Async, NextPc};
use crate::registers::{Absolute, Bits16, Bus, Flag, Relative};
use crate::Registers;
use memory::Memory;
use shared::Error;
use std::future::Future;
use std::pin::Pin;

type Jumper = Pin<Box<dyn Future<Output = Result<(), Error>>>>;

pub enum Jump {
    Absolute,
    AbsoluteCheck(Flag),
    AbsoluteNot(Flag),
    Relative,
    RelativeCheck(Flag),
    RelativeNot(Flag),
    Call,
    CallCheck(Flag),
    CallNot(Flag),
}

impl Jump {
    pub fn jump(self, register: Registers, memory: Memory) -> Jumper {
        match self {
            Jump::Absolute => Box::pin(Reader::new(Box::pin(absolute(register, memory)))),
            Jump::Relative => Box::pin(Reader::new(Box::pin(relative(register, memory)))),
            Jump::AbsoluteCheck(flag) => {
                Box::pin(Reader::new(Box::pin(abs_check(register, memory, flag))))
            }
            Jump::AbsoluteNot(flag) => {
                Box::pin(Reader::new(Box::pin(abs_not(register, memory, flag))))
            }
            Jump::RelativeCheck(flag) => {
                Box::pin(Reader::new(Box::pin(rel_check(register, memory, flag))))
            }
            Jump::RelativeNot(flag) => {
                Box::pin(Reader::new(Box::pin(rel_not(register, memory, flag))))
            }
            Jump::Call => Box::pin(Reader::new(Box::pin(call(register, memory)))),
            Jump::CallCheck(flag) => {
                Box::pin(Reader::new(Box::pin(call_check(register, memory, flag))))
            }
            Jump::CallNot(flag) => {
                Box::pin(Reader::new(Box::pin(call_not(register, memory, flag))))
            }
        }
    }
}

async fn call(registers: Registers, memory: Memory) -> Result<(), Error> {
    let address: u16 = registers.clone().next_pc(memory.clone()).await?;
    Async::Push(Bits16::PC)
        .run(registers.clone(), memory)
        .await?;
    registers.borrow_mut().set(Bits16::PC, address);
    Ok(())
}

async fn call_check(registers: Registers, memory: Memory, flag: Flag) -> Result<(), Error> {
    let address: u16 = registers.clone().next_pc(memory.clone()).await?;
    if registers.borrow().get(flag) {
        Async::Push(Bits16::PC)
            .run(registers.clone(), memory)
            .await?;
        registers.borrow_mut().set(Bits16::PC, address);
    }
    Ok(())
}

async fn call_not(registers: Registers, memory: Memory, flag: Flag) -> Result<(), Error> {
    let address: u16 = registers.clone().next_pc(memory.clone()).await?;
    if !registers.borrow().get(flag) {
        Async::Push(Bits16::PC)
            .run(registers.clone(), memory)
            .await?;
        registers.borrow_mut().set(Bits16::PC, address);
    }
    Ok(())
}

async fn absolute(registers: Registers, memory: Memory) -> Result<(), Error> {
    let address: u16 = registers.clone().next_pc(memory).await?;
    registers.borrow_mut().absolute(address);
    Ok(())
}

async fn relative(registers: Registers, memory: Memory) -> Result<(), Error> {
    let offset: u8 = registers.clone().next_pc(memory).await?;
    registers.borrow_mut().relative(offset as i8);
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
    registers.borrow_mut().relative_check(offset as i8, flag);
    Ok(())
}

async fn rel_not(registers: Registers, memory: Memory, flag: Flag) -> Result<(), Error> {
    let offset: u8 = registers.clone().next_pc(memory).await?;
    registers.borrow_mut().relative_check(offset as i8, flag);
    Ok(())
}
