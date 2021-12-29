use super::Bus;
use super::Registers;
use super::{Bits8, Flag};

pub trait Bitwise<T> {
    fn test(&mut self, area: T, bit: u8) -> u8;
    fn reset(&mut self, area: T, bit: u8) -> u8;
    fn bitset(&mut self, area: T, bit: u8) -> u8;
}

impl Bitwise<Bits8> for Registers {
    fn test(&mut self, area: Bits8, bit: u8) -> u8 {
        let byte = self.get(area);
        let test = ((bit as u8) & !byte) != 0;
        self.set(Flag::Z, test);
        self.set(Flag::N, false);
        self.set(Flag::H, true);
        0
    }

    fn reset(&mut self, area: Bits8, bit: u8) -> u8 {
        let byte = self.get(area);
        self.set(area, byte & !bit);
        0
    }

    fn bitset(&mut self, area: Bits8, bit: u8) -> u8 {
        let byte = self.get(area);
        self.set(area, byte | bit);
        0
    }
}

impl Bitwise<u8> for Registers {
    fn test(&mut self, byte: u8, bit: u8) -> u8 {
        let test = ((bit as u8) & !byte) != 0;
        self.set(Flag::Z, test);
        self.set(Flag::N, false);
        self.set(Flag::H, true);
        0
    }

    fn reset(&mut self, byte: u8, bit: u8) -> u8 {
        byte & !bit
    }

    fn bitset(&mut self, byte: u8, bit: u8) -> u8 {
        byte | bit
    }
}
