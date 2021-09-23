use super::super::area::{Bits16, Bits8, Flag};
use super::super::pc::NextPc;
use crate::cpu::Registers;
use memory::Memory;
use num_enum::TryFromPrimitive;
use shared::{traits::Bus, Error};
use crate::cpu::flags::Flags;

/// 1. RLCA
/// Description:
///  Rotate A left. Old bit 7 to Carry flag.
/// Flags affected:
/// Z - Set if result is zero.
///  N - Reset.
///  H - Reset.
///  C - Contains old bit 7 data.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  RLCA -/- 07 4
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum RLCA {
    RLCA = 0x07,
}

impl<'a> RLCA {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let mut data = registers.get(Bits8::A);
            let bit7 = (data & 0x80) >> 7;
            //  todo!() registers.set(Bits8::F, bit7); // SET Carry flag withbit7
            data <<= 1 | bit7;
            dbg!("data {:?} b7 {:?}", data, bit7);
            registers.set(Bits8::A, data)
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

/// 2. RLA
/// Description:
///  Rotate A left through Carry flag.
/// Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Reset.
///  C - Contains old bit 7 data.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  RLA -/- 17 4
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum RLA {
    RLA = 0x07,
}

impl<'a> RLA {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let mut data = registers.get(Bits8::A);
            // let carry = registers.get(Bits8::F) & CFLAG;
            let bit7 = (data & 0x80) >> 7;
            //  todo!() registers.set(Bits8::F, bit7); // SET Carry flag withbit7
            data <<= 1 | carry;
            dbg!("data {:?} b7 {:?} carry {:?}", data, bit7, carry);
            registers.set(Bits8::A, data)
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

/// 3. RRCA
/// Description:
///  Rotate A right. Old bit 0 to Carry flag.
/// Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Reset.
///  C - Contains old bit 0 data.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  RRCA -/- 0F 4
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum RRCA {
    RRCA = 0x0f,
}

impl<'a> RRCA {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let mut data = registers.get(Bits8::A);
            let bit0 = (data & 0x01) << 7;
            //  todo!() registers.set(Bits8::F, bit0); // SET Carry flag with bit0
            data >>= 1 | bit0;
            dbg!("data {:?} b0 {:?}", data, bit0);
            registers.set(Bits8::A, data)
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

/// 4. RRA
/// Description:
///  Rotate A right through Carry flag.
/// Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Reset.
///  C - Contains old bit 0 data.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  RRA -/- 1F 4
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum RRA {
    RRA = 0x07,
}

impl<'a> RRA {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let mut data = registers.get(Bits8::A);
            // let carry = registers.get(Bits8::F) & CFLAG;
            let bit0 = (data & 0x01) << 7;
            //  todo!() registers.set(Bits8::F, bit0); // SET Carry flag withbit0
            data >>= 1 | carry;
            dbg!("data {:?} b0 {:?} carry {:?}", data, bit0, carry);
            registers.set(Bits8::A, data)
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

/// 5. RLC n
/// Description:
///  Rotate n left. Old bit 7 to Carry flag.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL)
/// Flags affected:
/// Z - Set if result is zero.
///  N - Reset.
///  H - Reset.
///  C - Contains old bit 7 data.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  RLC A CB07 8
///  RLC B CB00 8
///  RLC C CB01 8
///  RLC D CB02 8
///  RLC E CB03 8
///  RLC H CB04 8
///  RLC L CB05 8
///  RLC (HL) CB06 16
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum RLCN {
    RLCA = 0xcb07,
    RLCB = 0xcb00,
    RLCC = 0xcb01,
    RLCD = 0xcb02,
    RLCE = 0xcb03,
    RLCH = 0xcb04,
    RLCL = 0xcb05,
    // RLCHL = 0xcb06,
}

impl<'a> RLCN {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let mut bits = match self {
                RLCN::RLCA => Bits8::A,
                RLCN::RLCB => Bits8::B,
                RLCN::RLCC => Bits8::C,
                RLCN::RLCD => Bits8::D,
                RLCN::RLCE => Bits8::E,
                RLCN::RLCH => Bits8::H,
                RLCN::RLCL => Bits8::L,
                // RLCN::RLCHL => Bits16::HL,
            };
            let mut data = registers.get(bits);
            let bit7 = (data & 0x80) >> 7;
            // registers.set(Bits8::F, bit7);
            //  todo!() registers.set(Bits8::F, bit7); // SET Carry flag withbit7
            data <<= 1 | bit7;
            dbg!("data {:?} b7 {:?}", data, bit7);
            registers.set(bits, data)
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

