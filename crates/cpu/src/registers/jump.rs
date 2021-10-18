use super::{Bits16, Bus, Flag, Registers};

pub(crate) trait Absolute<T> {
    fn absolute(&mut self, address: T);
    fn absolute_check(&mut self, adress: T, flag: Flag);
    fn absolute_not(&mut self, adress: T, flag: Flag);
}

pub trait Relative {
    fn jump_relative(&mut self, offset: i8);
    fn jump_relative_check(&mut self, offset: i8, flag: Flag);
    fn jump_relative_not(&mut self, offset: i8, flag: Flag);
}

impl Relative for Registers {
    fn jump_relative(&mut self, offset: i8) {
        let mut address: i16 = self.get(Bits16::PC) as i16;
        address = address.wrapping_add(offset.into());
        self.set(Bits16::PC, address as u16);
    }

    fn jump_relative_check(&mut self, offset: i8, flag: Flag) {
        if self.get(flag) {
            let mut address: i16 = self.get(Bits16::PC) as i16;
            address = address.wrapping_add(offset.into());
            self.set(Bits16::PC, address as u16);
        }
    }

    fn jump_relative_not(&mut self, offset: i8, flag: Flag) {
        if !self.get(flag) {
            let mut address: i16 = self.get(Bits16::PC) as i16;
            address = address.wrapping_add(offset.into());
            self.set(Bits16::PC, address as u16);
        }
    }
}

impl Absolute<u16> for Registers {
    fn absolute(&mut self, address: u16) {
        self.set(Bits16::PC, address);
    }

    fn absolute_check(&mut self, address: u16, flag: Flag) {
        if self.get(flag) {
            self.set(Bits16::PC, address)
        }
    }

    fn absolute_not(&mut self, address: u16, flag: Flag) {
        if !self.get(flag) {
            self.set(Bits16::PC, address)
        }
    }
}

impl Absolute<Bits16> for Registers {
    fn absolute(&mut self, address: Bits16) {
        self.set(Bits16::PC, self.get(address));
    }

    fn absolute_check(&mut self, address: Bits16, flag: Flag) {
        if self.get(flag) {
            self.set(Bits16::PC, self.get(address));
        }
    }

    fn absolute_not(&mut self, address: Bits16, flag: Flag) {
        if !self.get(flag) {
            self.set(Bits16::PC, self.get(address));
        }
    }
}
