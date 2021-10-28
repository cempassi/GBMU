use crate::Cpu;
use shared::Error;
use std::future::Future;
use std::pin::Pin;

use crate::opcodes::decode::{Decode, Decoder};
use crate::opcodes::Arithmetic;
use crate::opcodes::Arithmetic16b;
use crate::opcodes::Control;
use crate::opcodes::Jump;
use crate::opcodes::Load;
use crate::opcodes::Load16b;
use crate::opcodes::Logic;

use crate::registers::futures::{AsyncGet, Get};
use num_enum::TryFromPrimitive;

pub type Output = Pin<Box<dyn Future<Output = Result<u8, Error>>>>;

pub trait Run {
    fn run(self) -> Output;
}

async fn decode(cpu: Cpu, opcode: u8) -> Result<Decode, Error> {
    if let Ok(operation) = Control::try_from_primitive(opcode) {
        Ok(operation.decode(cpu))
    } else if let Ok(operation) = Load::try_from_primitive(opcode) {
        Ok(operation.decode(cpu))
    } else if let Ok(operation) = Load16b::try_from_primitive(opcode) {
        Ok(operation.decode(cpu))
    } else if let Ok(operation) = Jump::try_from_primitive(opcode) {
        Ok(operation.decode(cpu))
    } else if let Ok(operation) = Arithmetic::try_from_primitive(opcode) {
        Ok(operation.decode(cpu))
    } else if let Ok(operation) = Arithmetic16b::try_from_primitive(opcode) {
        Ok(operation.decode(cpu))
    } else if let Ok(operation) = Logic::try_from_primitive(opcode) {
        Ok(operation.decode(cpu))
    } else {
        println!("Something went wrong?, opcode: {}", opcode);
        Err(Error::Unimplemented)
    }
}

pub async fn run(cpu: Cpu) -> Result<u8, Error> {
    let (opcode, cycles) = Get::Next.get(cpu.clone()).await?;
    println!("New Cpu Execution, Opcode: {:#X}", opcode);

    let execute = decode(cpu, opcode).await?;
    Ok(execute.await? + cycles)
}

impl Run for Cpu {
    fn run(self) -> Output {
        Box::pin(run(self))
    }
}
