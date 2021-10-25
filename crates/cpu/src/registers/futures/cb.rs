use super::{AsyncGet, Get, Set};
use crate::registers::{Bits16, Bitwise, Rotation, Shift};
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
    Reset(u8),
    Bitset(u8),
}

fn calculate(registers: Registers, data: u8, operation: Operation) -> u8 {
    let mut registers = registers.borrow_mut();
    match operation {
        Operation::RLCarry => registers.left_carry(data),
        Operation::RLNOCarry => registers.left_nocarry(data),
        Operation::RRCarry => registers.right_carry(data),
        Operation::RRNoCarry => registers.right_nocarry(data),
        Operation::SLeft => registers.shift_left(data),
        Operation::Swap => registers.swap(data),
        Operation::SRArithmetic => registers.shift_arithmetic(data),
        Operation::SRLogic => registers.shift_logic(data),
        Operation::Reset(bit) => registers.reset(data, bit),
        Operation::Bitset(bit) => registers.bitset(data, bit),
    }
}

pub(crate) async fn hl(
    registers: Registers,
    memory: Memory,
    operation: Operation,
) -> Result<u8, Error> {
    let (data, cycles) = Get::BitsAt(Bits16::HL)
        .get(registers.clone(), memory.clone())
        .await?;
    let data = calculate(registers.clone(), data, operation);
    Ok(Set::Bits8At(Bits16::HL, data)
        .run(registers, memory)
        .await?
        + cycles)
}

pub(crate) async fn test(registers: Registers, memory: Memory, bit: u8) -> Result<u8, Error> {
    let (data, cycles): (u8, u8) = Get::BitsAt(Bits16::HL)
        .get(registers.clone(), memory.clone())
        .await?;
    registers.borrow_mut().test(data, bit);
    Ok(cycles)
}
