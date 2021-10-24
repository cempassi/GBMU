use super::instruction::{Data, Disass};
use cpu::opcodes::Load16b;

impl From<Load16b> for Disass<u8> {
    fn from(opcode: Load16b) -> Self {
        let name = format!("{:?}", opcode);
        let (cycles, data): (u8, Data) = match opcode {
            Load16b::PushAF => (16, Data::Bits8(0)),
            Load16b::PushBC => (16, Data::Bits8(0)),
            Load16b::PushDE => (16, Data::Bits8(0)),
            Load16b::PushHL => (16, Data::Bits8(0)),
            _ => (12, Data::Bits8(0)),
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
