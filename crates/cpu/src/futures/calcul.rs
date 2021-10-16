use crate::area::Bits16;
use crate::logical::Logical;
use super::GetAt;
use crate::nextpc::NextPc;
use crate::{Reader, Registers};
use memory::Memory;
use shared::Error;
use std::future::Future;
use std::pin::Pin;

type Calculation = Pin<Box<dyn Future<Output = Result<(), Error>>>>;

pub enum Operation {
    And,
    Or,
    Xor,
    Comapre
}

pub(crate) trait CalculHL {
    fn do_hl(self, memory: Memory, operation: Operation) -> Calculation;
}

pub(crate) trait CalculNext {
    fn do_next(self, memory: Memory, operation: Operation) -> Calculation;
}

fn calculate(registers: Registers, data: u8, operation: Operation) ->  Result<(), Error>{
    let mut registers = registers.borrow_mut();
    match operation {
        Operation::And => registers.and(data),
        Operation::Or => registers.or(data),
        Operation::Xor => registers.xor(data),
        Operation::Comapre => registers.compare(data),
    }
    Ok(())
}

async fn get_hl(registers: Registers, memory: Memory, operation: Operation) ->  Result<(), Error>{
    let data: u8 = registers.clone().get_at(memory, Bits16::HL).await?;
    calculate(registers, data, operation)
}

async fn get_next(registers: Registers, memory: Memory, operation: Operation) ->  Result<(), Error>{
    let data = registers.clone().next_pc(memory).await.unwrap();
    calculate(registers, data, operation)
}

impl CalculHL for Registers {
    fn do_hl(self, memory: Memory, operation: Operation) -> Calculation {
        let inner = Box::pin(get_hl(self, memory, operation));
        Box::pin(Reader::new(inner))
    }
}

impl CalculNext for Registers {
    fn do_next(self, memory: Memory, operation: Operation) -> Calculation {
        let inner = Box::pin(get_next(self, memory, operation));
        Box::pin(Reader::new(inner))
    }
}
