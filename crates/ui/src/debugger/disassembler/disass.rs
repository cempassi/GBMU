use super::DisassMsg;
use crate::debugger::widgets::Cell;
use iced_wgpu::{Renderer, Row};
use iced_winit::Element;
use memory::Memory;
use shared::Error;

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
        let code = Cell::light(format!("{:^6}", code), 20);

        let cycles = format!("{:>2}", self.cycles);
        let cycles = Cell::light(format!("{:^12}", cycles), 20);

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
        let name = Cell::light(self.name.clone(), 20);

        let code = format!("{:#X}", self.opcode);
        let code = Cell::light(format!("{:^6}", code), 20);

        let cycles = format!("{}/{}", self.cycles.0, self.cycles.1);
        let cycles = Cell::light(format!("{:^12}", cycles), 20);

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
        format!("{:^26}", name)
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
            Data::None | Data::Cb => format!("{:^6}", "None"),
            Data::Bits8(data) => {
                format!("{:^6}", format!("{:#X}", *data))
            }
            Data::Bits16(data) => {
                format!("{:^6}", format!("{:#X}", *data))
            }
        }
    }
}
