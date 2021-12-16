use crate::consts;
use shared::{Interrupt, Interrupts};

const CLOCK_1024: u16 = 0b0000_0010_0000_0000;
const CLOCK_16: u16 = 0b0000_0000_0000_1000;
const CLOCK_64: u16 = 0b0000_0000_0010_0000;
const CLOCK_256: u16 = 0b0000_0000_1000_0000;

#[derive(Debug)]
pub struct Timer {
    interrupts: Interrupts,
    div: u16,
    tima: u8,
    tma: u8,
    tac: u8,
}

impl Timer {
    pub fn new(interrupts: Interrupts) -> Self {
        Self {
            interrupts,
            div: 0xAC00,
            tima: 0,
            tma: 0,
            tac: 0,
        }
    }

    pub fn tick(&mut self) {
        let prev_div = self.div;

        self.div = self.div.wrapping_add(1);

        let update = match self.tac & 0b11 {
            0b00 => (prev_div & CLOCK_1024) != 0 && (self.div & CLOCK_1024) == 0,
            0b01 => (prev_div & CLOCK_16) != 0 && (self.div & CLOCK_16) == 0,
            0b10 => (prev_div & CLOCK_64) != 0 && (self.div & CLOCK_64) == 0,
            0b11 => (prev_div & CLOCK_256) != 0 && (self.div & CLOCK_256) == 0,
            _ => unreachable!(),
        };

        if update && (self.tac & 0b10) != 0 {
            self.tima = self.tima.wrapping_add(1);
            if self.tima == 0xFF {
                self.tima = self.tma;
                self.interrupts.borrow_mut().request(Interrupt::Timer);
            }
        }
    }

    pub fn get(&self, address: u16) -> u8 {
        match address {
            consts::DIV => (self.div >> 8) as u8,
            consts::TIMA => self.tima,
            consts::TMA => self.tma,
            consts::TAC => self.tac,
            _ => unreachable!(),
        }
    }

    pub fn set(&mut self, address: u16, data: u8) {
        match address {
            consts::DIV => self.div = 0,
            consts::TIMA => self.tima = data,
            consts::TMA => self.tma = data,
            consts::TAC => self.tac = data,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test_memory {
    use super::*;
    #[test]
    fn test_clock_1024() {
        assert_eq!(CLOCK_1024, (1 << 9));
    }

    #[test]
    fn test_clock_16() {
        assert_eq!(CLOCK_16, 1 << 3);
    }
    #[test]
    fn test_clock_64() {
        assert_eq!(CLOCK_64, 1 << 5);
    }
    #[test]
    fn test_clock_256() {
        assert_eq!(CLOCK_256, 1 << 7);
    }
}
