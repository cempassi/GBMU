use crate::registers::{Bits16, Bits8};
use crate::bus::Bus;
use crate::cpu::Registers;
use crate::registers::futures::{GetAt, SetAt};
use memory::Memory;

pub(crate) enum Src {
    Register(Bits8),
    Pointer,
}

impl Src {
    pub async fn get(&self, registers: Registers, memory: Memory) -> u8 {
        match self {
            Src::Register(src) => registers.borrow().get(*src),
            Src::Pointer => registers.clone().get_at(memory, Bits16::HL).await.unwrap(),
        }
    }

    pub async fn set(&self, registers: Registers, memory: Memory, data: u8) {
        match self {
            Src::Register(src) => registers.borrow_mut().set(*src, data),
            Src::Pointer => registers
                .clone()
                .set_at(memory, Bits16::HL, data)
                .await
                .unwrap(),
        }
    }
}
