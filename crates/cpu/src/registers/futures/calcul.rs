use super::{GetAt, NextPc};
use crate::registers::{Bits16, Logical as L};
use crate::Registers;
use memory::Memory;
use shared::Error;

pub enum Logical {
    And,
    Or,
    Xor,
    Compare,
}

fn calculate(registers: Registers, data: u8, operation: Logical) -> Result<(), Error> {
    let mut registers = registers.borrow_mut();
    match operation {
        Logical::And => registers.and(data),
        Logical::Or => registers.or(data),
        Logical::Xor => registers.xor(data),
        Logical::Compare => registers.compare(data),
    }
    Ok(())
}

pub(crate) async fn hl(
    registers: Registers,
    memory: Memory,
    operation: Logical,
) -> Result<(), Error> {
    let data: u8 = registers.clone().get_at(memory, Bits16::HL).await?;
    calculate(registers, data, operation)
}

pub(crate) async fn next(
    registers: Registers,
    memory: Memory,
    operation: Logical,
) -> Result<(), Error> {
    let data = registers.clone().next_pc(memory).await.unwrap();
    calculate(registers, data, operation)
}
