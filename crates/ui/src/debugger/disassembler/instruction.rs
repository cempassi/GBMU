use iced_wgpu::{Renderer, Row};
use iced_winit::Element;
use num_enum::TryFromPrimitive;

use super::DisassMsg;
use crate::debugger::widgets::Cell;
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

        let mut disass: Cycles = {
            if opcode == 0xCB {
                let opcode = memory.borrow().get_u8(address + 1)?;
                Self::from_cb(opcode)
            } else if let Ok(opcode) = Jump::try_from_primitive(opcode) {
                Self::from_jump(opcode)?
            } else {
                Self::from_opcode(opcode)
            }
        };
        disass.set(memory, address)?;
        Ok(Self {
            address,
            disass,
            // is_next,
        })
    }

    pub fn view(&mut self) -> Element<DisassMsg, Renderer> {
        let address = Cell::light(format!("{:^12}", format!("{:#04X}", self.address)), 20);
        Row::new().push(address).push(self.disass.view()).into()
    }

    pub fn fetched(&self) -> u16 {
        match &self.disass {
            Cycles::Absolute(disass) => disass.fetched(),
            Cycles::Conditionnal(disass) => disass.fetched(),
        }
    }
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

    pub fn view(&mut self) -> Element<DisassMsg, Renderer> {
        match self {
            Cycles::Absolute(ref mut disass) => Disass::<u8>::view(disass),
            Cycles::Conditionnal(ref mut disass) => Disass::<(u8, u8)>::view(disass),
        }
    }
}

pub(super) struct Disass<T> {
    pub(super) name: String,
    pub(super) opcode: u8,
    pub(super) cycles: T,
    pub(super) data: Data, //description: String,
}

impl Disass<u8> {
    pub fn view(&mut self) -> Element<DisassMsg, Renderer> {
        let name = Cell::light(self.name.clone(), 20);
        let code = format!("{:#04X}", self.opcode);
        let code = Cell::light(format!("{:^14}", code), 20);
        let cycles = Cell::light(format!("{:^12}", format!("{:>2}", self.cycles)), 20);
        let data = Cell::light(self.data.to_string(), 20);

        Row::new()
            .push(name)
            .push(code)
            .push(cycles)
            .push(data)
            .into()
    }

    pub fn unimplemented(opcode: u8) -> Self {
        let name = "Unimplemented".to_string();
        let cycles = 4;
        let data = Data::None;
        Self {
            name,
            opcode,
            cycles,
            data,
        }
    }
}

impl Disass<(u8, u8)> {
    pub fn view(&mut self) -> Element<DisassMsg, Renderer> {
        let name = Cell::light(self.name.to_string(), 20);
        let code = format!("{:#X}", self.opcode);
        let code = Cell::light(format!("{:^12}", code), 20);
        let cycles = format!("{:^12}", format!("{}/{}", self.cycles.0, self.cycles.1));
        let cycles = Cell::light(cycles, 20);
        let data = Cell::light(self.data.to_string(), 20);

        Row::new()
            .push(name)
            .push(code)
            .push(cycles)
            .push(data)
            .into()
    }
}

impl<T> Disass<T> {
    pub fn name(name: String) -> String {
        format!("{:^16}", name)
    }
}

pub enum Data {
    None,
    Cb,
    Bits8(u8),
    Bits16(u16),
}

impl<T> Disass<T> {
    pub fn fetched(&self) -> u16 {
        match self.data {
            Data::None => 1,
            Data::Bits8(_) | Data::Cb => 2,
            Data::Bits16(_) => 3,
        }
    }
}

impl Data {
    pub fn set(&mut self, memory: &Memory, address: u16) -> Result<(), Error> {
        match self {
            Data::None | Data::Cb => (),
            Data::Bits8(ref mut data) => *data = memory.borrow().get_u8(address + 1)?,
            Data::Bits16(ref mut data) => *data = memory.borrow().get_u16(address + 1)?,
        };
        Ok(())
    }
}

impl ToString for Data {
    fn to_string(&self) -> String {
        match self {
            Data::None | Data::Cb => format!("{:^16}", "None"),
            Data::Bits8(data) => {
                format!("{:^16}", format!("{:#X}", *data))
            }
            Data::Bits16(data) => {
                format!("{:^16}", format!("{:#X}", *data))
            }
        }
    }
}
