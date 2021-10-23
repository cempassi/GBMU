use super::{Bits8, Flag};
use super::{Bus, Registers};

const BIT7: u8 = 0x80;
const BIT0: u8 = 0x1;

/// Shift arithmetic and logic are right shifts.

pub trait Rotation<T> {
    fn left_carry(&mut self, data: T) -> u8;
    fn left_nocarry(&mut self, data: T) -> u8;
    fn right_carry(&mut self, data: T) -> u8;
    fn right_nocarry(&mut self, data: T) -> u8;
}

impl Rotation<Bits8> for Registers {
    fn left_carry(&mut self, dst: Bits8) -> u8 {
        let data = self.get(dst);
        let carry = (data & BIT7) != 0;
        let data = (data << 1) | self.get(Flag::C) as u8;
        self.set(Flag::C, carry);
        self.set(Flag::Z, data == 0);
        self.set(dst, data);
        0
    }

    fn left_nocarry(&mut self, dst: Bits8) -> u8 {
        let data = self.get(dst);
        let carry = (data & BIT7) != 0;
        let data = (data << 1) | carry as u8;
        self.set(Flag::C, carry);
        self.set(Flag::Z, data == 0);
        self.set(dst, data);
        0
    }

    fn right_carry(&mut self, dst: Bits8) -> u8 {
        let data = self.get(dst);
        let carry = (data & BIT0) != 0;
        let data = (data >> 1) | ((self.get(Flag::C) as u8) << 7);
        self.set(Flag::C, carry);
        self.set(Flag::Z, data == 0);
        self.set(dst, data);
        0
    }

    fn right_nocarry(&mut self, dst: Bits8) -> u8 {
        let data = self.get(dst);
        let carry = (data & BIT0) != 0;
        let data = (data >> 1) | ((carry as u8) << 7);
        self.set(Flag::C, carry);
        self.set(Flag::Z, data == 0);
        self.set(dst, data);
        0
    }
}

impl Rotation<u8> for Registers {
    fn left_carry(&mut self, data: u8) -> u8 {
        let carry = (data & BIT7) != 0;
        let data = (data << 1) | self.get(Flag::C) as u8;
        self.set(Flag::C, carry);
        self.set(Flag::Z, data == 0);
        data
    }

    fn left_nocarry(&mut self, data: u8) -> u8 {
        let carry = (data & BIT7) != 0;
        let data = (data << 1) | carry as u8;
        self.set(Flag::C, carry);
        self.set(Flag::Z, data == 0);
        data
    }

    fn right_carry(&mut self, data: u8) -> u8 {
        let carry = (data & BIT0) != 0;
        let data = (data >> 1) | ((self.get(Flag::C) as u8) << 7);
        self.set(Flag::C, carry);
        self.set(Flag::Z, data == 0);
        data
    }

    fn right_nocarry(&mut self, data: u8) -> u8 {
        let carry = (data & BIT0) != 0;
        let data = (data >> 1) | ((carry as u8) << 7);
        self.set(Flag::C, carry);
        self.set(Flag::Z, data == 0);
        data
    }
}
