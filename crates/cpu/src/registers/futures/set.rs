use super::calcul::{self, Operation};
use super::cb::{self, Operation as CbOperation};
use super::load;
use super::setters as set;
use crate::registers::{Bits16, Bits8};
use crate::Cpu;
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
    LoadSPHL,
}

impl Set {
    pub fn run(self, cpu: Cpu) -> Processing {
        match self {
            Set::CalculHL(op) => Box::pin(calcul::hl(cpu, op)),
            Set::CalculNext(op) => Box::pin(calcul::next(cpu, op)),
            Set::Load8b(area) => Box::pin(load::u8(cpu, area)),
            Set::Load16b(area) => Box::pin(load::u16(cpu, area)),
            Set::LoadHL(area) => Box::pin(load::hl(cpu, area)),
            Set::LoadHL8b => Box::pin(load::hl8b(cpu)),
            Set::HL(area) => Box::pin(set::hl(cpu, area)),
            Set::RegisterAt(dst, src) => Box::pin(set::reg_at(cpu, dst, src)),
            Set::Bits8At(dst, src) => Box::pin(set::u8_at(cpu, dst, src)),
            Set::Bits16At(dst, src) => Box::pin(set::u16_at(cpu, dst, src)),
            Set::Data(area) => Box::pin(set::data(cpu, area)),
            Set::Push(area) => Box::pin(load::push(cpu, area)),
            Set::Pop(area) => Box::pin(load::pop(cpu, area)),
            Set::Increase => Box::pin(set::hl_add(cpu)),
            Set::Decrease => Box::pin(set::hl_sub(cpu)),
            Set::LoadIncrease => Box::pin(load::hl_add(cpu)),
            Set::LoadDecrease => Box::pin(load::hl_sub(cpu)),
            Set::LoadRegisterFrom(dst, src) => Box::pin(load::reg_from(cpu, dst, src)),
            Set::CbHL(operation) => Box::pin(cb::hl(cpu, operation)),
            Set::TestHL(bit) => Box::pin(cb::test(cpu, bit)),
            Set::LoadIOC => Box::pin(load::io_c(cpu)),
            Set::LoadIONext => Box::pin(load::io_next(cpu)),
            Set::IOC => Box::pin(set::io_c(cpu)),
            Set::IONext => Box::pin(set::io_next(cpu)),
            Set::LoadHLSP => Box::pin(load::hl_sp(cpu)),
            Set::LoadSPHL => Box::pin(load::sp_hl(cpu)),
        }
    }
}
