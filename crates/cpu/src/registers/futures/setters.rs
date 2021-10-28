use super::{AsyncGet, Get, Set};
use crate::registers::{Bits16, Bits8, Bus, IncDec};
use crate::{Access, Cpu};
use memory::Async as A;
use shared::Error;

const IO_REG: u16 = 0xFF00;

pub async fn hl_sub(cpu: Cpu) -> Result<u8, Error> {
    let data = cpu.registers().borrow_mut().get(Bits8::A);
    let cycles = Set::Bits8At(Bits16::HL, data).run(cpu.clone()).await?;
    cpu.registers().borrow_mut().decrease(Bits16::HL, 1);
    Ok(cycles)
}

pub async fn hl_add(cpu: Cpu) -> Result<u8, Error> {
    let data = cpu.registers().borrow_mut().get(Bits8::A);
    let cycles = Set::Bits8At(Bits16::HL, data).run(cpu.clone()).await?;
    cpu.registers().borrow_mut().increase(Bits16::HL, 1);
    Ok(cycles)
}

pub async fn data(cpu: Cpu, area: Bits16) -> Result<u8, Error> {
    let data: u16 = cpu.registers().borrow().get(area);
    let (dst, cycles) = Get::Next.get(cpu.clone()).await?;

    Ok(cpu.memory().set(dst, data).await? + cycles)
}

pub async fn reg_at(cpu: Cpu, dst: Bits16, src: Bits8) -> Result<u8, Error> {
    let data: u8 = cpu.registers().borrow().get(src);
    let address: u16 = cpu.registers().borrow().get(dst);
    cpu.memory().set(address, data).await
}

pub async fn u8_at(cpu: Cpu, dst: Bits16, data: u8) -> Result<u8, Error> {
    let address: u16 = cpu.registers().borrow().get(dst);
    cpu.memory().set(address, data).await
}

pub async fn u16_at(cpu: Cpu, dst: Bits16, data: u16) -> Result<u8, Error> {
    let address: u16 = cpu.registers().borrow().get(dst);
    cpu.memory().set(address, data).await
}

pub async fn hl(cpu: Cpu, area: Bits8) -> Result<u8, Error> {
    let data: u8 = cpu.registers().borrow().get(area);
    let dst: u16 = cpu.registers().borrow().get(Bits16::HL);
    cpu.memory().set(dst, data).await
}

pub async fn io_c(cpu: Cpu) -> Result<u8, Error> {
    let a = cpu.registers().borrow_mut().get(Bits8::A);
    let c: u16 = cpu.registers().borrow().get(Bits8::C).into();
    let cycles = cpu.memory().set::<u8>(c + IO_REG, a).await?;
    Ok(cycles)
}

pub async fn io_next(cpu: Cpu) -> Result<u8, Error> {
    let a = cpu.registers().borrow_mut().get(Bits8::A);
    let (data, mut cycles): (u8, u8) = Get::Next.get(cpu.clone()).await?;
    cycles += cpu.memory().set::<u8>(u16::from(data) + IO_REG, a).await?;
    Ok(cycles)
}
