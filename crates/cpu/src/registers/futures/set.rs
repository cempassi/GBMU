use super::calcul::{self, Operation};
use super::cb::{self, Operation as CbOperation};
use super::load;
use super::setters as set;
use crate::registers::{Bits16, Bits8};
use crate::Registers;
use memory::Memory;
use shared::Error;
use std::future::Future;
use std::pin::Pin;

type Processing = Pin<Box<dyn Future<Output = Result<u8, Error>>>>;

#[allow(clippy::upper_case_acronyms)]
pub(crate) enum Set {
    CalculHL(Operation),
    CalculNext(Operation),
    CbHL(CbOperation),
    TestHL(u8),
    Load8b(Bits8),
    Load16b(Bits16),
    LoadHL(Bits8),
    LoadHL8b,
    HL(Bits8),
    RegisterAt(Bits16, Bits8),
    Bits8At(Bits16, u8),
    Bits16At(Bits16, u16),
    LoadRegisterFrom(Bits8, Bits16),
    Increase,
    Decrease,
    LoadIncrease,
    LoadDecrease,
    Data(Bits16),
    Pop(Bits16),
    Push(Bits16),
    LoadIOC,
    LoadIONext,
    IOC,
    IONext,
    LoadHLSP,
    LoadSPHL
}

impl Set {
    pub fn run(self, registers: Registers, memory: Memory) -> Processing {
        match self {
            Set::CalculHL(op) => Box::pin(calcul::hl(registers, memory, op)),
            Set::CalculNext(op) => Box::pin(calcul::next(registers, memory, op)),
            Set::Load8b(area) => Box::pin(load::u8(registers, memory, area)),
            Set::Load16b(area) => Box::pin(load::u16(registers, memory, area)),
            Set::LoadHL(area) => Box::pin(load::hl(registers, memory, area)),
            Set::LoadHL8b => Box::pin(load::hl8b(registers, memory)),
            Set::HL(area) => Box::pin(set::hl(registers, memory, area)),
            Set::RegisterAt(dst, src) => Box::pin(set::reg_at(registers, memory, dst, src)),
            Set::Bits8At(dst, src) => Box::pin(set::u8_at(registers, memory, dst, src)),
            Set::Bits16At(dst, src) => Box::pin(set::u16_at(registers, memory, dst, src)),
            Set::Data(area) => Box::pin(set::data(registers, memory, area)),
            Set::Push(area) => Box::pin(load::push(registers, memory, area)),
            Set::Pop(area) => Box::pin(load::pop(registers, memory, area)),
            Set::Increase => Box::pin(set::hl_add(registers, memory)),
            Set::Decrease => Box::pin(set::hl_sub(registers, memory)),
            Set::LoadIncrease => Box::pin(load::hl_add(registers, memory)),
            Set::LoadDecrease => Box::pin(load::hl_sub(registers, memory)),
            Set::LoadRegisterFrom(dst, src) => {
                Box::pin(load::reg_from(registers, memory, dst, src))
            }
            Set::CbHL(operation) => Box::pin(cb::hl(registers, memory, operation)),
            Set::TestHL(bit) => Box::pin(cb::test(registers, memory, bit)),
            Set::LoadIOC => Box::pin(load::io_c(registers, memory)),
            Set::LoadIONext => Box::pin(load::io_next(registers, memory)),
            Set::IOC => Box::pin(set::io_c(registers, memory)),
            Set::IONext => Box::pin(set::io_next(registers, memory)),
            Set::LoadHLSP => Box::pin(load::hl_sp(registers, memory)),
            Set::LoadSPHL => Box::pin(load::sp_hl(registers, memory)),
        }
    }
}
