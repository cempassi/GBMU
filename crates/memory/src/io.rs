use crate::{consts, Area};
use crate::{Serial, Timer, Joypad};
use apu::Apu;
use shared::{Error, Interrupts};

#[derive(Debug)]
pub struct IO {
    _apu: Apu,
    joypad: Joypad,
    timer: Timer,
    serial: Serial,
    temp: Vec<u8>,
}

impl IO {
    pub fn new(interrupts: Interrupts) -> Self {
        let _apu = Apu::default();
        let joypad = Joypad::new(interrupts.clone());
        let temp = vec![0; 0xF7];
        let timer = Timer::new(interrupts);
        let serial = Serial::default();
        Self {
            _apu,
            joypad,
            timer,
            serial,
            temp,
        }
    }

    pub fn get(&self, address: u16) -> u8 {
        match address {
            consts::JOYPAD => self.joypad.get(),
            consts::SERIAL_DATA | consts::SERIAL_CONTROL => self.serial.get(address),
            consts::DIV..=consts::TAC => self.timer.get(address),
            _ => {
                let address = Area::IOReg.relative(address);
                self.temp[address]
            }
        }
    }

    pub fn set(&mut self, address: u16, data: u8) -> Result<(), Error> {
        match address {
            consts::JOYPAD => self.joypad.set(data),
            consts::SERIAL_DATA | consts::SERIAL_CONTROL => self.serial.set(address, data),
            consts::DIV..=consts::TAC => self.timer.set(address, data),
            _ => {
                let address = Area::IOReg.relative(address);
                self.temp[address] = data;
            }
        };
        Ok(())
    }

    pub fn tick(&mut self) {
        self.timer.tick()
    }
}
