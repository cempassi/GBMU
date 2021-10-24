use iced_wgpu::{Renderer, Row, Text};
use iced_winit::Element;
use num_enum::TryFromPrimitive;

use super::DisassMsg;
use crate::style::fonts;
use cpu::opcodes::Jump;
use memory::Memory;
use shared::Error;

pub struct Instruction {
    address: u16,
    disass: Disass,
    is_next: bool,
}

impl Instruction {
    pub fn try_new(address: u16, memory: &Memory, is_next: bool) -> Result<Self, Error> {
        let opcode = memory.borrow().get_u8(address)?;

        let mut disass: Disass = {
            if let Ok(opcode) = Jump::try_from_primitive(opcode) {
                Ok(Disass::from(opcode))
            } else {
                Err(Error::Unimplemented)
            }
        }?;
        disass.data.set(memory, address)?;
        Ok(Self {
            address,
            disass,
            is_next,
        })
    }

    pub fn view(&mut self) -> Element<DisassMsg, Renderer> {
        let address = Text::new(format!("{:#X}", self.address))
            .font(fonts::HASKLIG_LIGHT)
            .size(20);
        Row::new().push(address).push(self.disass.view()).into()
    }
}

pub enum Data {
    None,
    Bits8(u8),
    Bits16(u16),
}

impl Data {
    pub fn set(&mut self, memory: &Memory, address: u16) -> Result<(), Error> {
        match self {
            Data::None => (),
            Data::Bits8(ref mut data) => *data = memory.borrow().get_u8(address)?,
            Data::Bits16(ref mut data) => *data = memory.borrow().get_u16(address)?,
        };
        Ok(())
    }
}

impl ToString for Data {
    fn to_string(&self) -> String {
        match self {
            Data::None => "None".to_owned(),
            Data::Bits8(data) => format!("{:#X}", data),
            Data::Bits16(data) => format!("{:#X}", data),
        }
    }
}

pub(super) struct Disass {
    pub(super) name: String,
    pub(super) code: u8,
    pub(super) cycles: Vec<u8>,
    pub(super) data: Data, //description: String,
}

impl Disass {
    pub fn view(&mut self) -> Element<DisassMsg, Renderer> {
        let name = Text::new(self.name.to_string())
            .font(fonts::HASKLIG_LIGHT)
            .size(20);
        let code = Text::new(format!("{:#X}", self.code))
            .font(fonts::HASKLIG_LIGHT)
            .size(20);
        let cycles = {
            let cycles = match self.cycles.len() {
                1 => format!("{}", self.cycles[0]),
                2 => format!("{}/{}", self.cycles[0], self.cycles[1]),
                _ => unreachable!(),
            };

            Text::new(cycles).font(fonts::HASKLIG_LIGHT).size(20)
        };
        let data = Text::new(self.data.to_string())
            .font(fonts::HASKLIG_LIGHT)
            .size(20);

        Row::new()
            .push(name)
            .push(code)
            .push(cycles)
            .push(data)
            .into()
    }
}
