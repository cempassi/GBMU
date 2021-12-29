#![allow(dead_code, unused_attributes, unused_imports)]
use crate::Error;
use modular_bitfield::{bitfield, specifiers::B3};
use num_enum::{IntoPrimitive, TryFromPrimitive};

pub mod interface {
    use std::cell::RefCell;
    use std::rc::Rc;
    pub type Interrupts = Rc<RefCell<super::Interrupts>>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Interrupts {
    data: u8,
}

impl Interrupts {
    pub fn get(&self) -> Result<u8, Error> {
        Ok(self.data)
    }

    pub fn check(&self, requested: u8) -> u8 {
        self.data & requested
    }

    pub fn set(&mut self, data: u8) -> Result<(), Error> {
        self.data = data;
        Ok(())
    }

    pub fn status(&self, interrupt: Interrupt) -> bool {
        self.data & interrupt as u8 != 0
    }

    pub fn processed(&mut self, interrupt: Interrupt) {
        self.data &= !(interrupt as u8);
    }

    pub fn request(&mut self, interrupt: Interrupt) {
        self.data |= interrupt as u8;
    }
}

#[derive(Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum Interrupt {
    VBlank = 0b0000_0001,
    Lcd = 0b0000_0010,
    Timer = 0b0000_0100,
    Serial = 0b0000_1000,
    Joypad = 0b0001_0000,
}
