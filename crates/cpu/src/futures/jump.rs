use super::{AsyncGet, Get, Set};
use crate::registers::{Absolute, Bits16, Bus, Flag, Relative};
use crate::{Access, Cpu};
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
    ReturnInterrupt,
}

impl Jump {
    pub fn jump(self, cpu: Cpu) -> Jumper {
        match self {
            Jump::Absolute => Box::pin(absolute(cpu)),
            Jump::Relative => Box::pin(relative(cpu)),
            Jump::AbsoluteCheck(flag) => Box::pin(abs_check(cpu, flag)),
            Jump::AbsoluteNot(flag) => Box::pin(abs_not(cpu, flag)),
            Jump::RelativeCheck(flag) => Box::pin(rel_check(cpu, flag)),
            Jump::RelativeNot(flag) => Box::pin(rel_not(cpu, flag)),
            Jump::Call => Box::pin(call(cpu)),
            Jump::CallCheck(flag) => Box::pin(call_check(cpu, flag)),
            Jump::CallNot(flag) => Box::pin(call_not(cpu, flag)),
            Jump::Return => Box::pin(ret(cpu)),
            Jump::ReturnCheck(flag) => Box::pin(ret_check(cpu, flag)),
            Jump::ReturnNot(flag) => Box::pin(ret_not(cpu, flag)),
            Jump::ReturnInterrupt => Box::pin(ret_interrupt(cpu)),
        }
    }
}

async fn ret(cpu: Cpu) -> Result<u8, Error> {
    Set::Pop(Bits16::PC).run(cpu).await
}

async fn ret_interrupt(cpu: Cpu) -> Result<u8, Error> {
    cpu.memory().borrow_mut().enable_interrupts();
    Set::Pop(Bits16::PC).run(cpu).await
}

async fn ret_check(cpu: Cpu, flag: Flag) -> Result<u8, Error> {
    if cpu.registers().borrow().get(flag) {
        Set::Pop(Bits16::PC).run(cpu).await
    } else {
        Ok(0)
    }
}

async fn ret_not(cpu: Cpu, flag: Flag) -> Result<u8, Error> {
    if !cpu.registers().borrow().get(flag) {
        Set::Pop(Bits16::PC).run(cpu).await
    } else {
        Ok(0)
    }
}

async fn call(cpu: Cpu) -> Result<u8, Error> {
    let (address, mut cycles): (u16, u8) = Get::Next.get(cpu.clone()).await?;
    cycles += Set::Push(Bits16::PC).run(cpu.clone()).await?;
    cpu.registers().borrow_mut().set(Bits16::PC, address);
    Ok(cycles)
}

async fn call_check(cpu: Cpu, flag: Flag) -> Result<u8, Error> {
    let (address, mut cycles): (u16, u8) = Get::Next.get(cpu.clone()).await?;
    if cpu.registers().borrow().get(flag) {
        cycles += Set::Push(Bits16::PC).run(cpu.clone()).await?;
        cpu.registers().borrow_mut().set(Bits16::PC, address);
    }
    Ok(cycles)
}

async fn call_not(cpu: Cpu, flag: Flag) -> Result<u8, Error> {
    let (address, mut cycles): (u16, u8) = Get::Next.get(cpu.clone()).await?;
    if !cpu.registers().borrow().get(flag) {
        cycles += Set::Push(Bits16::PC).run(cpu.clone()).await?;
        cpu.registers().borrow_mut().set(Bits16::PC, address);
    }
    Ok(cycles)
}

async fn absolute(cpu: Cpu) -> Result<u8, Error> {
    let (address, cycles): (u16, u8) = Get::Next.get(cpu.clone()).await?;
    cpu.registers().borrow_mut().absolute(address);
    Ok(cycles)
}

async fn relative(cpu: Cpu) -> Result<u8, Error> {
    let (offset, cycles): (u8, u8) = Get::Next.get(cpu.clone()).await?;
    cpu.registers().borrow_mut().relative(offset as i8);
    Ok(cycles)
}

async fn abs_check(cpu: Cpu, flag: Flag) -> Result<u8, Error> {
    let (address, cycles): (u16, u8) = Get::Next.get(cpu.clone()).await?;
    cpu.registers().borrow_mut().absolute_check(address, flag);
    Ok(cycles)
}

async fn abs_not(cpu: Cpu, flag: Flag) -> Result<u8, Error> {
    let (address, cycles): (u16, u8) = Get::Next.get(cpu.clone()).await?;
    cpu.registers().borrow_mut().absolute_check(address, flag);
    Ok(cycles)
}

async fn rel_check(cpu: Cpu, flag: Flag) -> Result<u8, Error> {
    let (offset, cycles): (u8, u8) = Get::Next.get(cpu.clone()).await?;
    cpu.registers()
        .borrow_mut()
        .relative_check(offset as i8, flag);
    Ok(cycles)
}

async fn rel_not(cpu: Cpu, flag: Flag) -> Result<u8, Error> {
    let (offset, cycles): (u8, u8) = Get::Next.get(cpu.clone()).await?;
    cpu.registers()
        .borrow_mut()
        .relative_check(offset as i8, flag);
    Ok(cycles)
}
