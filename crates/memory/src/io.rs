use crate::{consts, Area};
use crate::{Serial, Timer};
use apu::Apu;
use shared::Interrupts;

#[derive(Debug)]
pub struct IO {
    _apu: Apu,
    _joypad: u8,
    timer: Timer,
    serial: Serial,
    temp: Vec<u8>,
}

impl IO {
    pub fn new(interrupts: Interrupts) -> Self {
        let _apu = Apu::default();
        let _joypad = 0;
        let temp = vec![0; 0xF7];
        let timer = Timer::new(interrupts);
        let serial = Serial::default();
        Self {
            _apu,
            _joypad,
            timer,
            serial,
            temp,
        }
    }

    pub fn get(&self, address: u16) -> u8 {
        match address {
            consts::SERIAL_DATA | consts::SERIAL_CONTROL => self.serial.get(address),
            consts::DIV..=consts::TIMA => self.timer.get(address),
            _ => {
                let address = Area::IOReg.relative(address);
                self.temp[address]
            }
        }
    }

    pub fn set(&mut self, address: u16, data: u8) {
        match address {
            consts::SERIAL_DATA | consts::SERIAL_CONTROL => self.serial.set(address, data),
            consts::DIV..=consts::TIMA => self.timer.set(address, data),
            _ => {
                let address = Area::IOReg.relative(address);
                self.temp[address] = data;
            }
        }
    }

    pub fn tick(&mut self) {
        self.timer.tick()
    }
}
