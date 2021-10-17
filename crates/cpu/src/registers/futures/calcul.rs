use super::{GetAt, NextPc, Reader};
use crate::registers::{Bits16, Logical};
use crate::Registers;
use memory::Memory;
use shared::Error;
use std::future::Future;
use std::pin::Pin;

type Calculation = Pin<Box<dyn Future<Output = Result<(), Error>>>>;

pub enum Operation {
    And,
    Or,
    Xor,
    Comapre,
}

pub(crate) trait LogicalHL {
    fn do_hl(self, memory: Memory, operation: Operation) -> Calculation;
}

pub(crate) trait LogicalNext {
    fn do_next(self, memory: Memory, operation: Operation) -> Calculation;
}

fn calculate(registers: Registers, data: u8, operation: Operation) -> Result<(), Error> {
    let mut registers = registers.borrow_mut();
    match operation {
        Operation::And => registers.and(data),
        Operation::Or => registers.or(data),
        Operation::Xor => registers.xor(data),
        Operation::Comapre => registers.compare(data),
    }
    Ok(())
}

async fn get_hl(registers: Registers, memory: Memory, operation: Operation) -> Result<(), Error> {
    let data: u8 = registers.clone().get_at(memory, Bits16::HL).await?;
    calculate(registers, data, operation)
}

async fn get_next(registers: Registers, memory: Memory, operation: Operation) -> Result<(), Error> {
    let data = registers.clone().next_pc(memory).await.unwrap();
    calculate(registers, data, operation)
}

impl LogicalHL for Registers {
    fn do_hl(self, memory: Memory, operation: Operation) -> Calculation {
        let inner = Box::pin(get_hl(self, memory, operation));
        Box::pin(Reader::new(inner))
    }
}

impl LogicalNext for Registers {
    fn do_next(self, memory: Memory, operation: Operation) -> Calculation {
        let inner = Box::pin(get_next(self, memory, operation));
        Box::pin(Reader::new(inner))
    }
}
