#![allow(dead_code, unused_attributes, unused_imports)]
use super::{Bus, Flag};
use modular_bitfield::{bitfield, specifiers::B4};

#[bitfield]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Flags {
    pub(crate) z: bool,
    pub(crate) n: bool,
    pub(crate) h: bool,
    pub(crate) c: bool,
    #[skip]
    _unused: B4,
}

impl Default for Flags {
    fn default() -> Self {
        Flags::new()
    }
}

impl Flags {
    pub fn is_carried(&self, use_carry: bool) -> u8 {
        if use_carry && self.c() {
            1
        } else {
            0
        }
    }

    pub fn get_all(&self) -> u8 {
        ((self.z() as u8) << 7)
            | ((self.n() as u8) << 6)
            | ((self.h() as u8) << 5)
            | ((self.c() as u8) << 4)
    }

    pub fn set_all(&mut self, data: u8) {
        self.set_z((data & 0b1000_0000) != 0);
        self.set_n((data & 0b0100_0000) != 0);
        self.set_h((data & 0b0010_0000) != 0);
        self.set_c((data & 0b0001_0000) != 0);
    }

    pub fn complement_c(&mut self) {
        self.set_c(!self.c());
    }
}

pub trait Carry<T> {
    fn checked_add(&mut self, a: T, data: T, carry: T) -> T;
    fn checked_sub(&mut self, a: T, data: T, carry: T) -> T;
}

impl Carry<u8> for Flags {
    fn checked_add(&mut self, a: u8, data: u8, carry: u8) -> u8 {
        let add = a.wrapping_add(data).wrapping_add(carry);

        self.set_c((a as u16) + (data as u16) + (carry as u16) > 0xFF);
        self.set_h((a & 0xF) + (data & 0xF) + carry > 0xF);
        self.set_n(false);
        self.set_z(add == 0);
        add
    }

    fn checked_sub(&mut self, a: u8, data: u8, carry: u8) -> u8 {
        let sub = a.wrapping_sub(data).wrapping_sub(carry);

        self.set_h((a & 0x0F) < (data & 0x0F) + carry);
        self.set_c((a as u16) < (data as u16) + (carry as u16));
        self.set_n(true);
        self.set_z(sub == 0);
        sub
    }
}

impl Carry<u16> for Flags {
    fn checked_add(&mut self, hl: u16, data: u16, _carry: u16) -> u16 {
        let add = hl.wrapping_add(data);
        self.set_h((hl & 0x07FF) + (data & 0x07FF) > 0x07FF);
        self.set_c(hl > 0xFFFF - data);
        add
    }

    fn checked_sub(&mut self, _a: u16, _data: u16, _carry: u16) -> u16 {
        0
    }
}

impl Bus<Flag, bool> for Flags {
    fn get(&self, flag: Flag) -> bool {
        match flag {
            Flag::Z => self.z(),
            Flag::N => self.n(),
            Flag::H => self.h(),
            Flag::C => self.c(),
        }
    }

    fn set(&mut self, flag: Flag, data: bool) {
        match flag {
            Flag::Z => self.set_z(data),
            Flag::N => self.set_n(data),
            Flag::H => self.set_h(data),
            Flag::C => self.set_c(data),
        }
    }
}

#[cfg(test)]
mod test_flags {
    use super::{Bus, Flag, Flags};

    #[test]
    fn test_valid_flag_set_get() {
        let mut flags: Flags = Flags::new();

        flags.set(Flag::Z, true);
        let value = flags.get(Flag::Z);
        assert!(value);
    }
}
