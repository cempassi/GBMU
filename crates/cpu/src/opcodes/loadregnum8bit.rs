use crate::area::Bits8;
use crate::pc::NextPc;
use crate::RegisterBus;
use crate::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;
use shared::Error;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum LoadRegNum8bit {
    B = 0x06,
    C = 0x0E,
    D = 0x16,
    E = 0x1E,
    H = 0x26,
    L = 0x2E,
}

#[allow(dead_code)]
impl<'a> LoadRegNum8bit {
    pub fn exec(
        self,
        registers: Registers,
        memory: Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.borrow_mut().pc.next(memory.clone()) {
            let bits = match self {
                LoadRegNum8bit::B => Bits8::B,
                LoadRegNum8bit::C => Bits8::C,
                LoadRegNum8bit::D => Bits8::D,
                LoadRegNum8bit::E => Bits8::E,
                LoadRegNum8bit::H => Bits8::H,
                LoadRegNum8bit::L => Bits8::L,
            };
            let result = memory.borrow_mut().set(byte.into(), registers.borrow().get(bits));
            match result {
                Ok(_) => Ok(8),
                Err(e) => Err(e),
            }
        } else {
            Err(Error::InvalidPC(registers.borrow().pc))
        }
    }
}
