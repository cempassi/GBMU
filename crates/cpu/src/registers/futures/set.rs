use super::{NextPc, SetAt};
use crate::registers::{Arithmetic, Bits16, Bits8, Bus};
use crate::Registers;
use memory::{Async, Memory};
use shared::Error;

pub async fn set_update(
    registers: Registers,
    memory: Memory,
    is_increase: bool,
) -> Result<(), Error> {
    let data = registers.borrow().get(Bits8::A);
    registers.clone().set_at(memory, Bits16::HL, data).await?;
    match is_increase {
        true => registers.borrow_mut().increase(Bits16::HL, 1),
        false => registers.borrow_mut().decrease(Bits16::HL, 1),
    }
    Ok(())
}

pub async fn set_data(registers: Registers, memory: Memory, area: Bits16) -> Result<(), Error> {
    let data: u16 = registers.borrow().get(area);
    let dst: u16 = registers.next_pc(memory.clone()).await?;
    memory.set(dst, data).await
}

pub async fn set_reg_at(
    registers: Registers,
    memory: Memory,
    dst: Bits16,
    src: Bits8,
) -> Result<(), Error> {
    let data: u8 = registers.borrow().get(src);
    let address: u16 = registers.borrow().get(dst);
    memory.set(address, data).await
}

pub async fn set_hl(registers: Registers, memory: Memory, area: Bits8) -> Result<(), Error> {
    let data: u8 = registers.borrow().get(area);
    let dst: u16 = registers.borrow().get(Bits16::HL);
    memory.set(dst, data).await
}
