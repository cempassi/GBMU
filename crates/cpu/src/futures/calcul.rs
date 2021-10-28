use super::{AsyncGet, Get};
use crate::registers::{Arithmetic, Bits16, IncDec, Logical as L};
use crate::Registers;
use crate::{Access, Cpu};
use shared::Error;

pub enum Operation {
    AddCarry,
    SubCarry,
    AddNoCarry,
    SubNoCarry,
    Increase,
    Decrease,
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
        Operation::SubCarry => registers.sub(data, true),
        Operation::AddNoCarry => registers.add(data, false),
        Operation::SubNoCarry => registers.sub(data, false),
        Operation::Increase => registers.increase(data, 1),
        Operation::Decrease => registers.decrease(data, 1),
    }
}

pub(crate) async fn hl(cpu: Cpu, operation: Operation) -> Result<u8, Error> {
    let (data, cycles) = Get::BitsAt(Bits16::HL).get(cpu.clone()).await?;
    Ok(calculate(cpu.registers(), data, operation) + cycles)
}

pub(crate) async fn next(cpu: Cpu, operation: Operation) -> Result<u8, Error> {
    let (data, cycles) = Get::Next.get(cpu.clone()).await?;
    Ok(calculate(cpu.registers(), data, operation) + cycles)
}
