use super::{GetAt, NextPc};
use crate::registers::{Bits16, Logical as L, Arithmetic};
use crate::Registers;
use memory::Memory;
use shared::Error;

pub enum Operation {
    AddCarry,
    SubCarry,
    AddNoCarry,
    SubNoCarry,
    And,
    Or,
    Xor,
    Compare,
}

fn calculate(registers: Registers, data: u8, operation: Operation) -> u8 {
    let mut registers = registers.borrow_mut();
    match operation {
        Operation::And => registers.and(data),
        Operation::Or => registers.or(data),
        Operation::Xor => registers.xor(data),
        Operation::Compare => registers.compare(data),
        Operation::AddCarry => registers.add(data, true),
        Operation::SubCarry => registers.add(data, true),
        Operation::AddNoCarry => registers.add(data, false),
        Operation::SubNoCarry => registers.add(data, false),
    }
}

pub(crate) async fn hl(
    registers: Registers,
    memory: Memory,
    operation: Operation,
) -> Result<u8, Error> {
    let (data, cycles)  = registers.clone().get_at(memory, Bits16::HL).await?;
    Ok(calculate(registers, data, operation) + cycles)
}

pub(crate) async fn next(
    registers: Registers,
    memory: Memory,
    operation: Operation,
) -> Result<u8, Error> {
    let (data, cycles) = registers.clone().next_pc(memory).await.unwrap();
    Ok(calculate(registers, data, operation) + cycles)
}
