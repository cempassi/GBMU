use super::{AsyncGet, Get, Set};
use crate::registers::{Bits16, Bits8, Bus, IncDec};
use crate::Registers;
use memory::{Async, Memory};
use shared::Error;

const IO_REG: u16 = 0xFF00;

pub async fn u8(registers: Registers, memory: Memory, area: Bits8) -> Result<u8, Error> {
    let (data, cycles) = Get::Next.get(registers.clone(), memory).await?;
    registers.borrow_mut().set(area, data);
    Ok(cycles)
}
pub async fn u16(registers: Registers, memory: Memory, area: Bits16) -> Result<u8, Error> {
    let (data, cycles) = Get::Next.get(registers.clone(), memory).await?;
    registers.borrow_mut().set(area, data);
    Ok(cycles)
}
pub async fn hl8b(registers: Registers, memory: Memory) -> Result<u8, Error> {
    let (data, cycles): (u8, u8) = Get::Next.get(registers.clone(), memory.clone()).await?;
    Ok(Set::Bits8At(Bits16::HL, data)
        .run(registers, memory)
        .await?
        + cycles)
}

pub async fn hl(registers: Registers, memory: Memory, area: Bits8) -> Result<u8, Error> {
    let (data, cycles) = Get::BitsAt(Bits16::HL)
        .get(registers.clone(), memory)
        .await?;
    registers.borrow_mut().set(area, data);
    Ok(cycles)
}

pub async fn hl_sub(registers: Registers, memory: Memory) -> Result<u8, Error> {
    let (data, cycles) = Get::BitsAt(Bits16::HL)
        .get(registers.clone(), memory)
        .await?;
    registers.borrow_mut().set(Bits8::A, data);
    registers.borrow_mut().decrease(Bits16::HL, 1);
    Ok(cycles)
}

pub async fn hl_add(registers: Registers, memory: Memory) -> Result<u8, Error> {
    let (data, cycles) = Get::BitsAt(Bits16::HL)
        .get(registers.clone(), memory)
        .await?;
    registers.borrow_mut().set(Bits8::A, data);
    registers.borrow_mut().increase(Bits16::HL, 1);
    Ok(cycles)
}

pub async fn reg_from(
    registers: Registers,
    memory: Memory,
    dst: Bits8,
    src: Bits16,
) -> Result<u8, Error> {
    let (data, cycles): (u8, u8) = Get::BitsAt(src).get(registers.clone(), memory).await?;
    registers.borrow_mut().set(dst, data);
    Ok(cycles)
}

/// SP instructions take an extra 4 clocks to execute
pub async fn push(registers: Registers, memory: Memory, area: Bits16) -> Result<u8, Error> {
    let data = registers.borrow().get(area);
    let (_, delay) = memory.clone().get::<u8>(0xc00).await?;
    registers.borrow_mut().decrease(Bits16::SP, 2);
    let cycles = Set::Bits16At(Bits16::SP, data)
        .run(registers.clone(), memory)
        .await?;
    Ok(cycles + delay)
}

/// SP instructions take an extra 4 clocks to execute
pub async fn pop(registers: Registers, memory: Memory, area: Bits16) -> Result<u8, Error> {
    let (_, delay) = memory.clone().get::<u8>(0xc00).await?;
    let (data, cycles): (u16, u8) = Get::BitsAt(Bits16::SP)
        .get(registers.clone(), memory)
        .await?;
    registers.borrow_mut().set(area, data);
    registers.borrow_mut().increase(Bits16::SP, 2);
    Ok(cycles + delay)
}

pub async fn io_c(registers: Registers, memory: Memory) -> Result<u8, Error> {
    let c: u16 = registers.borrow().get(Bits8::C).into();
    let (data, cycles) = memory.get::<u8>(c + IO_REG).await?;
    registers.borrow_mut().set(Bits8::A, data);
    Ok(cycles)
}

pub async fn io_next(registers: Registers, memory: Memory) -> Result<u8, Error> {
    let (src, next): (u8, u8) = Get::Next.get(registers.clone(), memory.clone()).await?;
    let (data, get) = memory.get::<u8>(u16::from(src) + IO_REG).await?;
    registers.borrow_mut().set(Bits8::A, data);
    Ok(next + get)
}

pub async fn hl_sp(registers: Registers, memory: Memory) -> Result<u8, Error> {
    let (mut data, cycles) = Get::Next.get(registers.clone(), memory).await?;
    data += registers.borrow().get(Bits16::SP);
    registers.borrow_mut().set(Bits16::HL, data);
    Ok(cycles)
}

pub async fn sp_hl(registers: Registers, memory: Memory) -> Result<u8, Error> {
    let data = registers.borrow().get(Bits16::HL);
    let (_, cycles) = memory.get::<u8>(0xc000).await?;
    registers.borrow_mut().set(Bits16::SP, data);
    Ok(cycles)
}
