use super::{Bits8, Bus, Flag, Registers};

pub trait Complement {
    fn daa(&mut self) -> u8;
    fn set_carry(&mut self) -> u8;
    fn complement_carry(&mut self) -> u8;
    fn complement_a(&mut self) -> u8;
}

impl Complement for Registers {
    fn daa(&mut self) -> u8 {
        let mut data = self.get(Bits8::A);
        let mut adjust = if self.get(Flag::C) { 0x60 } else { 0x00 };
        if self.get(Flag::H) {
            adjust |= 0x06;
        };

        if !self.get(Flag::N) {
            if data & 0x0F > 0x09 {
                adjust |= 0x06
            };
            if data > 0x99 {
                adjust |= 0x60
            };
            data = data.wrapping_add(adjust);
        } else {
            data = data.wrapping_sub(adjust);
        }

        self.set(Flag::C, adjust >= 0x60);
        self.set(Flag::H, false);
        self.set(Flag::Z, data == 0);

        self.set(Bits8::A, data);
        0
    }

    fn set_carry(&mut self) -> u8 {
        self.set(Flag::C, true);
        self.set(Flag::N, false);
        self.set(Flag::H, false);
        0
    }

    fn complement_carry(&mut self) -> u8 {
        self.f.complement_c();
        self.set(Flag::N, false);
        self.set(Flag::H, false);
        0
    }

    fn complement_a(&mut self) -> u8 {
        self.a = !self.a;
        self.set(Flag::N, true);
        self.set(Flag::H, true);
        0
    }
}
