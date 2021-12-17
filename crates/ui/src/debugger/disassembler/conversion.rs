use super::disass::{Data, Disass};
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
use shared::Error;

impl From<Arithmetic16b> for Disass<u8> {
    fn from(opcode: Arithmetic16b) -> Self {
        let name = Self::name(format!("{}", opcode));
        let (cycles, data): (u8, Data) = match opcode {
            Arithmetic16b::IncBC => (8, Data::None),
            Arithmetic16b::IncDE => (8, Data::None),
            Arithmetic16b::IncHL => (8, Data::None),
            Arithmetic16b::IncSP => (8, Data::None),
            Arithmetic16b::DecBC => (8, Data::None),
            Arithmetic16b::DecDE => (8, Data::None),
            Arithmetic16b::DecHL => (8, Data::None),
            Arithmetic16b::DecSP => (8, Data::None),
            Arithmetic16b::AddBC => (8, Data::None),
            Arithmetic16b::AddDE => (8, Data::None),
            Arithmetic16b::AddHL => (8, Data::None),
            Arithmetic16b::AddSP => (8, Data::None),
            Arithmetic16b::AddSPr8 => (16, Data::Bits8(0)),
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

impl From<Control> for Disass<u8> {
    fn from(opcode: Control) -> Self {
        let name = Self::name(format!("{}", opcode));
        let (cycles, data): (u8, Data) = match opcode {
            Control::NOP => (4, Data::None),
            Control::STOP => (4, Data::None),
            Control::HALT => (4, Data::None),
            Control::DI => (4, Data::None),
            Control::EI => (4, Data::None),
            Control::CB => unreachable!(),
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

impl From<Bitset> for Disass<u8> {
    fn from(opcode: Bitset) -> Self {
        let name = Self::name(format!("(CB) {}", opcode));
        let (cycles, data): (u8, Data) = match opcode {
            Bitset::HLBit0 => (12, Data::Cb),
            Bitset::HLBit1 => (12, Data::Cb),
            Bitset::HLBit2 => (12, Data::Cb),
            Bitset::HLBit3 => (12, Data::Cb),
            Bitset::HLBit4 => (12, Data::Cb),
            Bitset::HLBit5 => (12, Data::Cb),
            Bitset::HLBit6 => (12, Data::Cb),
            Bitset::HLBit7 => (12, Data::Cb),
            _ => (8, Data::Cb),
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
        let name = Self::name(format!("(CB) {}", opcode));
        let (cycles, data): (u8, Data) = match opcode {
            Reset::HLBit0 => (12, Data::Cb),
            Reset::HLBit1 => (12, Data::Cb),
            Reset::HLBit2 => (12, Data::Cb),
            Reset::HLBit3 => (12, Data::Cb),
            Reset::HLBit4 => (12, Data::Cb),
            Reset::HLBit5 => (12, Data::Cb),
            Reset::HLBit6 => (12, Data::Cb),
            Reset::HLBit7 => (12, Data::Cb),
            _ => (8, Data::Cb),
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
        let name = Self::name(format!("(CB) {}", opcode));
        let (cycles, data): (u8, Data) = match opcode {
            Test::HLBit0 => (12, Data::Cb),
            Test::HLBit1 => (12, Data::Cb),
            Test::HLBit2 => (12, Data::Cb),
            Test::HLBit3 => (12, Data::Cb),
            Test::HLBit4 => (12, Data::Cb),
            Test::HLBit5 => (12, Data::Cb),
            Test::HLBit6 => (12, Data::Cb),
            Test::HLBit7 => (12, Data::Cb),
            _ => (8, Data::Cb),
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
        let name = Self::name(format!("{}", opcode));
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
            _ => Err(Error::Unimplemented(0x42)),
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
        let name = Self::name(format!("{}", opcode));
        let (cycles, data): (u8, Data) = match opcode {
            Jump::NN => Ok((16, Data::Bits16(0))),
            Jump::HL => Ok((4, Data::None)),
            Jump::R8b => Ok((12, Data::Bits8(0))),
            Jump::Call => Ok((24, Data::Bits16(0))),
            Jump::Return => Ok((16, Data::None)),
            Jump::Reset00 => Ok((16, Data::None)),
            Jump::Reset10 => Ok((16, Data::None)),
            Jump::Reset20 => Ok((16, Data::None)),
            Jump::Reset30 => Ok((16, Data::None)),
            Jump::Reset08 => Ok((16, Data::None)),
            Jump::Reset18 => Ok((16, Data::None)),
            Jump::Reset28 => Ok((16, Data::None)),
            Jump::Reset38 => Ok((16, Data::None)),
            _ => Err(Error::Unimplemented(0x42)),
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
        let name = Self::name(format!("{}", opcode));
        let (cycles, data): (u8, Data) = match opcode {
            Logic::AndAHL => (8, Data::None),
            Logic::AndA8b => (8, Data::Bits8(0)),
            Logic::OrAHL => (8, Data::None),
            Logic::OrA8b => (8, Data::Bits8(0)),
            Logic::XorAHL => (8, Data::None),
            Logic::XorA8b => (8, Data::Bits8(0)),
            Logic::CmpAHL => (8, Data::None),
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
        let name = Self::name(format!("(CB) {}", opcode));
        let (cycles, data): (u8, Data) = match opcode {
            Rotate::LCHL => (16, Data::Cb),
            Rotate::LHL => (16, Data::Cb),
            Rotate::RCHL => (16, Data::Cb),
            Rotate::RHL => (16, Data::Cb),
            _ => (8, Data::Cb),
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
        let name = Self::name(format!("{}", opcode));
        let (cycles, data): (u8, Data) = match opcode {
            Load::HL8b => (12, Data::Bits8(0)),
            Load::B8b => (8, Data::Bits8(0)),
            Load::C8b => (8, Data::Bits8(0)),
            Load::D8b => (8, Data::Bits8(0)),
            Load::E8b => (8, Data::Bits8(0)),
            Load::H8b => (8, Data::Bits8(0)),
            Load::L8b => (8, Data::Bits8(0)),
            Load::A8b => (8, Data::Bits8(0)),
            Load::BHL => (8, Data::None),
            Load::CHL => (8, Data::None),
            Load::DHL => (8, Data::None),
            Load::EHL => (8, Data::None),
            Load::HHL => (8, Data::None),
            Load::LHL => (8, Data::None),
            Load::AHL => (8, Data::None),
            Load::HLB => (8, Data::None),
            Load::HLC => (8, Data::None),
            Load::HLD => (8, Data::None),
            Load::HLE => (8, Data::None),
            Load::HLH => (8, Data::None),
            Load::HLL => (8, Data::None),
            Load::HLA => (8, Data::None),
            Load::BCA => (8, Data::None),
            Load::DEA => (8, Data::None),
            Load::ABC => (8, Data::None),
            Load::ADE => (8, Data::None),
            Load::HLPA => (8, Data::None),
            Load::HLMA => (8, Data::None),
            Load::AHLP => (8, Data::None),
            Load::AHLM => (8, Data::None),
            Load::ToIOC => (8, Data::None),
            Load::IOC => (8, Data::None),
            Load::ToIONext => (12, Data::Bits8(0)),
            Load::IONext => (12, Data::Bits8(0)),
            Load::AtNextA => (16, Data::Bits16(0)),
            Load::AAtNext => (16, Data::Bits16(0)),
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
        let name = Self::name(format!("{}", opcode));
        let (cycles, data): (u8, Data) = match opcode {
            Load16b::PushAF => (16, Data::None),
            Load16b::PushBC => (16, Data::None),
            Load16b::PushDE => (16, Data::None),
            Load16b::PushHL => (16, Data::None),
            Load16b::LoadBC => (12, Data::Bits16(0)),
            Load16b::LoadDE => (12, Data::Bits16(0)),
            Load16b::LoadHL => (12, Data::Bits16(0)),
            Load16b::LoadSP => (12, Data::Bits16(0)),
            Load16b::LoadA16SP => (16, Data::Bits16(0)),
            Load16b::LoadSPHL => (8, Data::Bits16(0)),
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
        let name = Self::name(format!("(CB) {}", opcode));
        let (cycles, data): (u8, Data) = match opcode {
            Shift::LHL => (16, Data::Cb),
            Shift::RAHL => (16, Data::Cb),
            Shift::SHL => (16, Data::Cb),
            Shift::RLHL => (16, Data::Cb),
            _ => (8, Data::Cb),
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
        let name = Self::name(format!("{}", opcode));
        let (cycles, data): (u8, Data) = match opcode {
            Arithmetic::AAHL => (8, Data::None),
            Arithmetic::AA8b => (8, Data::Bits8(0)),
            Arithmetic::AAcHL => (8, Data::None),
            Arithmetic::AAc8b => (8, Data::Bits8(0)),
            Arithmetic::SAHL => (8, Data::None),
            Arithmetic::SAA => (8, Data::None),
            Arithmetic::SAcHL => (8, Data::None),
            Arithmetic::SA8b => (8, Data::Bits8(0)),
            Arithmetic::IncHL => (12, Data::None),
            Arithmetic::DecHL => (12, Data::None),
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
