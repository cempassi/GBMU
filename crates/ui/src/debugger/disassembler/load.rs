use super::instruction::{Data, Disass};
use cpu::opcodes::Jump;

impl From<Jump> for Disass {
    fn from(opcode: Jump) -> Self {
        let name = format!("{:?}", opcode);
        let (cycles, data): (Vec<u8>, Data) = match opcode {
            Jump::NN => (vec![16], Data::Bits16(0)),
            Jump::HL => (vec![16], Data::Bits16(0)),
            Jump::NZNN => (vec![16, 12], Data::Bits16(0)),
            Jump::NCNN => (vec![16, 12], Data::Bits16(0)),
            Jump::ZNN => (vec![16, 12], Data::Bits16(0)),
            Jump::CNN => (vec![16, 12], Data::Bits16(0)),
            Jump::R8b => (vec![12], Data::Bits8(0)),
            Jump::NZR8b => (vec![12, 8], Data::Bits8(0)),
            Jump::NCR8b => (vec![12, 8], Data::Bits8(0)),
            Jump::ZR8b => (vec![12, 8], Data::Bits8(0)),
            Jump::CR8b => (vec![12, 8], Data::Bits8(0)),
            Jump::Call => (vec![24], Data::Bits16(0)),
            Jump::CallZ => (vec![24, 12], Data::Bits16(0)),
            Jump::CallC => (vec![24, 12], Data::Bits16(0)),
            Jump::CallNZ => (vec![24, 12], Data::Bits16(0)),
            Jump::CallNC => (vec![24, 12], Data::Bits16(0)),
            Jump::Return => (vec![24], Data::Bits16(0)),
            Jump::ReturnZ => (vec![24, 8], Data::Bits16(0)),
            Jump::ReturnC => (vec![24, 8], Data::Bits16(0)),
            Jump::ReturnNZ => (vec![24, 8], Data::Bits16(0)),
            Jump::ReturnNC => (vec![20, 8], Data::Bits16(0)),
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
