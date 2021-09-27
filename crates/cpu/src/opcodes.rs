mod loadreg1reg2;
mod loadregnum8bit;

pub use loadregnum8bit::LoadRegNum8bit;
pub use loadreg1reg2::LoadReg1Reg2;

//
// type Address = u16;
// type Byte = u8;
// type Word = u16; ///Little endian
//
// pub enum Instructions {
//     // ADD(Address),
//     // BIT(Byte, Bits8),
//     // CALL(Address),
//     // CP(byte),
//     // CPHL,
//     // DEC(Bits8),
//     // INCReg(Bits8),
//     // INCb16(Bits16),
//     // JR(u8, Address),
//     // LDA(Address),
//     // LDb16W(Bits16, Word),
//     // LDAddressA(Address),
//     // LDAddressReg(Address, Bits8),
//     LDRegReg(Bits8, Bits8),
//     // LDHLpA,
//     LD(Bits8,Address),
//     // LDRegByte(Bits8, Byte),
//     // LDHLmA,
//     // POP(Bits16),
//     // PUSH(Bits16),
//     // RET,
//     // RL(Bits8),
//     // RLA,
//     // SUB(Bits8),
//     // XOR(Bits8),
//     Error(Byte),
// }
//
//
// impl Instructions {
//     pub fn exec(self, mut registers: Registers) {
//         match self {
//             Instructions::LDRegReg(r1, r2) => LoadReg1Reg2::exec(r1, r2),
//             Instructions::LD(bits, address) => LoadRegNum8bit::exec()
//             _ => (),
//         }
//     }
// }
