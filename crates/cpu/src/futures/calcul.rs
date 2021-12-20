use super::{AsyncGet, Get, Set};
use crate::registers::{Arithmetic, Bits16, Bus, Flag, IncDec, Logical as L};
use crate::Cpu;
use crate::Registers;
use shared::Error;

#[derive(PartialEq, Eq)]
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

fn calculate(registers: &mut Registers, data: u8, operation: Operation) -> u8 {
    match operation {
        Operation::And => registers.and(data),
        Operation::Or => registers.or(data),
        Operation::Xor => registers.xor(data),
        Operation::Compare => registers.compare(data),
        Operation::AddCarry => registers.add(data, true),
        Operation::AddNoCarry => registers.add(data, false),
        Operation::SubCarry => registers.sub(data, true),
        Operation::SubNoCarry => registers.sub(data, false),
        Operation::Increase => registers.increase(data, 1),
        Operation::Decrease => registers.decrease(data, 1),
    }
}

pub(crate) async fn hl(cpu: Cpu, operation: Operation) -> Result<u8, Error> {
    let (data, mut cycles) = Get::BitsAt(Bits16::HL).get(cpu.clone()).await?;
    if operation == Operation::Increase || operation == Operation::Decrease {
        let data = calculate(&mut cpu.borrow_mut().registers, data, operation);
        cycles += Set::Bits8At(Bits16::HL, data).run(cpu).await?;
        Ok(cycles)
    } else {
        Ok(calculate(&mut cpu.borrow_mut().registers, data, operation) + cycles)
    }
}

pub(crate) async fn next(cpu: Cpu, operation: Operation) -> Result<u8, Error> {
    let (data, cycles): (u8, u8) = Get::Next.get(cpu.clone()).await?;
    Ok(calculate(&mut cpu.borrow_mut().registers, data, operation) + cycles)
}

pub async fn add_sp_signed(cpu: Cpu) -> Result<u8, Error> {
    let sp = cpu.borrow().registers.get(Bits16::SP);
    let (data, cycles): (u8, u8) = Get::Next.get(cpu.clone()).await?;
    let data = data as i8 as i16 as u16;
    cpu.borrow_mut().registers.set(Flag::N, false);
    cpu.borrow_mut().registers.set(Flag::Z, false);
    cpu.borrow_mut()
        .registers
        .set(Flag::H, (sp & 0x000F) + (data & 0x000F) > 0x000F);
    cpu.borrow_mut()
        .registers
        .set(Flag::C, (sp & 0x00FF) + (data & 0x00FF) > 0x00FF);
    cpu.borrow_mut()
        .registers
        .set(Bits16::SP, sp.wrapping_add(data));
    Ok(cycles)
}