/// 6. RL n
/// Description:
///  Rotate n left through Carry flag.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL)
/// Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Reset.
///  C - Contains old bit 7 data.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  RL A CB17 8
///  RL B CB10 8
///  RL C CB11 8
///  RL D CB12 8
///  RL E CB13 8
///  RL H CB14 8
///  RL L CB15 8
///  RL (HL) CB16 16
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum RLN {
    RLA = 0xcb17,
    RLB = 0xcb10,
    RLC = 0xcb11,
    RLD = 0xcb12,
    RLE = 0xcb13,
    RLH = 0xcb14,
    RLL = 0xcb15,
    // RLHL = 0xcb16,
}

impl<'a> RLN {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let mut bits = match self {
                RLN::RLA => Bits8::A,
                RLN::RLB => Bits8::B,
                RLN::RLC => Bits8::C,
                RLN::RLD => Bits8::D,
                RLN::RLE => Bits8::E,
                RLN::RLH => Bits8::H,
                RLN::RLL => Bits8::L,
                // RLN::RLHL => Bits16::HL,
            };
            let mut data = registers.get(bits);
            let bit7 = (data & 0x80) >> 7;
            // registers.set(Bits8::F, bit7);
            //  todo!() registers.set(Bits8::F, bit7); // SET Carry flag withbit7
            data <<= 1 | bit7; //CArry FLAG PAS LE BIT7
            dbg!("data {:?} b7 {:?}", data, bit7);
            registers.set(bits, data)
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

/// 7. RRC n
/// Description:
///  Rotate n right. Old bit 0 to Carry flag.
/// Use with:
/// n = A,B,C,D,E,H,L,(HL)
/// Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Reset.
///  C - Contains old bit 0 data.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  RRC A CB0F 8
///  RRC B CB08 8
///  RRC C CB09 8
///  RRC D CB0A 8
///  RRC E CB0B 8
///  RRC H CB0C 8
///  RRC L CB0D 8
///  RRC (HL) CB0E 16
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum RRCN {
    RRCA = 0xcb0f,
    RRCB = 0xcb08,
    RRCC = 0xcb09,
    RRCD = 0xcb0a,
    RRCE = 0xcb0b,
    RRCH = 0xcb0c,
    RRCL = 0xcb0d,
    // RRCHL = 0xcb0e,
}

impl<'a> RRCN {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let mut bits = match self {
                RRCN::RRCA => Bits8::A,
                RRCN::RRCB => Bits8::B,
                RRCN::RRCC => Bits8::C,
                RRCN::RRCD => Bits8::D,
                RRCN::RRCE => Bits8::E,
                RRCN::RRCH => Bits8::H,
                RRCN::RRCL => Bits8::L,
                // RLCN::RLCHL => Bits16::HL,
            };
            let mut data = registers.get(bits);
            let bit0 = (data & 0x01) << 7;
            // registers.set(Bits8::F, bit0);
            //  todo!() registers.set(Bits8::F, bit0); // SET Carry flag with bit0
            data >>= 1 | bit0;
            dbg!("data {:?} b7 {:?}", data, bit0);
            registers.set(bits, data)
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

/// 8. RR n
/// Description:
///  Rotate n right through Carry flag.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL)
/// Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Reset.
///  C - Contains old bit 0 data.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  RR A CB1F 8
///  RR B CB18 8
///  RR C CB19 8
///  RR D CB1A 8
///  RR E CB1B 8
///  RR H CB1C 8
///  RR L CB1D 8
///  RR (HL) CB 1E 16
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum RRN {
    RRA = 0xcb1f,
    RRB = 0xcb18,
    RRC = 0xcb19,
    RRD = 0xcb1a,
    RRE = 0xcb1b,
    RRH = 0xcb1c,
    RRL = 0xcb1d,
    // RRHL = 0xcb1e,
}

impl<'a> RRN {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let mut bits = match self {
                RRN::RRA => Bits8::A,
                RRN::RRB => Bits8::B,
                RRN::RRC => Bits8::C,
                RRN::RRD => Bits8::D,
                RRN::RRE => Bits8::E,
                RRN::RRH => Bits8::H,
                RRN::RRL => Bits8::L,
                // RLN::RLHL => Bits16::HL,
            };
            let mut data = registers.get(bits);
            let bit0 = (data & 0x01) << 7;
            // registers.set(Bits8::F, bit0);
            //  todo!() registers.set(Bits8::F, bit0); // SET Carry flag with bit0
            data >>= 1 | bit0; //CArry FLAG PAS LE BIT7
            dbg!("data {:?} b7 {:?}", data, bit0);
            registers.set(bits, data)
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

/// 9. SLA n
/// Description:
///  Shift n left into Carry. LSB of n set to 0.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL)
/// Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Reset.
///  C - Contains old bit 7 data.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  SLA A CB27 8
///  SLA B CB20 8
///  SLA C CB21 8
///  SLA D CB22 8
///  SLA E CB23 8
///  SLA H CB24 8
///  SLA L CB25 8
///  SLA (HL) CB 26 16
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum SLAN {
    SLAA = 0xcb27,
    SLAB = 0xcb20,
    SLAC = 0xcb21,
    SLAD = 0xcb22,
    SLAE = 0xcb23,
    SLAH = 0xcb24,
    SLAL = 0xcb25,
    // SLAHL = 0xcb26,
}

