use super::{Bits8, Flag};
use super::{Bus, Registers};

const BIT7: u8 = 0x80;
const BIT0: u8 = 0x1;

const LOWER_HALF: u8 = 0xF;
const UPPER_HALF: u8 = 0xF0;

/// Shift arithmetic and logic are right shifts.

pub trait Shift<T> {
    fn shift_left(&mut self, data: T) -> u8;
    fn shift_arithmetic(&mut self, data: T) -> u8;
    fn shift_logic(&mut self, data: T) -> u8;
    fn swap(&mut self, data: T) -> u8;
}

impl Shift<Bits8> for Registers {
    fn shift_left(&mut self, dst: Bits8) -> u8 {
        let mut data = self.get(dst);
        let carry = (data & BIT7) != 0;
        data <<= 1;
        self.set(Flag::C, carry);
        self.set(Flag::H, false);
        self.set(Flag::N, false);
        self.set(Flag::Z, data == 0);
        self.set(dst, data);
        0
    }

    fn shift_arithmetic(&mut self, dst: Bits8) -> u8 {
        let mut data = self.get(dst);
        let carry = (data & BIT0) != 0;
        let sign = data & BIT7;
        data &= !(BIT7);
        data >>= 1;
        data |= sign;
        self.set(Flag::C, carry);
        self.set(Flag::H, false);
        self.set(Flag::N, false);
        self.set(Flag::Z, data == 0);
        self.set(dst, data);
        0
    }

    fn shift_logic(&mut self, dst: Bits8) -> u8 {
        let mut data = self.get(dst);
        let carry = (data & BIT0) != 0;
        data >>= 1;
        self.set(Flag::C, carry);
        self.set(Flag::H, false);
        self.set(Flag::N, false);
        self.set(Flag::Z, data == 0);
        self.set(dst, data);
        0
    }

    fn swap(&mut self, dst: Bits8) -> u8 {
        let data = self.get(dst);
        let lower = data & LOWER_HALF;
        let upper = data & UPPER_HALF;
        let data = (lower << 4) | (upper >> 4);
        self.set(Flag::Z, data == 0);
        self.set(Flag::H, false);
        self.set(Flag::N, false);
        self.set(Flag::C, false);
        self.set(dst, data);
        0
    }
}

impl Shift<u8> for Registers {
    fn shift_left(&mut self, mut data: u8) -> u8 {
        let carry = (data & BIT7) != 0;
        data <<= 1;
        self.set(Flag::C, carry);
        self.set(Flag::H, false);
        self.set(Flag::N, false);
        self.set(Flag::Z, data == 0);
        data
    }

    fn shift_arithmetic(&mut self, mut data: u8) -> u8 {
        let carry = (data & BIT0) != 0;
        let sign = data & BIT7;
        data &= !(BIT7);
        data >>= 1;
        data |= sign;
        self.set(Flag::C, carry);
        self.set(Flag::H, false);
        self.set(Flag::N, false);
        self.set(Flag::Z, data == 0);
        data
    }

    fn shift_logic(&mut self, mut data: u8) -> u8 {
        let carry = (data & BIT0) != 0;
        data >>= 1;
        self.set(Flag::C, carry);
        self.set(Flag::H, false);
        self.set(Flag::N, false);
        self.set(Flag::Z, data == 0);
        data
    }

    fn swap(&mut self, data: u8) -> u8 {
        let lower = data & LOWER_HALF;
        let upper = data & UPPER_HALF;
        let data = (lower << 4) | (upper >> 4);
        self.set(Flag::Z, data == 0);
        self.set(Flag::H, false);
        self.set(Flag::N, false);
        self.set(Flag::C, false);
        data
    }
}
