use crate::area::Bits16;
use crate::cpu::Registers;
use crate::{Cpu, RegisterBus};
use memory::{Async, Memory};
use num_enum::TryFromPrimitive;

/// [2. JP cc,nn]                                        | [JP nn]                              | [JP (HL)]
/// [Description:]                                       | [Description:]                       | [Description:]
///  [Jump to address n if following condition is true:] | [Jump to address nn.]                |  [Jump to address contained in HL.]
///  cc = NZ, Jump if Z flag is reset.
///  cc = Z, Jump if Z flag is set.
///  cc = NC, Jump if C flag is reset.
///  cc = C, Jump if C flag is set.
/// [Use with:]                                          [Use with:]
///  [nn = two byte immediate value. (LS byte first.)]  |  [nn = two byte immediate value.]     |
/// [Opcodes:]                                          | [Opcodes:]                            | [Opcodes:]
/// Instruction Parameters Opcode Cycles                | Instruction Parameters Opcode Cycles  | Instruction Parameters Opcode Cycles
/// JP          NZ,nn      0xC2  12                       JP          nn         0xC3   12        JP         (HL)        0xE9   4
/// JP          Z,nn       0xCA  12
/// JP          NC,nn      0xD2  12
/// JP          C,nn       0xDA  12
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum Jump {
    JPNZ = 0xc2,
    JPZ = 0xca,
    JPNC = 0xd2,
    JPC = 0xda,
    JPNN = 0xc3,
    JPHL = 0xe9,
}

impl Jump {
    pub async fn exec(self, registers: Registers, memory: Memory) {
        let src = match self {
            Jump::JPNZ | Jump::JPZ | Jump::JPNC | Jump::JPC => {
                if !Cpu::flags_conditions(self as u8, registers.clone()) {
                    return;
                }
                Bits16::PC
            }
            Jump::JPNN => Bits16::PC,
            Jump::JPHL => Bits16::HL,
        };
        let src = registers.borrow().get(src);
        let data = <Memory as Async<u16>>::get(memory, src).await.unwrap();
        Cpu::jump(registers.clone(), data);
    }
}
