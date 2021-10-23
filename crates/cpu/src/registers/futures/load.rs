use super::{GetAt, NextPc, SetAt};
use crate::registers::{Arithmetic, Bits16, Bits8, Bus};
use crate::Registers;
use memory::Memory;
use shared::Error;

pub async fn u8(registers: Registers, memory: Memory, area: Bits8) -> Result<u8, Error> {
    let (data, cycles) = registers.clone().next_pc(memory.clone()).await?;
    registers.borrow_mut().set(area, data);
    Ok(cycles)
}

pub async fn u16(registers: Registers, memory: Memory, area: Bits16) -> Result<u8, Error> {
    let (data, cycles) = registers.clone().next_pc(memory.clone()).await?;
    registers.borrow_mut().set(area, data);
    Ok(cycles)
}

pub async fn hl8b(registers: Registers, memory: Memory) -> Result<u8, Error> {
    let (data, cycles): (u8, u8) = registers.clone().next_pc(memory.clone()).await?;
    Ok(registers.set_at(memory, Bits16::HL, data).await? + cycles)
}

pub async fn hl(registers: Registers, memory: Memory, area: Bits8) -> Result<u8, Error> {
    let (data, cycles) = registers.clone().get_at(memory, Bits16::HL).await?;
    registers.borrow_mut().set(area, data);
    Ok(cycles)
}

// pub async fn load_sp(registers: Registers, memory: Memory, area: u16) -> Result<(), Error> {
//     let data = registers.borrow().get(Bits16::SP);
//     memory.set::<u16>(area, data).await
//}

pub async fn update(registers: Registers, memory: Memory, is_increase: bool) -> Result<u8, Error> {
    let (data, cycles) = registers.clone().get_at(memory, Bits16::HL).await?;
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
    let (data, cycles): (u8, u8) = registers.clone().get_at(memory, src).await?;
    registers.borrow_mut().set(dst, data);
    Ok(cycles)
}

pub async fn push(registers: Registers, memory: Memory, area: Bits16) -> Result<u8, Error> {
    let data = registers.borrow().get(area);
    let cycles = registers.clone().set_at(memory, Bits16::SP, data).await?;
    registers.borrow_mut().decrease(Bits16::SP, 2);
    Ok(cycles)
}

pub async fn pop(registers: Registers, memory: Memory, area: Bits16) -> Result<u8, Error> {
    let (data, cycles): (u16, u8) = registers.clone().get_at(memory, Bits16::SP).await?;
    registers.borrow_mut().set(area, data);
    registers.borrow_mut().increase(Bits16::SP, 2);
    Ok(cycles)
}
