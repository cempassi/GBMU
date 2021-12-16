use crate::Timer;
use crate::{consts, Area};
use apu::Apu;
use shared::Interrupts;

#[derive(Debug)]
pub struct IO {
    _apu: Apu,
    _joypad: u8,
    timer: Timer,
    temp: Vec<u8>,
}

impl IO {
    pub fn new(interrupts: Interrupts) -> Self {
        let _apu = Apu::default();
        let _joypad = 0;
        let temp = vec![0; 0xF7];
        let timer = Timer::new(interrupts);
        Self {
            _apu,
            _joypad,
            timer,
            temp,
        }
    }

    pub fn get(&self, address: u16) -> u8 {
        if matches!(address, consts::DIV..=consts::TIMA) {
            self.timer.get(address)
        } else {
            let address = Area::IOReg.relative(address);
            self.temp[address]
        }
    }

    pub fn set(&mut self, address: u16, data: u8) {
        if matches!(address, consts::DIV..=consts::TIMA) {
            self.timer.set(address, data)
        } else {
            let address = Area::IOReg.relative(address);
            self.temp[address] = data;
        }
    }

    pub fn tick(&mut self) {
        self.timer.tick()
    }
}
