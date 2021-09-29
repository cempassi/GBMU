/// LD          (HL),n     0x36   12
pub enum HL8bits {
    LDHLN = 0x36,
}

impl HL8bits {
    pub fn exec(self, registers: Registers, memory: Memory) {
        let data = registers.borrow_mut().pc.next(memory.clone()).unwrap();
        match self {
           HL8bits::LDHLN => { memory
                .borrow_mut()
                .set(registers.borrow().get(Bits16::HL), data)
                .unwrap()
           }
        }
    }
}


#[cfg(test)]
mod test_instruction_load_8bit_into_reg {
    use super::HL8bits;
    use crate::area::Bits16;
    use crate::{RegisterBus, Registers};
    use memory::Memory;

    #[test]
    fn test_load_hlb() {
        let register = Registers::default();
        let memory = Memory::default();
        let ldhl8b = HL8bits::LDHLN;
        let byte = memory.borrow().get(register.borrow().pc).unwrap();
        assert_eq!(byte, 0x31);
        ldhl8b.exec(register.clone(), memory.clone());
        assert_eq!(byte, memory.borrow().get(register.borrow().get(Bits16::HL)));
    }
}