impl<'a> SLAN {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let mut bits = match self {
                SLAN::SLAA => Bits8::A,
                SLAN::SLAB => Bits8::B,
                SLAN::SLAC => Bits8::C,
                SLAN::SLAD => Bits8::D,
                SLAN::SLAE => Bits8::E,
                SLAN::SLAH => Bits8::H,
                SLAN::SLAL => Bits8::L,
                // SLAN::SLAHL => Bits16::HL,
            };
            let mut data = registers.get(bits);
            let bit7 = (data & 0x80) >> 7;
            // registers.set(Bits8::F, bit7);
            //  todo!() registers.set(Bits8::F, bit7); // SET Carry flag withbit7
            data <<= 1 | bit7;
            dbg!("data {:?} b7 {:?}", data, bit7);
            registers.set(bits, data)
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

/// 10. SRA n
/// Description:
///  Shift n right into Carry. MSB doesn't change.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL)
/// Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Reset.
///  C - Contains old bit 0 data.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  SRA A CB2F 8
///  SRA B CB28 8
///  SRA C CB29 8
///  SRA D CB2A 8
///  SRA E CB2B 8
///  SRA H CB2C 8
///  SRA L CB2D 8
///  SRA (HL) CB2E 16
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum SRAN {
    SRAA = 0xcb2f,
    SRAB = 0xcb28,
    SRAC = 0xcb29,
    SRAD = 0xcb2a,
    SRAE = 0xcb2b,
    SRAH = 0xcb2c,
    SRAL = 0xcb2d,
    // SRAHL = 0xcb2e,
}

impl<'a> SRAN {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let mut bits = match self {
                SRAN::SRAA => Bits8::A,
                SRAN::SRAB => Bits8::B,
                SRAN::SRAC => Bits8::C,
                SRAN::SRAD => Bits8::D,
                SRAN::SRAE => Bits8::E,
                SRAN::SRAH => Bits8::H,
                SRAN::SRAL => Bits8::L,
                // RLCN::RLCHL => Bits16::HL,
            };
            let mut data = registers.get(bits);
            let bit0 = (data & 0x01) << 7;
            // registers.set(Bits8::F, bit0);
            //  todo!() registers.set(Bits8::F, bit0); // SET Carry flag withbit0
            data >>= 1 | bit0;
            dbg!("data {:?} b7 {:?}", data, bit0);
            registers.set(bits, data)
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}

/// 11. SRL n
/// Description:
///  Shift n right into Carry. MSB set to 0.
/// Use with:
///  n = A,B,C,D,E,H,L,(HL)
/// Flags affected:
///  Z - Set if result is zero.
///  N - Reset.
///  H - Reset.
///  C - Contains old bit 0 data.
/// Opcodes:
/// Instruction Parameters Opcode Cycles
///  SRL A CB3F 8
///  SRL B CB38 8
///  SRL C CB39 8
///  SRL D CB3A 8
///  SRL E CB3B 8
///  SRL H CB3C 8
///  SRL L CB3D 8
///  SRL (HL) CB3E 16
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum SRLN {
    SRLA = 0xcb3f,
    SRLB = 0xcb38,
    SRLC = 0xcb39,
    SRLD = 0xcb3a,
    SRLE = 0xcb3b,
    SRLH = 0xcb3c,
    SRLL = 0xcb3d,
    // SRLHL = 0xcb3e,
}

impl<'a> SRLN {
    pub fn proceed(
        self,
        registers: &'a mut Registers,
        memory: &'a mut Memory,
    ) -> Result<u32, Error> {
        if let Ok(byte) = registers.pc.next(memory) {
            let mut bits = match self {
                SRLN::SRLA => Bits8::A,
                SRLN::SRLB => Bits8::B,
                SRLN::SRLC => Bits8::C,
                SRLN::SRLD => Bits8::D,
                SRLN::SRLE => Bits8::E,
                SRLN::SRLH => Bits8::H,
                SRLN::SRLL => Bits8::L,
                // RLCN::RLCHL => Bits16::HL,
            };
            let mut data = registers.get(bits);
            let bit0 = (data & 0x01) << 7;
            // registers.set(Bits8::F, bit0);
            //  todo!() registers.set(Bits8::F, bit0); // SET Carry flag withbit0
            data >>= 1 | bit0;
            dbg!("data {:?} b7 {:?}", data, bit0);
            registers.set(bits, data)
        } else {
            Err(Error::InvalidPC(registers.pc))
        }
    }
}
