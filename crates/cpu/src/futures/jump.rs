use super::{AsyncGet, Get, Set};
use crate::registers::{Absolute, Bits16, Bus, Flag, Relative};
use crate::{Access, Cpu};
use memory::Async;
use shared::Error;
use std::future::Future;
use std::pin::Pin;

type Jumper = Pin<Box<dyn Future<Output = Result<u8, Error>>>>;

pub enum Reset {
    H00,
    H10,
    H20,
    H30,
    H08,
    H18,
    H28,
    H38,
}

impl Reset {
    pub fn dst(self) -> u16 {
        match self {
            Reset::H00 => 0x00,
            Reset::H10 => 0x10,
            Reset::H20 => 0x20,
            Reset::H30 => 0x30,
            Reset::H08 => 0x08,
            Reset::H18 => 0x18,
            Reset::H28 => 0x28,
            Reset::H38 => 0x38,
        }
    }
}

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
    Reset(Reset),
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
            Jump::Reset(reset) => Box::pin(res(cpu, reset)),
        }
    }
}

async fn res(cpu: Cpu, reset: Reset) -> Result<u8, Error> {
    let (_, mut cycles): (u8, u8) = Get::Next.get(cpu.clone()).await?;
    cpu.borrow_mut().registers.set(Bits16::PC, reset.dst());
    cycles += Set::Push(Bits16::PC).run(cpu).await?;
    Ok(cycles)
}

async fn ret(cpu: Cpu) -> Result<u8, Error> {
    let (_, delay) = cpu.memory().get::<u8>(0xc00).await?;
    Ok(Set::Pop(Bits16::PC).run(cpu).await? + delay)
}

async fn ret_interrupt(cpu: Cpu) -> Result<u8, Error> {
    cpu.memory().borrow_mut().enable_interrupts();
    ret(cpu).await
}

async fn ret_check(cpu: Cpu, flag: Flag) -> Result<u8, Error> {
    let flag = cpu.borrow().registers.get(flag);
    if flag {
        ret(cpu).await
    } else {
        Ok(0)
    }
}

async fn ret_not(cpu: Cpu, flag: Flag) -> Result<u8, Error> {
    let flag = cpu.borrow().registers.get(flag);
    if !flag {
        ret(cpu).await
    } else {
        Ok(0)
    }
}

async fn call(cpu: Cpu) -> Result<u8, Error> {
    let (address, mut cycles): (u16, u8) = Get::Next.get(cpu.clone()).await?;
    cycles += Set::Push(Bits16::PC).run(cpu.clone()).await?;
    cpu.borrow_mut().registers.set(Bits16::PC, address);
    Ok(cycles)
}

async fn call_check(cpu: Cpu, flag: Flag) -> Result<u8, Error> {
    let (address, mut cycles): (u16, u8) = Get::Next.get(cpu.clone()).await?;
    let flag = cpu.borrow().registers.get(flag);
    if flag {
        cycles += Set::Push(Bits16::PC).run(cpu.clone()).await?;
        cpu.borrow_mut().registers.set(Bits16::PC, address)
    }
    Ok(cycles)
}

async fn call_not(cpu: Cpu, flag: Flag) -> Result<u8, Error> {
    let (address, mut cycles): (u16, u8) = Get::Next.get(cpu.clone()).await?;
    let flag = cpu.borrow().registers.get(flag);
    if !flag {
        cycles += Set::Push(Bits16::PC).run(cpu.clone()).await?;
        cpu.borrow_mut().registers.set(Bits16::PC, address)
    }
    Ok(cycles)
}

async fn absolute(cpu: Cpu) -> Result<u8, Error> {
    let (address, cycles): (u16, u8) = Get::Next.get(cpu.clone()).await?;
    let (_, delay) = cpu.memory().get::<u8>(0xc00).await?;
    cpu.borrow_mut().registers.absolute(address);
    Ok(cycles + delay)
}

async fn abs_check(cpu: Cpu, flag: Flag) -> Result<u8, Error> {
    let (address, cycles): (u16, u8) = Get::Next.get(cpu.clone()).await?;
    let delay = {
        if cpu.borrow_mut().registers.absolute_check(address, flag) {
            let (_, delay) = cpu.memory().get::<u8>(0xc00).await?;
            delay
        } else {
            0
        }
    };
    Ok(cycles + delay)
}

async fn abs_not(cpu: Cpu, flag: Flag) -> Result<u8, Error> {
    let (address, cycles): (u16, u8) = Get::Next.get(cpu.clone()).await?;
    let delay = {
        if cpu.borrow_mut().registers.absolute_not(address, flag) {
            let (_, delay) = cpu.memory().get::<u8>(0xc00).await?;
            delay
        } else {
            0
        }
    };
    Ok(cycles + delay)
}

// Vigilence sur la conversion en i8
async fn relative(cpu: Cpu) -> Result<u8, Error> {
    let (_, delay) = cpu.memory().get::<u8>(0xc00).await?;
    let (offset, cycles): (u8, u8) = Get::Next.get(cpu.clone()).await?;
    cpu.borrow_mut().registers.relative(offset as i8);
    Ok(cycles + delay)
}

async fn rel_check(cpu: Cpu, flag: Flag) -> Result<u8, Error> {
    let (data, cycles): (u8, u8) = Get::Next.get(cpu.clone()).await?;

    let delay = {
        if cpu.borrow_mut().registers.relative_check(data as i8, flag) {
            let (_, delay) = cpu.memory().get::<u8>(0xc00).await?;
            delay
        } else {
            0
        }
    };
    Ok(cycles + delay)
}

async fn rel_not(cpu: Cpu, flag: Flag) -> Result<u8, Error> {
    let (offset, cycles): (u8, u8) = Get::Next.get(cpu.clone()).await?;
    let delay = {
        if cpu.borrow_mut().registers.relative_not(offset as i8, flag) {
            let (_, delay) = cpu.memory().get::<u8>(0xc00).await?;
            delay
        } else {
            0
        }
    };
    Ok(cycles + delay)
}
