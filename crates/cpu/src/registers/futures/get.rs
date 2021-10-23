use crate::registers::{Bits16, Bus};
use crate::Registers;
use memory::{Async, Memory};
use shared::Error;
use std::future::Future;
use std::pin::Pin;

type Getter<T> = Pin<Box<dyn Future<Output = Result<(T, u8), Error>>>>;

pub(crate) enum Get {
    Next,
    BitsAt(Bits16),
}

pub trait AsyncGet<T> {
    fn get(self, registers: Registers, memory: Memory) -> Getter<T>;
}

impl AsyncGet<u8> for Get {
    fn get(self, registers: Registers, memory: Memory) -> Getter<u8> {
        match self {
            Get::Next => Box::pin(bits8(registers, memory)),
            Get::BitsAt(area) => Box::pin(u8_at(registers, memory, area)),
        }
    }
}

impl AsyncGet<u16> for Get {
    fn get(self, registers: Registers, memory: Memory) -> Getter<u16> {
        match self {
            Get::Next => Box::pin(bits16(registers, memory)),
            Get::BitsAt(area) => Box::pin(u16_at(registers, memory, area)),
        }
    }
}

pub async fn bits8(registers: Registers, memory: Memory) -> Result<(u8, u8), Error> {
    let pc = registers.borrow().pc;
    let data = memory.get::<u8>(pc).await?;
    registers.borrow_mut().pc = pc.wrapping_add(1);
    Ok(data)
}

pub async fn bits16(registers: Registers, memory: Memory) -> Result<(u16, u8), Error> {
    let pc = registers.borrow().pc;
    let data = memory.get::<u16>(pc).await?;
    registers.borrow_mut().pc = pc.wrapping_add(2);
    Ok(data)
}

pub async fn u8_at(registers: Registers, memory: Memory, area: Bits16) -> Result<(u8, u8), Error> {
    let address = registers.borrow().get(area);
    memory.get::<u8>(address).await
}

pub async fn u16_at(
    registers: Registers,
    memory: Memory,
    area: Bits16,
) -> Result<(u16, u8), Error> {
    let address = registers.borrow().get(area);
    memory.get::<u16>(address).await
}
