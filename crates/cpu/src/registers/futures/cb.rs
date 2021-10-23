use super::{GetAt, SetAt};
use crate::registers::{Bits16, Rotation, Shift};
use crate::Registers;
use memory::Memory;
use shared::Error;

/// RL = Rotate Left
/// RR = Rotate Right
/// SL = Shift Left
/// SR = Shift Right
pub enum Operation {
    RLCarry,
    RLNOCarry,
    RRCarry,
    RRNoCarry,
    SLeft,
    Swap,
    SRArithmetic,
    SRLogic,
}

fn calculate(registers: Registers, data: u8, operation: Operation) -> u8 {
    let mut registers = registers.borrow_mut();
    match operation {
        Operation::RLCarry => registers.left_carry(data),
        Operation::RLNOCarry => registers.left_nocarry(data),
        Operation::RRCarry => registers.right_carry(data),
        Operation::RRNoCarry => registers.right_nocarry(data),
        Operation::SLeft=> registers.shift_left(data),
        Operation::Swap => registers.swap(data),
        Operation::SRArithmetic => registers.shift_arithmetic(data),
        Operation::SRLogic => registers.shift_logic(data),
    }
}

pub(crate) async fn hl(
    registers: Registers,
    memory: Memory,
    operation: Operation,
) -> Result<u8, Error> {
    let (data, cycles) = registers.clone().get_at(memory.clone(), Bits16::HL).await?;
    let data = calculate(registers.clone(), data, operation);
    Ok(registers.set_at(memory, Bits16::HL, data).await? + cycles)
}
