use super::disass::Disass;
use crate::debugger::widgets::Cell;
use iced::{Element, Row};
use num_enum::TryFromPrimitive;

use super::DisassMsg;
use cpu::opcodes::Arithmetic;
use cpu::opcodes::Arithmetic16b;
use cpu::opcodes::Bitset;
use cpu::opcodes::Control;
use cpu::opcodes::Jump;
use cpu::opcodes::Load;
use cpu::opcodes::Load16b;
use cpu::opcodes::Logic;
use cpu::opcodes::Reset;
use cpu::opcodes::Rotate;
use cpu::opcodes::Shift;
use cpu::opcodes::Test;
use memory::Memory;
use shared::Error;

pub struct Instruction {
    address: u16,
    disass: Cycles,
    pub is_jump: bool,
    //is_next: bool,
}

impl Instruction {
    fn from_cb(opcode: u8) -> Cycles {
        if let Ok(opcode) = Rotate::try_from_primitive(opcode) {
            Cycles::Absolute(Disass::<u8>::from(opcode))
        } else if let Ok(opcode) = Shift::try_from_primitive(opcode) {
            Cycles::Absolute(Disass::<u8>::from(opcode))
        } else if let Ok(opcode) = Test::try_from_primitive(opcode) {
            Cycles::Absolute(Disass::<u8>::from(opcode))
        } else if let Ok(opcode) = Bitset::try_from_primitive(opcode) {
            Cycles::Absolute(Disass::<u8>::from(opcode))
        } else {
            let opcode = Reset::try_from_primitive(opcode).unwrap();
            Cycles::Absolute(Disass::<u8>::from(opcode))
        }
    }

    fn from_jump(jump: Jump) -> Result<Cycles, Error> {
        if let Ok(disass) = Disass::<(u8, u8)>::try_from(jump) {
            Ok(Cycles::Conditionnal(disass))
        } else {
            Ok(Cycles::Absolute(Disass::<u8>::try_from(jump)?))
        }
    }

    fn from_opcode(opcode: u8) -> Cycles {
        if let Ok(opcode) = Load::try_from_primitive(opcode) {
            Cycles::Absolute(Disass::<u8>::from(opcode))
        } else if let Ok(opcode) = Load16b::try_from_primitive(opcode) {
            Cycles::Absolute(Disass::<u8>::from(opcode))
        } else if let Ok(opcode) = Arithmetic::try_from_primitive(opcode) {
            Cycles::Absolute(Disass::<u8>::from(opcode))
        } else if let Ok(opcode) = Logic::try_from_primitive(opcode) {
            Cycles::Absolute(Disass::<u8>::from(opcode))
        } else if let Ok(opcode) = Control::try_from_primitive(opcode) {
            Cycles::Absolute(Disass::<u8>::from(opcode))
        } else if let Ok(opcode) = Arithmetic16b::try_from_primitive(opcode) {
            Cycles::Absolute(Disass::<u8>::from(opcode))
        } else {
            Cycles::unimplemented(opcode)
        }
    }

    pub fn try_new(address: u16, memory: &Memory, _is_next: bool) -> Result<Self, Error> {
        let opcode = memory.borrow().get_u8(address)?;
        let mut is_jump = false;

        let mut disass: Cycles = {
            if opcode == 0xCB {
                let opcode = memory.borrow().get_u8(address + 1)?;
                Self::from_cb(opcode)
            } else if let Ok(opcode) = Jump::try_from_primitive(opcode) {
                is_jump = true;
                Self::from_jump(opcode)?
            } else {
                Self::from_opcode(opcode)
            }
        };
        disass.set(memory, address)?;
        Ok(Self {
            address,
            disass,
            is_jump, // is_next,
        })
    }

    pub fn view(&mut self) -> Element<DisassMsg> {
        let address = format!("{:#04X}", self.address);
        let address = Cell::light(format!("{:^10}", address), 20);
        Row::new().push(address).push(self.disass.view()).into()
    }

    pub fn fetched(&self) -> u16 {
        match &self.disass {
            Cycles::Absolute(disass) => disass.fetched(),
            Cycles::Conditionnal(disass) => disass.fetched(),
        }
    }

    // pub fn get_cycle(&self) {
    //     match &self.disass {
    //         Cycles::Absolute(disass) => {
    //             println!("Cycle of next instruction: {}", disass.cycles);
    //         }
    //         Cycles::Conditionnal(disass) => {
    //             println!(
    //                 "Cycle of next instruction: {} or {}",
    //                 disass.cycles.0, disass.cycles.1
    //             );
    //         }
    //     }
    // }
}

enum Cycles {
    Absolute(Disass<u8>),
    Conditionnal(Disass<(u8, u8)>),
}

impl Cycles {
    pub fn unimplemented(opcode: u8) -> Self {
        Self::Absolute(Disass::unimplemented(opcode))
    }

    pub fn set(&mut self, memory: &Memory, address: u16) -> Result<(), Error> {
        match self {
            Cycles::Absolute(ref mut disass) => disass.data.set(memory, address),
            Cycles::Conditionnal(ref mut disass) => disass.data.set(memory, address),
        }
    }

    pub fn view(&mut self) -> Element<DisassMsg> {
        match self {
            Cycles::Absolute(ref mut disass) => Disass::<u8>::view(disass),
            Cycles::Conditionnal(ref mut disass) => Disass::<(u8, u8)>::view(disass),
        }
    }
}
