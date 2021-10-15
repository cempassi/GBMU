use super::area::{Bits16, Bits8};
use crate::registers::Registers;
use crate::RegisterBus;

pub trait Arithmetic<T, U> {
    fn increase(&mut self, _: T, n: U);

    fn decrease(&mut self, _: T, n: U);

    fn add(&mut self, lhs: T, rhs: T);
}

impl Arithmetic<Bits8, u8> for Registers {
    fn increase(&mut self, area: Bits8, n: u8) {
        let data = self.get(area).wrapping_add(n);
        self.set(area, data)
    }

    fn decrease(&mut self, area: Bits8, n: u8) {
        let data = self.get(area).wrapping_sub(n);
        self.set(area, data)
    }

    fn add(&mut self, lhs: Bits8, rhs: Bits8) {
        let data = self.get(lhs).wrapping_add(self.get(rhs));
        self.set(lhs, data);
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

    fn add(&mut self, lhs: Bits16, rhs: Bits16) {
        let data = self.get(lhs).wrapping_add(self.get(rhs));
        self.set(lhs, data);
    }
}
