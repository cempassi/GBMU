use super::{AsyncGet, Get, Set};
use crate::registers::{Absolute, Bits16, Bus, Flag, Relative};
use crate::Registers;
use memory::Memory;
use shared::Error;
use std::future::Future;
use std::pin::Pin;

type Jumper = Pin<Box<dyn Future<Output = Result<u8, Error>>>>;

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
    Return,
    ReturnCheck(Flag),
    ReturnNot(Flag),
}

impl Jump {
    pub fn jump(self, register: Registers, memory: Memory) -> Jumper {
        match self {
            Jump::Absolute => Box::pin(absolute(register, memory)),
            Jump::Relative => Box::pin(relative(register, memory)),
            Jump::AbsoluteCheck(flag) => Box::pin(abs_check(register, memory, flag)),
            Jump::AbsoluteNot(flag) => Box::pin(abs_not(register, memory, flag)),
            Jump::RelativeCheck(flag) => Box::pin(rel_check(register, memory, flag)),
            Jump::RelativeNot(flag) => Box::pin(rel_not(register, memory, flag)),
            Jump::Call => Box::pin(call(register, memory)),
            Jump::CallCheck(flag) => Box::pin(call_check(register, memory, flag)),
            Jump::CallNot(flag) => Box::pin(call_not(register, memory, flag)),
            Jump::Return => Box::pin(ret(register, memory)),
            Jump::ReturnCheck(flag) => Box::pin(ret_check(register, memory, flag)),
            Jump::ReturnNot(flag) => Box::pin(ret_not(register, memory, flag)),
        }
    }
}

async fn ret(registers: Registers, memory: Memory) -> Result<u8, Error> {
    Set::Pop(Bits16::PC).run(registers, memory).await
}

async fn ret_check(registers: Registers, memory: Memory, flag: Flag) -> Result<u8, Error> {
    if registers.borrow().get(flag) {
        Set::Pop(Bits16::PC).run(registers, memory).await
    } else {
        Ok(0)
    }
}

async fn ret_not(registers: Registers, memory: Memory, flag: Flag) -> Result<u8, Error> {
    if !registers.borrow().get(flag) {
        Set::Pop(Bits16::PC).run(registers, memory).await
    } else {
        Ok(0)
    }
}

async fn call(registers: Registers, memory: Memory) -> Result<u8, Error> {
    let (address, mut cycles): (u16, u8) = Get::Next.get(registers.clone(), memory.clone()).await?;
    cycles += Set::Push(Bits16::PC).run(registers.clone(), memory).await?;
    registers.borrow_mut().set(Bits16::PC, address);
    Ok(cycles)
}

async fn call_check(registers: Registers, memory: Memory, flag: Flag) -> Result<u8, Error> {
    let (address, mut cycles): (u16, u8) = Get::Next.get(registers.clone(), memory.clone()).await?;
    if registers.borrow().get(flag) {
        cycles += Set::Push(Bits16::PC).run(registers.clone(), memory).await?;
        registers.borrow_mut().set(Bits16::PC, address);
    }
    Ok(cycles)
}

async fn call_not(registers: Registers, memory: Memory, flag: Flag) -> Result<u8, Error> {
    let (address, mut cycles): (u16, u8) = Get::Next.get(registers.clone(), memory.clone()).await?;
    if !registers.borrow().get(flag) {
        cycles += Set::Push(Bits16::PC).run(registers.clone(), memory).await?;
        registers.borrow_mut().set(Bits16::PC, address);
    }
    Ok(cycles)
}

async fn absolute(registers: Registers, memory: Memory) -> Result<u8, Error> {
    let (address, cycles): (u16, u8) = Get::Next.get(registers.clone(), memory).await?;
    registers.borrow_mut().absolute(address);
    Ok(cycles)
}

async fn relative(registers: Registers, memory: Memory) -> Result<u8, Error> {
    let (offset, cycles): (u8, u8) = Get::Next.get(registers.clone(), memory).await?;
    registers.borrow_mut().relative(offset as i8);
    Ok(cycles)
}

async fn abs_check(registers: Registers, memory: Memory, flag: Flag) -> Result<u8, Error> {
    let (address, cycles): (u16, u8) = Get::Next.get(registers.clone(), memory).await?;
    registers.borrow_mut().absolute_check(address, flag);
    Ok(cycles)
}

async fn abs_not(registers: Registers, memory: Memory, flag: Flag) -> Result<u8, Error> {
    let (address, cycles): (u16, u8) = Get::Next.get(registers.clone(), memory).await?;
    registers.borrow_mut().absolute_check(address, flag);
    Ok(cycles)
}

async fn rel_check(registers: Registers, memory: Memory, flag: Flag) -> Result<u8, Error> {
    let (offset, cycles): (u8, u8) = Get::Next.get(registers.clone(), memory).await?;
    registers.borrow_mut().relative_check(offset as i8, flag);
    Ok(cycles)
}

async fn rel_not(registers: Registers, memory: Memory, flag: Flag) -> Result<u8, Error> {
    let (offset, cycles): (u8, u8) = Get::Next.get(registers.clone(), memory).await?;
    registers.borrow_mut().relative_check(offset as i8, flag);
    Ok(cycles)
}
