use super::{Arithmetic, Bits8, Bus, Flag};
use crate::registers::Registers;

pub trait Logical<T> {
    fn and(&mut self, area: T) -> u8;
    fn or(&mut self, area: T) -> u8;
    fn xor(&mut self, area: T) -> u8;
    fn compare(&mut self, area: T) -> u8;
}

impl Logical<Bits8> for Registers {
    fn and(&mut self, src: Bits8) -> u8 {
        let data = self.get(Bits8::A) & self.get(src);

        self.set(Flag::Z, data == 0);
        self.set(Flag::H, true);
        self.set(Flag::C, false);
        self.set(Flag::N, false);

        self.set(Bits8::A, data);
        0
    }

    fn or(&mut self, src: Bits8) -> u8 {
        let data = self.get(Bits8::A) | self.get(src);

        self.set(Flag::Z, data == 0);
        self.set(Flag::N, false);
        self.set(Flag::H, false);
        self.set(Flag::C, false);

        self.set(Bits8::A, data);
        0
    }

    fn xor(&mut self, src: Bits8) -> u8 {
        let data = self.get(Bits8::A) ^ self.get(src);

        self.set(Flag::Z, data == 0);
        self.set(Flag::N, false);
        self.set(Flag::H, false);
        self.set(Flag::C, false);

        self.set(Bits8::A, data);
        0
    }

    fn compare(&mut self, src: Bits8) -> u8 {
        let a = self.get(Bits8::A);
        self.sub(src, false);
        self.set(Bits8::A, a);
        0
    }
}

impl Logical<u8> for Registers {
    fn and(&mut self, src: u8) -> u8 {
        let data = self.get(Bits8::A) & src;

        self.set(Flag::Z, data == 0);
        self.set(Flag::H, true);
        self.set(Flag::C, false);
        self.set(Flag::N, false);

        self.set(Bits8::A, data);
        0
    }

    fn or(&mut self, src: u8) -> u8 {
        let data = self.get(Bits8::A) | src;
        self.set(Flag::Z, data == 0);
        self.set(Flag::N, false);
        self.set(Flag::H, false);
        self.set(Flag::C, false);

        self.set(Bits8::A, data);
        0
    }

    fn xor(&mut self, src: u8) -> u8 {
        let data = self.get(Bits8::A) ^ src;
        self.set(Flag::Z, data == 0);
        self.set(Flag::N, false);
        self.set(Flag::H, false);
        self.set(Flag::C, false);

        self.set(Bits8::A, data);
        0
    }

    fn compare(&mut self, src: u8) -> u8 {
        let a = self.get(Bits8::A);
        self.sub(src, false);
        self.set(Bits8::A, a);
        0
    }
}
