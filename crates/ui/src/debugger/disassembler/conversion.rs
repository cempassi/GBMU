use super::instruction::{Data, Disass};
use cpu::opcodes::Arithmetic;
use cpu::opcodes::Bitset;
use cpu::opcodes::Jump;
use cpu::opcodes::Load;
use cpu::opcodes::Load16b;
use cpu::opcodes::Logic;
use cpu::opcodes::Reset;
use cpu::opcodes::Rotate;
use cpu::opcodes::Shift;
use cpu::opcodes::Test;
use shared::Error;

impl From<Bitset> for Disass<u8> {
    fn from(opcode: Bitset) -> Self {
        let name = Self::name(format!("Bitset {:?}", opcode));
        let (cycles, data): (u8, Data) = match opcode {
            Bitset::HLBit0 => (12, Data::None),
            Bitset::HLBit1 => (12, Data::None),
            Bitset::HLBit2 => (12, Data::None),
            Bitset::HLBit3 => (12, Data::None),
            Bitset::HLBit4 => (12, Data::None),
            Bitset::HLBit5 => (12, Data::None),
            Bitset::HLBit6 => (12, Data::None),
            Bitset::HLBit7 => (12, Data::None),
            _ => (8, Data::None),
        };

        let code: u8 = opcode.into();
        Self {
            name,
            opcode: code,
            cycles,
            data,
        }
    }
}

impl From<Reset> for Disass<u8> {
    fn from(opcode: Reset) -> Self {
        let name = Self::name(format!("Reset {:?}", opcode));
        let (cycles, data): (u8, Data) = match opcode {
            Reset::HLBit0 => (12, Data::None),
            Reset::HLBit1 => (12, Data::None),
            Reset::HLBit2 => (12, Data::None),
            Reset::HLBit3 => (12, Data::None),
            Reset::HLBit4 => (12, Data::None),
            Reset::HLBit5 => (12, Data::None),
            Reset::HLBit6 => (12, Data::None),
            Reset::HLBit7 => (12, Data::None),
            _ => (8, Data::None),
        };

        let code: u8 = opcode.into();
        Self {
            name,
            opcode: code,
            cycles,
            data,
        }
    }
}

impl From<Test> for Disass<u8> {
    fn from(opcode: Test) -> Self {
        let name = Self::name(format!("Test {:?}", opcode));
        let (cycles, data): (u8, Data) = match opcode {
            Test::HLBit0 => (12, Data::None),
            Test::HLBit1 => (12, Data::None),
            Test::HLBit2 => (12, Data::None),
            Test::HLBit3 => (12, Data::None),
            Test::HLBit4 => (12, Data::None),
            Test::HLBit5 => (12, Data::None),
            Test::HLBit6 => (12, Data::None),
            Test::HLBit7 => (12, Data::None),
            _ => (8, Data::None),
        };

        let code: u8 = opcode.into();
        Self {
            name,
            opcode: code,
            cycles,
            data,
        }
    }
}

impl TryFrom<Jump> for Disass<(u8, u8)> {
    type Error = shared::Error;
    fn try_from(opcode: Jump) -> Result<Self, Self::Error> {
        let name = Self::name(format!("{:?}", opcode));
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
            opcode: code,
            cycles,
            data,
        })
    }
}

impl TryFrom<Jump> for Disass<u8> {
    type Error = shared::Error;
    fn try_from(opcode: Jump) -> Result<Self, Self::Error> {
        let name = Self::name(format!("{:?}", opcode));
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
            opcode: code,
            cycles,
            data,
        })
    }
}

impl From<Logic> for Disass<u8> {
    fn from(opcode: Logic) -> Self {
        let name = Self::name(format!("{:?}", opcode));
        let (cycles, data): (u8, Data) = match opcode {
            Logic::AndAHL => (8, Data::Bits8(0)),
            Logic::AndA8b => (8, Data::Bits8(0)),
            Logic::OrAHL => (8, Data::Bits8(0)),
            Logic::OrA8b => (8, Data::Bits8(0)),
            Logic::XorAHL => (8, Data::Bits8(0)),
            Logic::XorA8b => (8, Data::Bits8(0)),
            Logic::CmpAHL => (8, Data::Bits8(0)),
            Logic::CmpA8b => (8, Data::Bits8(0)),
            _ => (4, Data::None),
        };

        let code: u8 = opcode.into();
        Self {
            name,
            opcode: code,
            cycles,
            data,
        }
    }
}

