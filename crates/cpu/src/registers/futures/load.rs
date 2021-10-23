use super::{AsyncGet, Get, Set};
use crate::registers::{Arithmetic, Bits16, Bits8, Bus};
use crate::Registers;
use memory::Memory;
use shared::Error;

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

pub async fn update(registers: Registers, memory: Memory, is_increase: bool) -> Result<u8, Error> {
    let (data, cycles) = Get::BitsAt(Bits16::HL)
        .get(registers.clone(), memory)
        .await?;
    registers.borrow_mut().set(Bits8::A, data);
    match is_increase {
        true => registers.borrow_mut().increase(Bits16::HL, 1),
        false => registers.borrow_mut().decrease(Bits16::HL, 1),
    };
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

pub async fn push(registers: Registers, memory: Memory, area: Bits16) -> Result<u8, Error> {
    let data = registers.borrow().get(area);
    let cycles = Set::Bits16At(Bits16::SP, data)
        .run(registers.clone(), memory)
        .await?;
    registers.borrow_mut().decrease(Bits16::SP, 2);
    Ok(cycles)
}

pub async fn pop(registers: Registers, memory: Memory, area: Bits16) -> Result<u8, Error> {
    let (data, cycles): (u16, u8) = Get::BitsAt(Bits16::SP)
        .get(registers.clone(), memory)
        .await?;
    registers.borrow_mut().set(area, data);
    registers.borrow_mut().increase(Bits16::SP, 2);
    Ok(cycles)
}
