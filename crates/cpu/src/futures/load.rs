use super::{AsyncGet, Get, Set};
use crate::registers::{Bits16, Bits8, Bus, IncDec};
use crate::{Access, Cpu};
use memory::Async;
use shared::Error;

const IO_REG: u16 = 0xFF00;

pub async fn u8(cpu: Cpu, area: Bits8) -> Result<u8, Error> {
    let (data, cycles) = Get::Next.get(cpu.clone()).await?;
    cpu.borrow_mut().registers.set(area, data);
    Ok(cycles)
}

pub async fn at_next_a(cpu: Cpu) -> Result<u8, Error> {
    let (address, mut cycles): (u16, u8) = Get::Next.get(cpu.clone()).await?;
    let a = cpu.borrow_mut().registers.get(Bits8::A);
    cycles += cpu.memory().set(address, a).await?;
    Ok(cycles)
}

pub async fn a_at_next(cpu: Cpu) -> Result<u8, Error> {
    let (address, cycles): (u16, u8) = Get::Next.get(cpu.clone()).await?;
    let (data, fetch): (u8, u8) = cpu.memory().get::<u8>(address).await?;
    cpu.borrow_mut().registers.set(Bits8::A, data);
    Ok(cycles + fetch)
}

pub async fn u16(cpu: Cpu, area: Bits16) -> Result<u8, Error> {
    let (data, cycles) = Get::Next.get(cpu.clone()).await?;
    cpu.borrow_mut().registers.set(area, data);
    Ok(cycles)
}
pub async fn hl8b(cpu: Cpu) -> Result<u8, Error> {
    let (data, cycles): (u8, u8) = Get::Next.get(cpu.clone()).await?;
    Ok(Set::Bits8At(Bits16::HL, data).run(cpu).await? + cycles)
}

pub async fn hl(cpu: Cpu, area: Bits8) -> Result<u8, Error> {
    let (data, cycles) = Get::BitsAt(Bits16::HL).get(cpu.clone()).await?;
    cpu.borrow_mut().registers.set(area, data);
    Ok(cycles)
}

pub async fn hl_sub(cpu: Cpu) -> Result<u8, Error> {
    let (data, cycles) = Get::BitsAt(Bits16::HL).get(cpu.clone()).await?;
    cpu.borrow_mut().registers.set(Bits8::A, data);
    cpu.borrow_mut().registers.decrease(Bits16::HL, 1);
    Ok(cycles)
}

pub async fn hl_add(cpu: Cpu) -> Result<u8, Error> {
    let (data, cycles) = Get::BitsAt(Bits16::HL).get(cpu.clone()).await?;
    cpu.borrow_mut().registers.set(Bits8::A, data);
    cpu.borrow_mut().registers.increase(Bits16::HL, 1);
    Ok(cycles)
}

pub async fn reg_from(cpu: Cpu, dst: Bits8, src: Bits16) -> Result<u8, Error> {
    let (data, cycles): (u8, u8) = Get::BitsAt(src).get(cpu.clone()).await?;
    cpu.borrow_mut().registers.set(dst, data);
    Ok(cycles)
}

/// SP instructions take an extra 4 clocks to execute
pub async fn push(cpu: Cpu, area: Bits16) -> Result<u8, Error> {
    let data = cpu.borrow().registers.get(area);
    let (_, delay) = cpu.memory().get::<u8>(0xc00).await?;
    cpu.borrow_mut().registers.decrease(Bits16::SP, 2);
    let cycles = Set::Bits16At(Bits16::SP, data).run(cpu).await?;
    Ok(cycles + delay)
}

pub async fn pop(cpu: Cpu, area: Bits16) -> Result<u8, Error> {
    //let (_, delay) = cpu.memory().clone().get::<u8>(0xc00).await?;
    let (data, cycles): (u16, u8) = Get::BitsAt(Bits16::SP).get(cpu.clone()).await?;
    cpu.borrow_mut().registers.set(area, data);
    cpu.borrow_mut().registers.increase(Bits16::SP, 2);
    Ok(cycles)
}

pub async fn io_c(cpu: Cpu) -> Result<u8, Error> {
    let c: u16 = cpu.borrow().registers.get(Bits8::C).into();
    let (data, cycles) = cpu.memory().get::<u8>(c + IO_REG).await?;
    cpu.borrow_mut().registers.set(Bits8::A, data);
    Ok(cycles)
}

pub async fn io_next(cpu: Cpu) -> Result<u8, Error> {
    let (src, next): (u8, u8) = Get::Next.get(cpu.clone()).await?;
    let (data, get) = cpu.memory().get::<u8>(u16::from(src) + IO_REG).await?;
    cpu.borrow_mut().registers.set(Bits8::A, data);
    Ok(next + get)
}

pub async fn hl_sp(cpu: Cpu) -> Result<u8, Error> {
    let (mut data, cycles) = Get::Next.get(cpu.clone()).await?;
    data += cpu.borrow().registers.get(Bits16::SP);
    cpu.borrow_mut().registers.set(Bits16::HL, data);
    Ok(cycles)
}

pub async fn sp_hl(cpu: Cpu) -> Result<u8, Error> {
    let data = cpu.borrow().registers.get(Bits16::HL);
    let (_, cycles) = cpu.memory().get::<u8>(0xc000).await?;
    cpu.borrow_mut().registers.set(Bits16::SP, data);
    Ok(cycles)
}
