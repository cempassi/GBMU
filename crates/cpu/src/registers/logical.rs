use super::{Bits8, Flag, Arithmetic};
use crate::registers::Registers;
use crate::RegisterBus;

pub trait Logical<T> {
    fn and(&mut self, area: T);
    fn or(&mut self, area: T);
    fn xor(&mut self, area: T);
    fn compare(&mut self, area: T);
}

impl Logical<Bits8> for Registers {
    fn and(&mut self, src: Bits8) {
        let data = self.get(Bits8::A) & self.get(src);
        self.set(Bits8::A, data);
        self.set(Flag::Z, data == 0);
        self.set(Flag::H, true);
    }

    fn or(&mut self, src: Bits8) {
        let data = self.get(Bits8::A) | self.get(src);
        self.set(Bits8::A, data);
        self.set(Flag::Z, data == 0);
        self.set(Flag::H, true);
    }

    fn xor(&mut self, src: Bits8) {
        let data = self.get(Bits8::A) ^ self.get(src);
        self.set(Bits8::A, data);
        self.set(Flag::Z, data == 0);
        self.set(Flag::H, true);
    }

    fn compare(&mut self, src: Bits8) {
        let data = self.get(Bits8::A);
        self.sub(self.get(src), false);
        self.set(Bits8::A, data);
    }
}

impl Logical<u8> for Registers {
    fn and(&mut self, src: u8) {
        let data = self.get(Bits8::A) & src;
        self.set(Bits8::A, data);
        self.set(Flag::Z, data == 0);
        self.set(Flag::H, true);
    }

    fn or(&mut self, src: u8) {
        let data = self.get(Bits8::A) | src;
        self.set(Bits8::A, data);
        self.set(Flag::Z, data == 0);
        self.set(Flag::H, true);
    }

    fn xor(&mut self, src: u8) {
        let data = self.get(Bits8::A) ^ src;
        self.set(Bits8::A, data);
        self.set(Flag::Z, data == 0);
        self.set(Flag::H, true);
    }

    fn compare(&mut self, src: u8) {
        let data = self.get(Bits8::A);
        self.sub(src, false);
        self.set(Bits8::A, data);
    }
}
