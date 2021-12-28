use super::{Bits16, Bits8, Bus, Flag, Registers};

pub trait IncDec<T, U> {
    fn increase(&mut self, _: T, n: U) -> u8;

    fn decrease(&mut self, _: T, n: U) -> u8;
}

impl IncDec<Bits8, u8> for Registers {
    fn increase(&mut self, area: Bits8, n: u8) -> u8 {
        let data = self.get(area);
        let inc = data.wrapping_add(n);
        self.set(Flag::Z, inc == 0);
        self.set(Flag::H, (data & 0xF) + 1 > 0xF);
        self.set(Flag::N, false);
        self.set(area, inc);
        0
    }

    fn decrease(&mut self, area: Bits8, n: u8) -> u8 {
        let data = self.get(area);
        let dec = data.wrapping_sub(n);
        self.set(Flag::Z, dec == 0);
        self.set(Flag::H, ((data - 1) & 0x0F) == 0);
        self.set(Flag::N, true);
        self.set(area, dec);
        0
    }
}

impl IncDec<u8, u8> for Registers {
    fn increase(&mut self, data: u8, n: u8) -> u8 {
        let inc = data.wrapping_add(n);

        self.set(Flag::Z, inc == 0);
        self.set(Flag::H, (data & 0xF) + 1 > 0xF);
        self.set(Flag::N, false);
        inc
    }

    fn decrease(&mut self, data: u8, n: u8) -> u8 {
        let dec = data.wrapping_sub(n);
        self.set(Flag::Z, dec == 0);
        self.set(Flag::H, (data & 0x0F) == 0);
        self.set(Flag::N, true);
        dec
    }
}

impl IncDec<Bits16, u16> for Registers {
    fn increase(&mut self, area: Bits16, n: u16) -> u8 {
        let data = self.get(area).wrapping_add(n);
        self.set(area, data);
        0
    }

    fn decrease(&mut self, area: Bits16, n: u16) -> u8 {
        let data = self.get(area).wrapping_sub(n);
        self.set(area, data);
        0
    }
}
