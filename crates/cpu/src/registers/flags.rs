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

    pub fn add_h_check(&self, data: u8) -> u8 {
        if self.h() || (data & 0xf) > 9 {
            6
        } else {
            0
        }
    }

    pub fn add_c_check(&mut self, data: u8) -> u8 {
        if self.c() || data > 0x99 {
            self.set_c(true);
            6
        } else {
            0
        }
    }

    pub fn sub_h_check(&self) -> u8 {
        if self.h() {
            6
        } else {
            0
        }
    }

    pub fn sub_c_check(&mut self) -> u8 {
        if self.c() {
            6
        } else {
            0
        }
    }

    pub fn complement_c(&mut self) {
        self.set_c(!self.c());
    }
}

pub trait Carry<T> {
    fn is_half_carry(&mut self, a: T, b: T);
    fn is_half_borrow(&mut self, a: T, b: T);
    fn checked_add(&mut self, a: T, b: T) -> T;
    fn checked_sub(&mut self, a: T, b: T) -> T;
}

impl Carry<u8> for Flags {
    fn is_half_carry(&mut self, a: u8, b: u8) {
        let a = u16::from(a);
        let b = u16::from(b);
        let sum = a + b;
        let check_half = (a ^ b ^ sum) & 0x10 == 0x10;
        self.set_h(check_half);
    }

    fn is_half_borrow(&mut self, a: u8, b: u8) {
        let a = i16::from(a);
        let b = i16::from(b);
        let sub = a - b;
        let check_half = (a ^ (-b) ^ sub) & 0x10 == 0x10;
        self.set_h(check_half);
    }

    fn checked_add(&mut self, a: u8, b: u8) -> u8 {
        let (data, check) = a.overflowing_add(b);
        self.is_half_carry(a, b);
        self.set_c(check);
        self.set_z(data == 0);
        data
    }

    fn checked_sub(&mut self, a: u8, b: u8) -> u8 {
        let (data, check) = a.overflowing_sub(b);
        self.is_half_borrow(a, b);
        self.set_c(check);
        self.set_z(data == 0);
        data
    }
}

impl Carry<u16> for Flags {
    fn is_half_carry(&mut self, a: u16, b: u16) {
        let sum = a + b;
        let check_half = (a ^ b ^ sum) & 0x1000 == 0x1000;
        self.set_h(check_half);
    }

    fn is_half_borrow(&mut self, a: u16, b: u16) {
        let a = i32::from(a);
        let b = i32::from(b);
        let sub = a - b;
        let check_half = (a ^ (-b) ^ sub) & 0x1000 == 0x1000;
        self.set_h(check_half);
    }

    fn checked_add(&mut self, a: u16, b: u16) -> u16 {
        let (data, check) = a.overflowing_add(b);
        self.is_half_carry(a, b);
        self.set_c(check);
        self.set_z(data == 0);
        data
    }

    fn checked_sub(&mut self, a: u16, b: u16) -> u16 {
        let (data, check) = a.overflowing_sub(b);
        self.is_half_borrow(a, b);
        self.set_c(check);
        self.set_z(data == 0);
        data
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
