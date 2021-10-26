use super::{Bits16, Bits8, Bus, Carry, Flag, Registers};

pub trait Arithmetic<T> {
    fn add(&mut self, lhs: T, use_carry: bool) -> u8;

    fn sub(&mut self, lhs: T, use_carry: bool) -> u8;
}

impl Arithmetic<u8> for Registers {
    fn add(&mut self, data: u8, use_carry: bool) -> u8 {
        let carry: u8 = self.f.is_carried(use_carry);
        let a = self.get(Bits8::A);
        let result = self.f.checked_add(a, data + carry);
        self.set(Flag::N, false);
        self.set(Bits8::A, result);
        0
    }

    fn sub(&mut self, data: u8, use_carry: bool) -> u8 {
        let carry: u8 = self.f.is_carried(use_carry);
        let a = self.get(Bits8::A);
        let result = self.f.checked_sub(a, data + carry);
        self.set(Flag::N, true);
        self.set(Bits8::A, result);
        0
    }
}

impl Arithmetic<Bits8> for Registers {
    fn add(&mut self, data: Bits8, use_carry: bool) -> u8 {
        let data: u8 = self.get(data);
        let carry: u8 = self.f.is_carried(use_carry);
        let a = self.get(Bits8::A);
        let result = self.f.checked_add(a, data + carry);
        self.set(Flag::N, false);
        self.set(Bits8::A, result);
        0
    }

    fn sub(&mut self, data: Bits8, use_carry: bool) -> u8 {
        let data: u8 = self.get(data);
        let carry: u8 = self.f.is_carried(use_carry);
        let a = self.get(Bits8::A);
        let result = self.f.checked_sub(a, data + carry);
        self.set(Flag::N, true);
        self.set(Bits8::A, result);
        0
    }
}

impl Arithmetic<Bits16> for Registers {
    fn add(&mut self, data: Bits16, use_carry: bool) -> u8 {
        let data = self.get(data);
        let carry: u16 = self.f.is_carried(use_carry).into();
        let dst = self.get(Bits16::HL);
        let result = self.f.checked_add(dst, data + carry);
        self.set(Flag::N, false);
        self.set(Bits16::HL, result);
        0
    }

    fn sub(&mut self, _data: Bits16, _use_carry: bool) -> u8 {
        unreachable!()
    }
}
