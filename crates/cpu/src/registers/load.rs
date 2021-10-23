use super::{Bits16, Bits8, Bus, Registers};

pub trait Load<T> {
    fn load(&mut self, dst: T, src: T) -> u8;
}

impl Load<Bits8> for Registers {
    fn load(&mut self, dst: Bits8, src: Bits8) -> u8 {
        self.set(dst, self.get(src));
        0
    }
}

impl Load<Bits16> for Registers {
    fn load(&mut self, dst: Bits16, src: Bits16) -> u8 {
        self.set(dst, self.get(src));
        0
    }
}
