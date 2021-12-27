use super::{Bits8, Flag};
use super::{Bus, Registers};

/// Shift arithmetic and logic are right shifts.

pub trait Rotation<T> {
    fn rotate_left(&mut self, dst: T, use_carry: bool, use_zero: bool) -> u8;
    fn rotate_right(&mut self, dst: T, use_carry: bool, use_zero: bool) -> u8;
}

impl Rotation<Bits8> for Registers {
    fn rotate_left(&mut self, dst: Bits8, use_carry: bool, use_zero: bool) -> u8 {
        let data = self.get(dst);
        let bit7 = data >> 7;
		let result = match use_carry {
			true =>  {
				(data << 1) | self.f.is_carried(use_carry)
			},
			false => data.rotate_left(1),
		};
		self.set(Flag::C, bit7 == 1);
		self.set(Flag::H, false);
		self.set(Flag::N, false);
		self.set(Flag::Z, result == 0 && use_zero);
		self.set(dst, result);
        0
    }

    fn rotate_right(&mut self, dst: Bits8, use_carry: bool, use_zero: bool) -> u8 {
        let data = self.get(dst);
        let bit1 = data & 1;
		let result = match use_carry {
			true =>  {
				(data >> 1) | (self.f.is_carried(use_carry) << 7)
			},
			false => data.rotate_right(1),
		};
		self.set(Flag::C, bit1 == 1);
		self.set(Flag::H, false);
		self.set(Flag::N, false);
		self.set(Flag::Z, result == 0 && use_zero);
		self.set(dst, result);
        0
    }
}

impl Rotation<u8> for Registers {
    fn rotate_left(&mut self, data: u8, use_carry: bool, use_zero: bool) -> u8 {
        let bit7 = data >> 7;
		let result = match use_carry {
			true =>  {
				(data << 1) | self.f.is_carried(use_carry)
			},
			false => data.rotate_left(1),
		};
		self.set(Flag::C, bit7 == 1);
		self.set(Flag::H, false);
		self.set(Flag::N, false);
		self.set(Flag::Z, result == 0 && use_zero);
        result
    }

    fn rotate_right(&mut self, data: u8, use_carry: bool, use_zero: bool) -> u8 {
        let bit1 = data & 1;
		let result = match use_carry {
			true =>  {
				(data >> 1) | (self.f.is_carried(use_carry) << 7)
			},
			false => data.rotate_right(1),
		};
		self.set(Flag::C, bit1 == 1);
		self.set(Flag::H, false);
		self.set(Flag::N, false);
		self.set(Flag::Z, result == 0 && use_zero);
        result
    }
}
