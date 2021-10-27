use crate::registers::lcd;
use crate::registers::Registers;

#[derive(Debug)]
pub struct Ppu {
    vram: Vec<u8>,
    registers: Registers,
}

impl AsRef<Vec<u8>> for Ppu {
    fn as_ref(&self) -> &Vec<u8> {
        self.vram.as_ref()
    }
}

impl AsRef<Registers> for Ppu {
    fn as_ref(&self) -> &Registers {
        &self.registers
    }
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}

impl Ppu {
    pub fn new() -> Self {
        let vram = vec![0; 8192];
        let registers = Registers::default();
        Self { vram, registers }
    }

    pub fn get_vram(&self, address: usize) -> u8 {
        self.vram[address]
    }

    pub fn get_lcd(&self) -> &lcd::Lcd {
        &self.registers.lcd
    }

    pub fn set_vram(&mut self, address: usize, data: u8) {
        self.vram[address] = data;
    }

    pub fn is_lower(&mut self, register: lcd::Field, nbr: u8) -> bool {
        self.registers.lcd.is_lower(register, nbr)
    }

    pub fn increase(&mut self, register: lcd::Field) {
        self.registers.lcd.increase(register);
    }

    pub fn clear(&mut self, register: lcd::Field) {
        self.registers.lcd.clear(register);
    }
}
