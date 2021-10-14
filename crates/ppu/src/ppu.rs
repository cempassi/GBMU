use crate::registers::lcd::Register;
use crate::registers::Registers;

#[derive(Debug, Default)]
pub struct Ppu {
    vram: Vec<u8>,
    registers: Registers,
}

impl AsRef<Vec<u8>> for Ppu {
    fn as_ref(&self) -> &Vec<u8> {
        self.vram.as_ref()
    }
}

impl Ppu {
    pub fn get_vram(&self, address: usize) -> u8 {
        self.vram[address]
    }

    pub fn set_vram(&mut self, address: usize, data: u8) {
        self.vram[address] = data;
    }

    pub fn is_lower(&mut self, register: Register, nbr: u8) -> bool {
        self.registers.is_lower(register, nbr)
    }

    pub fn increase(&mut self, register: Register) {
        self.registers.increase(register);
    }

    pub fn clear(&mut self, register: Register) {
        self.registers.clear(register);
    }
}
