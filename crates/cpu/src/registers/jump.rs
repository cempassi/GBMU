use super::{Bits16, Bus, Flag, Registers};

pub trait Relative {
    fn relative(&mut self, offset: i8) -> u8;
    fn relative_check(&mut self, offset: i8, flag: Flag) -> bool;
    fn relative_not(&mut self, offset: i8, flag: Flag) -> bool;
}

impl Relative for Registers {
    fn relative(&mut self, offset: i8) -> u8 {
        let mut address: i16 = self.get(Bits16::PC) as i16;
        address = address.wrapping_add(offset.into());
        self.set(Bits16::PC, address as u16);
        0
    }

    fn relative_check(&mut self, offset: i8, flag: Flag) -> bool {
        if self.get(flag) {
            let mut address: i16 = self.get(Bits16::PC) as i16;
            address = address.wrapping_add(offset.into());
            self.set(Bits16::PC, address as u16);
            true
        } else {
            false
        }
    }

    fn relative_not(&mut self, offset: i8, flag: Flag) -> bool {
        if !self.get(flag) {
            let mut address: i16 = self.get(Bits16::PC) as i16;
            address = address.wrapping_add(offset.into());
            self.set(Bits16::PC, address as u16);
            true
        } else {
            false
        }
    }
}

pub(crate) trait Absolute<T> {
    fn absolute(&mut self, address: T) -> u8;
    fn absolute_check(&mut self, adress: T, flag: Flag) -> bool;
    fn absolute_not(&mut self, adress: T, flag: Flag) -> bool;
}

impl Absolute<u16> for Registers {
    fn absolute(&mut self, address: u16) -> u8 {
        self.set(Bits16::PC, address);
        0
    }

    fn absolute_check(&mut self, address: u16, flag: Flag) -> bool {
        if self.get(flag) {
            self.set(Bits16::PC, address);
            true
        } else {
            false
        }
    }

    fn absolute_not(&mut self, address: u16, flag: Flag) -> bool {
        if !self.get(flag) {
            self.set(Bits16::PC, address);
            true
        } else {
            false
        }
    }
}

impl Absolute<Bits16> for Registers {
    fn absolute(&mut self, address: Bits16) -> u8 {
        self.set(Bits16::PC, self.get(address));
        0
    }

    fn absolute_check(&mut self, address: Bits16, flag: Flag) -> bool {
        if self.get(flag) {
            self.set(Bits16::PC, self.get(address));
            true
        } else {
            false
        }
    }

    fn absolute_not(&mut self, address: Bits16, flag: Flag) -> bool {
        if !self.get(flag) {
            self.set(Bits16::PC, self.get(address));
            true
        } else {
            false
        }
    }
}
