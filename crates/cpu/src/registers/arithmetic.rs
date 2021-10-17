use super::{Bits16, Bits8, Bus, Flag, Registers};

const HALF_U8: u8 = 0xF;
const HALF_U16: u16 = 0xFF;

pub trait Arithmetic<T, U> {
    fn increase(&mut self, _: T, n: U);

    fn decrease(&mut self, _: T, n: U);

    fn add(&mut self, lhs: U, use_carry: bool);

    fn sub(&mut self, lhs: U, use_carry: bool);
}

impl Arithmetic<Bits8, u8> for Registers {
    fn increase(&mut self, area: Bits8, n: u8) {
        let data = self.get(area).wrapping_add(n);
        self.set(area, data);
        self.set(Flag::N, false);
        self.set(Flag::Z, data == 0)
    }

    fn decrease(&mut self, area: Bits8, n: u8) {
        let data = self.get(area).wrapping_sub(n);
        self.set(area, data);
        self.set(Flag::N, true);
        self.set(Flag::Z, data == 0)
    }

    fn add(&mut self, data: u8, use_carry: bool) {
        let carry: u8 = if use_carry && self.get(Flag::C) { 1 } else { 0 };
        let a = self.get(Bits8::A);
        let result = a.wrapping_add(data).wrapping_add(carry);
        let check_half = (a & HALF_U8) + (data & HALF_U8) + carry > HALF_U8;
        let check_carry = (a as u16) + (data as u16) + carry as u16 > u8::MAX as u16;
        self.set(Bits8::A, result);
        self.set(Flag::H, check_half);
        self.set(Flag::C, check_carry);
        self.set(Flag::Z, result == 0);
    }

    fn sub(&mut self, data: u8, use_carry: bool) {
        let carry: u8 = if use_carry && self.get(Flag::C) { 1 } else { 0 };
        let a = self.get(Bits8::A);
        let result = a.wrapping_sub(data).wrapping_sub(carry);
        let check_half = (a & HALF_U8) < (data & HALF_U8) + carry;
        let check_carry = (a as u16) + (data as u16) + carry as u16 > u8::MAX as u16;
        self.set(Bits8::A, result);
        self.set(Flag::C, check_carry);
        self.set(Flag::H, check_half);
        self.set(Flag::N, true);
        self.set(Flag::Z, result == 0);
    }
}

impl Arithmetic<Bits16, u16> for Registers {
    fn increase(&mut self, area: Bits16, n: u16) {
        let data = self.get(area).wrapping_add(n);
        self.set(area, data)
    }

    fn decrease(&mut self, area: Bits16, n: u16) {
        let data = self.get(area).wrapping_sub(n);
        self.set(area, data)
    }

    fn add(&mut self, data: u16, use_carry: bool) {
        let carry: u16 = if use_carry && self.get(Flag::C) { 1 } else { 0 };
        let hl = self.get(Bits16::HL);
        let result = hl.wrapping_add(data).wrapping_add(carry);
        let check_half = (hl & HALF_U16) + (data & HALF_U16) + carry > HALF_U16;
        let check_carry = (hl as u32) + (data as u32) + carry as u32 > u16::MAX as u32;
        self.set(Bits16::HL, result);
        self.set(Flag::H, check_half);
        self.set(Flag::C, check_carry);
        self.set(Flag::Z, result == 0);
    }

    fn sub(&mut self, _data: u16, _use_carry: bool) {
        unreachable!()
    }
}
