use super::instruction::{Data, Disass};
use cpu::opcodes::Jump;
use shared::Error;

impl TryFrom<Jump> for Disass<(u8, u8)> {
    type Error = shared::Error;
    fn try_from(opcode: Jump) -> Result<Self, Self::Error> {
        let name = format!("{:?}", opcode);
        let (cycles, data): ((u8, u8), Data) = match opcode {
            Jump::NZNN => Ok(((16, 12), Data::Bits16(0))),
            Jump::NCNN => Ok(((16, 12), Data::Bits16(0))),
            Jump::ZNN => Ok(((16, 12), Data::Bits16(0))),
            Jump::CNN => Ok(((16, 12), Data::Bits16(0))),
            Jump::NZR8b => Ok(((12, 8), Data::Bits8(0))),
            Jump::NCR8b => Ok(((12, 8), Data::Bits8(0))),
            Jump::ZR8b => Ok(((12, 8), Data::Bits8(0))),
            Jump::CR8b => Ok(((12, 8), Data::Bits8(0))),
            Jump::CallZ => Ok(((24, 12), Data::Bits16(0))),
            Jump::CallC => Ok(((24, 12), Data::Bits16(0))),
            Jump::CallNZ => Ok(((24, 12), Data::Bits16(0))),
            Jump::CallNC => Ok(((24, 12), Data::Bits16(0))),
            Jump::ReturnZ => Ok(((24, 8), Data::Bits16(0))),
            Jump::ReturnC => Ok(((24, 8), Data::Bits16(0))),
            Jump::ReturnNZ => Ok(((24, 8), Data::Bits16(0))),
            Jump::ReturnNC => Ok(((20, 8), Data::Bits16(0))),
            _ => Err(Error::Unimplemented),
        }?;
        let code: u8 = opcode.into();
        Ok(Self {
            name,
            code,
            cycles,
            data,
        })
    }
}

impl TryFrom<Jump> for Disass<u8> {
    type Error = shared::Error;
    fn try_from(opcode: Jump) -> Result<Self, Self::Error> {
        let name = format!("{:?}", opcode);
        let (cycles, data): (u8, Data) = match opcode {
            Jump::NN => Ok((16, Data::Bits16(0))),
            Jump::HL => Ok((16, Data::Bits16(0))),
            Jump::R8b => Ok((12, Data::Bits8(0))),
            Jump::Call => Ok((24, Data::Bits16(0))),
            Jump::Return => Ok((24, Data::Bits16(0))),
            _ => Err(Error::Unimplemented),
        }?;
        let code: u8 = opcode.into();
        Ok(Self {
            name,
            code,
            cycles,
            data,
        })
    }
}
