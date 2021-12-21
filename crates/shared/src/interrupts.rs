#![allow(dead_code, unused_attributes, unused_imports)]
use crate::Error;
use modular_bitfield::{bitfield, specifiers::B3};
use num_enum::{IntoPrimitive, TryFromPrimitive};

pub mod interface {
    use std::cell::RefCell;
    use std::rc::Rc;
    pub type Interrupts = Rc<RefCell<super::Interrupts>>;
}

#[bitfield]
#[derive(Debug, Default, Clone, Copy)]
pub struct Interrupts {
    pub vblank: bool,
    pub lcd: bool,
    pub timer: bool,
    pub serial: bool,
    pub joypad: bool,
    #[skip]
    _unused: B3,
}

impl Interrupts {
    pub fn get(&self) -> Result<u8, Error> {
        Ok(self.into_bytes()[0])
    }

    pub fn check(&self, requested: u8) -> u8 {
        self.into_bytes()[0] & requested
    }

    pub fn set(&self, data: u8) -> Result<(), Error> {
        self.into_bytes()[0] = data;
        Ok(())
    }

    pub fn status(&self, interrupt: Interrupt) -> bool {
        match interrupt {
            Interrupt::Vblank => self.vblank(),
            Interrupt::Lcd => self.lcd(),
            Interrupt::Timer => self.timer(),
            Interrupt::Serial => self.serial(),
            Interrupt::Joypad => self.joypad(),
        }
    }

    pub fn processed(&mut self, interrupt: Interrupt) {
        match interrupt {
            Interrupt::Vblank => self.set_vblank(false),
            Interrupt::Lcd => self.set_lcd(false),
            Interrupt::Timer => self.set_timer(false),
            Interrupt::Serial => self.set_serial(false),
            Interrupt::Joypad => self.set_joypad(false),
        }
    }

    pub fn request(&mut self, interrupt: Interrupt) {
        match interrupt {
            Interrupt::Vblank => self.set_vblank(true),
            Interrupt::Lcd => self.set_lcd(true),
            Interrupt::Timer => self.set_timer(true),
            Interrupt::Serial => self.set_serial(true),
            Interrupt::Joypad => self.set_joypad(true),
        }
    }
}

#[derive(Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum Interrupt {
    Vblank = 0,
    Lcd = 1,
    Timer = 2,
    Serial = 3,
    Joypad = 4,
}
