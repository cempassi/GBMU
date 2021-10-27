use super::{AsyncGet, Get, Set};
use crate::registers::{Bits16, Bits8, Bus, IncDec};
use crate::Registers;
use memory::{Async as A, Memory};
use shared::Error;

const IO_REG: u16 = 0xFF00;

pub async fn update(registers: Registers, memory: Memory, is_increase: bool) -> Result<u8, Error> {
    let cycles = Set::RegisterAt(Bits16::HL, Bits8::A)
        .run(registers.clone(), memory)
        .await?;
    match is_increase {
        true => registers.borrow_mut().increase(Bits16::HL, 1),
        false => registers.borrow_mut().decrease(Bits16::HL, 1),
    };
    Ok(cycles)
}

pub async fn data(registers: Registers, memory: Memory, area: Bits16) -> Result<u8, Error> {
    let data: u16 = registers.borrow().get(area);
    let (dst, cycles) = Get::Next.get(registers.clone(), memory.clone()).await?;

    Ok(memory.set(dst, data).await? + cycles)
}

pub async fn reg_at(
    registers: Registers,
    memory: Memory,
    dst: Bits16,
    src: Bits8,
) -> Result<u8, Error> {
    let data: u8 = registers.borrow().get(src);
    let address: u16 = registers.borrow().get(dst);
    memory.set(address, data).await
}

pub async fn u8_at(
    registers: Registers,
    memory: Memory,
    dst: Bits16,
    data: u8,
) -> Result<u8, Error> {
    let address: u16 = registers.borrow().get(dst);
    memory.set(address, data).await
}

pub async fn u16_at(
    registers: Registers,
    memory: Memory,
    dst: Bits16,
    data: u16,
) -> Result<u8, Error> {
    let address: u16 = registers.borrow().get(dst);
    memory.set(address, data).await
}

pub async fn hl(registers: Registers, memory: Memory, area: Bits8) -> Result<u8, Error> {
    let data: u8 = registers.borrow().get(area);
    let dst: u16 = registers.borrow().get(Bits16::HL);
    memory.set(dst, data).await
}

pub async fn io_c(registers: Registers, memory: Memory) -> Result<u8, Error> {
    let a = registers.borrow_mut().get(Bits8::A);
    let c: u16 = registers.borrow().get(Bits8::C).into();
    let cycles = memory.set::<u8>(c + IO_REG, a).await?;
    Ok(cycles)
}

pub async fn io_next(registers: Registers, memory: Memory) -> Result<u8, Error> {
    let a = registers.borrow_mut().get(Bits8::A);
    let (data, mut cycles): (u8, u8) = Get::Next.get(registers.clone(), memory.clone()).await?;
    cycles += memory.set::<u8>(u16::from(data) + IO_REG, a).await?;
    Ok(cycles)
}
