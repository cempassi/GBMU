use crate::{Access, Cpu};
use shared::Error;
use std::future::Future;
use std::pin::Pin;

use crate::futures::Set;
use crate::registers::{Bits16, IncDec};

use crate::opcodes::decode::{Decode, Decoder};
use crate::opcodes::Arithmetic;
use crate::opcodes::Arithmetic16b;
use crate::opcodes::Control;
use crate::opcodes::Jump;
use crate::opcodes::Load;
use crate::opcodes::Load16b;
use crate::opcodes::Logic;

use crate::futures::{AsyncGet, Get};
use num_enum::TryFromPrimitive;

pub type Output = Pin<Box<dyn Future<Output = Result<u8, Error>>>>;

pub trait Run {
    fn run(self) -> Output;
}

pub async fn interrupt_handler(cpu: Cpu) -> Result<u8, Error> {
    println!("Interrupt Execution");
    let requested = cpu.memory().borrow_mut().get_requested()?;
    cpu.borrow_mut().unset_halt();

    cpu.memory().borrow().is_enabled()?;

    cpu.memory().borrow_mut().disable_interrupts();

    let address = cpu.memory().borrow_mut().get_interrupt_address(requested)?;

    cpu.registers().borrow_mut().decrease(Bits16::SP, 2);
    let pc = cpu.registers().borrow().pc;

    cpu.registers().borrow_mut().pc = address;
    let cycles = Set::Bits16At(Bits16::SP, pc).run(cpu).await?;

    Ok(cycles)
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
    if cpu.borrow().interrupt_enabled() {
        match interrupt_handler(cpu.clone()).await {
            Err(Error::DisabledInterrupts) => (),
            result => return result,
        };
    }
    cpu.memory().borrow_mut().check_interrupts();
    let (opcode, cycles) = Get::Next.get(cpu.clone()).await?;
    println!("New Cpu Execution, Opcode: {:#X}", opcode);
    if cpu.borrow().is_halted() {
        println!("Cpu is halted, Opcode: {:#X}", opcode);
    }

    let execute = decode(cpu, opcode).await?;
    Ok(execute.await? + cycles)
}

impl Run for Cpu {
    fn run(self) -> Output {
        Box::pin(run(self))
    }
}
