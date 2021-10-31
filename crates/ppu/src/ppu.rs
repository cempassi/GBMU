use crate::registers::lcd;
use crate::registers::Registers;
use shared::Interrupts;
use shared::{Error, Interrupt};

#[derive(Debug)]
pub struct Ppu {
    vram: Vec<u8>,
    registers: Registers,
    interrupts: Interrupts,
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

impl From<Interrupts> for Ppu {
    fn from(interrupts: Interrupts) -> Self {
        let vram = vec![0; 8192];
        let registers = Registers::default();
        Self {
            vram,
            registers,
            interrupts,
        }
    }
}

impl Ppu {
    pub fn get_vram(&self, address: usize) -> Result<u8, Error> {
        Ok(self.vram[address])
    }

    pub fn set_vram(&mut self, address: usize, data: u8) -> Result<(), Error> {
        self.vram[address] = data;
        Ok(())
    }

    pub fn get_registers(&self, address: u16) -> Result<u8, Error> {
        Ok(self.registers.get(address))
    }

    pub fn set_registers(&mut self, address: u16, data: u8) -> Result<(), Error> {
        self.registers.set(address, data);
        Ok(())
    }

    pub fn get_lcd(&self) -> &lcd::Lcd {
        &self.registers.lcd
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

    pub fn raise_vblank(&self) {
        self.interrupts.borrow_mut().request(Interrupt::Lcd);
    }
}
