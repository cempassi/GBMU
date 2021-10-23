use super::calcul::{self, Operation};
use super::cb::{self, Operation as CbOperation};
use super::load;
use super::set;
use crate::registers::{Bits16, Bits8};
use crate::Registers;
use memory::Memory;
use shared::Error;
use std::future::Future;
use std::pin::Pin;

type Processing = Pin<Box<dyn Future<Output = Result<u8, Error>>>>;

pub(crate) enum Async {
    CalculHL(Operation),
    CalculNext(Operation),
    CbHL(CbOperation),
    Load8b(Bits8),
    Load16b(Bits16),
    LoadHL(Bits8),
    LoadHL8b,
    SetHL(Bits8),
    SetRegisterAt(Bits16, Bits8),
    LoadRegisterFrom(Bits8, Bits16),
    SetIncrease,
    SetDecrease,
    LoadIncrease,
    LoadDecrease,
    //LoadSP(u16),
    SetData(Bits16),
    Pop(Bits16),
    Push(Bits16),
}

impl Async {
    pub fn run(self, registers: Registers, memory: Memory) -> Processing {
        match self {
            Async::CalculHL(op) => Box::pin(calcul::hl(registers, memory, op)),
            Async::CalculNext(op) => Box::pin(calcul::next(registers, memory, op)),
            Async::Load8b(area) => Box::pin(load::u8(registers, memory, area)),
            Async::Load16b(area) => Box::pin(load::u16(registers, memory, area)),
            Async::LoadHL(area) => Box::pin(load::hl(registers, memory, area)),
            Async::LoadHL8b => Box::pin(load::hl8b(registers, memory)),
            Async::SetHL(area) => Box::pin(set::hl(registers, memory, area)),
            Async::SetRegisterAt(dst, src) => Box::pin(set::reg_at(registers, memory, dst, src)),
            Async::SetData(area) => Box::pin(set::data(registers, memory, area)),
            Async::Push(area) => Box::pin(load::push(registers, memory, area)),
            Async::Pop(area) => Box::pin(load::pop(registers, memory, area)),
            Async::SetIncrease => Box::pin(set::update(registers, memory, true)),
            Async::SetDecrease => Box::pin(set::update(registers, memory, false)),
            Async::LoadIncrease => Box::pin(load::update(registers, memory, true)),
            Async::LoadDecrease => Box::pin(load::update(registers, memory, false)),
            Async::LoadRegisterFrom(dst, src) => {
                Box::pin(load::reg_from(registers, memory, dst, src))
            }
            Async::CbHL(operation) => Box::pin(cb::hl(registers, memory, operation)),
            // Async::LoadSP(area) =>{
            //     Box::pin(Reader::new(Box::pin(load_sp(registers, memory, area))))
            // }
        }
    }
}
