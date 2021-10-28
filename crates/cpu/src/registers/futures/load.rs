use super::{AsyncGet, Get, Set};
use crate::registers::{Bits16, Bits8, Bus, IncDec};
use crate::{Access, Cpu};
use memory::Async;
use shared::Error;

const IO_REG: u16 = 0xFF00;

pub async fn u8(cpu: Cpu, area: Bits8) -> Result<u8, Error> {
    let (data, cycles) = Get::Next.get(cpu.clone()).await?;
    cpu.registers().borrow_mut().set(area, data);
    Ok(cycles)
}
pub async fn u16(cpu: Cpu, area: Bits16) -> Result<u8, Error> {
    let (data, cycles) = Get::Next.get(cpu.clone()).await?;
    cpu.registers().borrow_mut().set(area, data);
    Ok(cycles)
}
pub async fn hl8b(cpu: Cpu) -> Result<u8, Error> {
    let (data, cycles): (u8, u8) = Get::Next.get(cpu.clone()).await?;
    Ok(Set::Bits8At(Bits16::HL, data).run(cpu).await? + cycles)
}

pub async fn hl(cpu: Cpu, area: Bits8) -> Result<u8, Error> {
    let (data, cycles) = Get::BitsAt(Bits16::HL).get(cpu.clone()).await?;
    cpu.registers().borrow_mut().set(area, data);
    Ok(cycles)
}

pub async fn hl_sub(cpu: Cpu) -> Result<u8, Error> {
    let (data, cycles) = Get::BitsAt(Bits16::HL).get(cpu.clone()).await?;
    cpu.registers().borrow_mut().set(Bits8::A, data);
    cpu.registers().borrow_mut().decrease(Bits16::HL, 1);
    Ok(cycles)
}

pub async fn hl_add(cpu: Cpu) -> Result<u8, Error> {
    let (data, cycles) = Get::BitsAt(Bits16::HL).get(cpu.clone()).await?;
    cpu.registers().borrow_mut().set(Bits8::A, data);
    cpu.registers().borrow_mut().increase(Bits16::HL, 1);
    Ok(cycles)
}

pub async fn reg_from(cpu: Cpu, dst: Bits8, src: Bits16) -> Result<u8, Error> {
    let (data, cycles): (u8, u8) = Get::BitsAt(src).get(cpu.clone()).await?;
    cpu.registers().borrow_mut().set(dst, data);
    Ok(cycles)
}

/// SP instructions take an extra 4 clocks to execute
pub async fn push(cpu: Cpu, area: Bits16) -> Result<u8, Error> {
    let data = cpu.registers().borrow().get(area);
    let (_, delay) = cpu.memory().get::<u8>(0xc00).await?;
    cpu.registers().borrow_mut().decrease(Bits16::SP, 2);
    let cycles = Set::Bits16At(Bits16::SP, data).run(cpu).await?;
    Ok(cycles + delay)
}

/// SP instructions take an extra 4 clocks to execute
pub async fn pop(cpu: Cpu, area: Bits16) -> Result<u8, Error> {
    let (_, delay) = cpu.memory().clone().get::<u8>(0xc00).await?;
    let (data, cycles): (u16, u8) = Get::BitsAt(Bits16::SP).get(cpu.clone()).await?;
    cpu.registers().borrow_mut().set(area, data);
    cpu.registers().borrow_mut().increase(Bits16::SP, 2);
    Ok(cycles + delay)
}

pub async fn io_c(cpu: Cpu) -> Result<u8, Error> {
    let c: u16 = cpu.registers().borrow().get(Bits8::C).into();
    let (data, cycles) = cpu.memory().get::<u8>(c + IO_REG).await?;
    cpu.registers().borrow_mut().set(Bits8::A, data);
    Ok(cycles)
}

pub async fn io_next(cpu: Cpu) -> Result<u8, Error> {
    let (src, next): (u8, u8) = Get::Next.get(cpu.clone()).await?;
    let (data, get) = cpu.memory().get::<u8>(u16::from(src) + IO_REG).await?;
    cpu.registers().borrow_mut().set(Bits8::A, data);
    Ok(next + get)
}

pub async fn hl_sp(cpu: Cpu) -> Result<u8, Error> {
    let (mut data, cycles) = Get::Next.get(cpu.clone()).await?;
    data += cpu.registers().borrow().get(Bits16::SP);
    cpu.registers().borrow_mut().set(Bits16::HL, data);
    Ok(cycles)
}

pub async fn sp_hl(cpu: Cpu) -> Result<u8, Error> {
    let data = cpu.registers().borrow().get(Bits16::HL);
    let (_, cycles) = cpu.memory().get::<u8>(0xc000).await?;
    cpu.registers().borrow_mut().set(Bits16::SP, data);
    Ok(cycles)
}
