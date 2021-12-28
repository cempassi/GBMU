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
        println!("Interrupts get >> interrupt: {:#b}", self.data);
        Ok(self.data)
    }

    pub fn check(&self, requested: u8) -> u8 {
        self.data & requested
    }

    pub fn set(&mut self, data: u8) -> Result<(), Error> {
        self.data = data;
        println!(
            "Interrupts after set >> data: {:#b}, stored: {:#b}",
            data, self.data
        );
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
    VBlank = 0b00000001,
    Lcd = 0b00000010,
    Timer = 0b00000100,
    Serial = 0b00001000,
    Joypad = 0b00010000,
}
