use super::{Bits16, Bits8, Bus, Registers};

pub trait Load<T> {
    fn load(&mut self, dst: T, src: T);
}

impl Load<Bits8> for Registers {
    fn load(&mut self, dst: Bits8, src: Bits8) {
        self.set(dst, self.get(src));
    }
}

impl Load<Bits16> for Registers {
    fn load(&mut self, dst: Bits16, src: Bits16) {
        self.set(dst, self.get(src));
    }
}
