use super::{Bits8, Flag};
use super::{Bus, Registers};

/// Shift arithmetic and logic are right shifts.

pub trait Shift<T> {
    fn shift_left(&mut self, data: T) -> u8;
    fn shift_arithmetic(&mut self, data: T) -> u8;
    fn shift_logic(&mut self, data: T) -> u8;
    fn swap(&mut self, data: T) -> u8;
}

impl Shift<Bits8> for Registers {
    fn shift_left(&mut self, dst: Bits8) -> u8 {
        let data = self.get(dst);
        let result = data << 1;
        let bit7 = data >> 7;
        self.set(Flag::C, bit7 == 1);
        self.set(Flag::H, false);
        self.set(Flag::N, false);
        self.set(Flag::Z, result == 0);
        self.set(dst, result);
        0
    }

    fn shift_arithmetic(&mut self, dst: Bits8) -> u8 {
        let data = self.get(dst);
        let result = data >> 1 | (data & 0x80);
        self.set(Flag::C, (data & 1) == 1);
        self.set(Flag::H, false);
        self.set(Flag::N, false);
        self.set(Flag::Z, result == 0);
        self.set(dst, result);
        0
    }

    fn shift_logic(&mut self, dst: Bits8) -> u8 {
        let data = self.get(dst);
        let result = data >> 1;
        self.set(Flag::C, (data & 1) == 1);
        self.set(Flag::H, false);
        self.set(Flag::N, false);
        self.set(Flag::Z, result == 0);
        self.set(dst, result);
        0
    }

    fn swap(&mut self, dst: Bits8) -> u8 {
        let data = self.get(dst);
        let upper = data >> 4;
        let lower = data << 4;
        let data = lower | upper;
        self.set(Flag::Z, data == 0);
        self.set(Flag::H, false);
        self.set(Flag::N, false);
        self.set(Flag::C, false);
        self.set(dst, data);
        0
    }
}

impl Shift<u8> for Registers {
    fn shift_left(&mut self, data: u8) -> u8 {
        let result = data << 1;
        let bit7 = data >> 7;
        self.set(Flag::C, bit7 == 1);
        self.set(Flag::H, false);
        self.set(Flag::N, false);
        self.set(Flag::Z, result == 0);
        result
    }

    fn shift_arithmetic(&mut self, data: u8) -> u8 {
        let result = data >> 1 | (data & 0x80);
        self.set(Flag::C, (data & 1) == 1);
        self.set(Flag::H, false);
        self.set(Flag::N, false);
        self.set(Flag::Z, result == 0);
        result
    }

    fn shift_logic(&mut self, data: u8) -> u8 {
        let result = data >> 1;
        self.set(Flag::C, (data & 1) == 1);
        self.set(Flag::H, false);
        self.set(Flag::N, false);
        self.set(Flag::Z, result == 0);
        result
    }

    fn swap(&mut self, data: u8) -> u8 {
        let upper = data >> 4;
        let lower = data << 4;
        let data = lower | upper;
        self.set(Flag::Z, data == 0);
        self.set(Flag::H, false);
        self.set(Flag::N, false);
        self.set(Flag::C, false);
        data
    }
}
