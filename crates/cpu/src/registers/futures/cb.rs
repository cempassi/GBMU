use super::{AsyncGet, Get, Set};
use crate::registers::{Bits16, Bitwise, Rotation, Shift};
use crate::Registers;
use crate::{Access, Cpu};
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

pub(crate) async fn hl(cpu: Cpu, operation: Operation) -> Result<u8, Error> {
    let (data, cycles) = Get::BitsAt(Bits16::HL).get(cpu.clone()).await?;
    let data = calculate(cpu.registers(), data, operation);
    Ok(Set::Bits8At(Bits16::HL, data).run(cpu).await? + cycles)
}

pub(crate) async fn test(cpu: Cpu, bit: u8) -> Result<u8, Error> {
    let (data, cycles): (u8, u8) = Get::BitsAt(Bits16::HL).get(cpu.clone()).await?;
    cpu.registers().borrow_mut().test(data, bit);
    Ok(cycles)
}
