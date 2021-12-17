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
        if !self.get(Flag::N) {
            data = data.wrapping_add(self.f.add_h_check(data));
            data = data.wrapping_add(self.f.add_c_check(data));
        } else {
            data = data.wrapping_sub(self.f.sub_c_check());
            data = data.wrapping_sub(self.f.sub_h_check());
        }
        self.set(Bits8::A, data);
        self.set(Flag::Z, data == 0);
        self.set(Flag::H, false);
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
