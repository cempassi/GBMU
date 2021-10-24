use super::instruction::{Data, Disass};
use cpu::opcodes::Load;

impl From<Load> for Disass<u8> {
    fn from(opcode: Load) -> Self {
        let name = format!("{:?}", opcode);
        let (cycles, data): (u8, Data) = match opcode {
            Load::HL8b => (12, Data::Bits8(0)),
            Load::B => (8, Data::Bits8(0)),
            Load::C => (8, Data::Bits8(0)),
            Load::D => (8, Data::Bits8(0)),
            Load::E => (8, Data::Bits8(0)),
            Load::H => (8, Data::Bits8(0)),
            Load::L => (8, Data::Bits8(0)),
            Load::A => (8, Data::Bits8(0)),
            Load::BHL => (8, Data::Bits8(0)),
            Load::CHL => (8, Data::Bits8(0)),
            Load::DHL => (8, Data::Bits8(0)),
            Load::EHL => (8, Data::Bits8(0)),
            Load::HHL => (8, Data::Bits8(0)),
            Load::LHL => (8, Data::Bits8(0)),
            Load::AHL => (8, Data::Bits8(0)),
            Load::HLB => (8, Data::Bits8(0)),
            Load::HLC => (8, Data::Bits8(0)),
            Load::HLD => (8, Data::Bits8(0)),
            Load::HLE => (8, Data::Bits8(0)),
            Load::HLH => (8, Data::Bits8(0)),
            Load::HLL => (8, Data::Bits8(0)),
            Load::HLA => (8, Data::Bits8(0)),
            Load::BCA => (8, Data::Bits8(0)),
            Load::DEA => (8, Data::Bits8(0)),
            Load::ABC => (8, Data::Bits8(0)),
            Load::ADE => (8, Data::Bits8(0)),
            Load::HLPA => (8, Data::Bits8(0)),
            Load::HLMA => (8, Data::Bits8(0)),
            Load::AHLP => (8, Data::Bits8(0)),
            Load::AHLM => (8, Data::Bits8(0)),
            _ => (4, Data::Bits8(0)),
        };

        let code: u8 = opcode.into();
        Self {
            name,
            code,
            cycles,
            data,
        }
    }
}
