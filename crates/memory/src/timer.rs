use crate::consts;
use shared::{Interrupt, Interrupts};

#[derive(Debug)]
pub struct Timer {
    interrupts: Interrupts,
    div: u8,
    tima: u8,
    tma: u8,
    enabled: bool,
    step: u32,
    internal_count: u32,
    internal_divider: u32,
}

impl Timer {
    pub fn new(interrupts: Interrupts) -> Self {
        Self {
            interrupts,
            div: 0,
            tima: 0,
            tma: 0,
            enabled: false,
            step: 256,
            internal_count: 0,
            internal_divider: 0,
        }
    }

    pub fn tick(&mut self) {
        self.internal_divider += 1;

        if self.internal_divider >= 256 {
            self.div = self.div.wrapping_add(1);
            self.internal_divider -= 256;
        }

        if self.enabled {
            self.internal_count += 1;

            if self.internal_count >= self.step {
                self.tima = self.tima.wrapping_add(1);
                if self.tima == 0 {
                    self.tima = self.tma;
                    self.interrupts.borrow_mut().request(Interrupt::Timer);
                }
                self.internal_count -= self.step;
            }
        }
    }

    pub fn get(&self, address: u16) -> u8 {
        match address {
            consts::DIV => self.div,
            consts::TIMA => self.tima,
            consts::TMA => self.tma,
            consts::TAC => {
                let enabled = if self.enabled { 0x4 } else { 0 };
                let step = match self.step {
                    16 => 1,
                    64 => 2,
                    256 => 3,
                    _ => 0,
                };
                enabled | step
            }
            _ => unreachable!(),
        }
    }

    pub fn set(&mut self, address: u16, data: u8) {
        println!("Setting timer. address: {:#X}, data: {:#b}", address, data);
        match address {
            consts::DIV => self.div = 0,
            consts::TIMA => self.tima = data,
            consts::TMA => self.tma = data,
            consts::TAC => {
                self.enabled = data & 0x4 != 0;
                if self.enabled {
                    println!("Timer enabled");
                }
                self.step = match data & 0x3 {
                    1 => 16,
                    2 => 64,
                    3 => 256,
                    _ => 1024,
                };
            }
            _ => unreachable!(),
        }
    }
}
