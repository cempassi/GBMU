use crate::{Access, Cpu};
use shared::{Error, Finished, Output, Run};

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

impl Run for Cpu {
    fn run(self) -> Output {
        Box::pin(run(self))
    }
}

pub async fn interrupt_handler(cpu: Cpu) -> Result<Finished, Error> {
    let address = cpu.memory().borrow_mut().get_interrupt_address();

    if let Some(address) = address {
        println!("Interrupt Execution, address: {:#X}", address);
        cpu.borrow_mut().halt = false;
        cpu.borrow_mut().stop = false;
        cpu.borrow_mut().registers.decrease(Bits16::SP, 2);
        cpu.memory().borrow_mut().disable_interrupts();
        let pc = cpu.borrow().registers.pc;
        cpu.borrow_mut().registers.pc = address;
        let cycles = Set::Bits16At(Bits16::SP, pc).run(cpu).await?;
        Ok(Finished::Cpu(cycles))
    } else {
        println!("No interrupts to handle");
        cpu.memory().borrow_mut().disable_interrupts();
        Ok(Finished::Cpu(0))
    }
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
        println!(
            "Something went wrong(Decode in runner.rs), opcode: {:#X}",
            opcode
        );
        Err(Error::Unimplemented(opcode))
    }
}

pub async fn run(cpu: Cpu) -> Result<Finished, Error> {
    let cycles = if !cpu.borrow().halt && !cpu.borrow().stop {
        if cpu.borrow().interrupt_enabled() {
            match interrupt_handler(cpu.clone()).await {
                Err(Error::DisabledInterrupts) => Ok(Finished::Cpu(1)),
                result => result,
            }
        } else {
            let (opcode, cycles) = Get::Next.get(cpu.clone()).await?;
            // println!("New Cpu Execution, Opcode: {:#X}", opcode);

            let execute = decode(cpu.clone(), opcode).await?;
            Ok(Finished::Cpu(execute.await? + cycles))
        }
    } else {
        println!("Halted!");
        if cpu.memory().borrow().is_requested() {
            cpu.borrow_mut().halt = false;
        }
        Ok(Finished::Cpu(1))
    };

    cpu.memory().borrow_mut().check_interrupts();
    cycles
}