impl From<Rotate> for Disass<u8> {
    fn from(opcode: Rotate) -> Self {
        let name = Self::name(format!("Rotate {:?}", opcode));
        let (cycles, data): (u8, Data) = match opcode {
            Rotate::LCHL => (8, Data::Bits8(0)),
            Rotate::LHL => (8, Data::Bits8(0)),
            Rotate::RCHL => (8, Data::Bits8(0)),
            Rotate::RHL => (8, Data::Bits8(0)),
            _ => (4, Data::None),
        };

        let code: u8 = opcode.into();
        Self {
            name,
            opcode: code,
            cycles,
            data,
        }
    }
}

impl From<Load> for Disass<u8> {
    fn from(opcode: Load) -> Self {
        let name = Self::name(format!("Load {:?}", opcode));
        let (cycles, data): (u8, Data) = match opcode {
            Load::HL8b => (12, Data::Bits8(0)),
            Load::B8b => (8, Data::Bits8(0)),
            Load::C8b => (8, Data::Bits8(0)),
            Load::D8b => (8, Data::Bits8(0)),
            Load::E8b => (8, Data::Bits8(0)),
            Load::H8b => (8, Data::Bits8(0)),
            Load::L8b => (8, Data::Bits8(0)),
            Load::A8b => (8, Data::Bits8(0)),
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
            _ => (4, Data::None),
        };

        let code: u8 = opcode.into();
        Self {
            name,
            opcode: code,
            cycles,
            data,
        }
    }
}

impl From<Load16b> for Disass<u8> {
    fn from(opcode: Load16b) -> Self {
        let name = Self::name(format!("{:?}", opcode));
        let (cycles, data): (u8, Data) = match opcode {
            Load16b::PushAF => (16, Data::None),
            Load16b::PushBC => (16, Data::None),
            Load16b::PushDE => (16, Data::None),
            Load16b::PushHL => (16, Data::None),
            Load16b::LoadBC => (16, Data::Bits16(0)),
            Load16b::LoadDE => (16, Data::Bits16(0)),
            Load16b::LoadHL => (16, Data::Bits16(0)),
            Load16b::LoadSP => (16, Data::Bits16(0)),
            Load16b::LoadA16SP => (16, Data::Bits16(0)),
            _ => (12, Data::None),
        };

        let code: u8 = opcode.into();
        Self {
            name,
            opcode: code,
            cycles,
            data,
        }
    }
}

impl From<Shift> for Disass<u8> {
    fn from(opcode: Shift) -> Self {
        let name = Self::name(format!("{:?}", opcode));
        let (cycles, data): (u8, Data) = match opcode {
            Shift::LHL => (8, Data::Bits8(0)),
            Shift::RAHL => (8, Data::Bits8(0)),
            Shift::SHL => (8, Data::Bits8(0)),
            Shift::RLHL => (8, Data::Bits8(0)),
            _ => (4, Data::None),
        };

        let code: u8 = opcode.into();
        Self {
            name,
            opcode: code,
            cycles,
            data,
        }
    }
}

impl From<Arithmetic> for Disass<u8> {
    fn from(opcode: Arithmetic) -> Self {
        let name = Self::name(format!("{:?}", opcode));
        let (cycles, data): (u8, Data) = match opcode {
            Arithmetic::AAHL => (8, Data::Bits8(0)),
            Arithmetic::AA8b => (8, Data::Bits8(0)),
            Arithmetic::AAcHL => (8, Data::Bits8(0)),
            Arithmetic::AAc8b => (8, Data::Bits8(0)),
            Arithmetic::SAHL => (8, Data::Bits8(0)),
            Arithmetic::SAA => (8, Data::Bits8(0)),
            Arithmetic::SAcHL => (8, Data::Bits8(0)),
            Arithmetic::SA8b => (8, Data::Bits8(0)),
            _ => (4, Data::None),
        };

        let code: u8 = opcode.into();
        Self {
            name,
            opcode: code,
            cycles,
            data,
        }
    }
}
