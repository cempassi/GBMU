use crate::Area;
use apu::Apu;

#[derive(Debug)]
pub struct IO {
    _apu: Apu,
    _joypad: u8,
    temp: Vec<u8>,
}

impl Default for IO {
    fn default() -> Self {
        Self::new()
    }
}

impl IO {
    pub fn new() -> Self {
        let _apu = Apu::default();
        let _joypad = 0;
        let temp = vec![0; 0xF7];
        Self {
            _apu,
            _joypad,
            temp,
        }
    }

    pub fn get(&self, address: u16) -> u8 {
        let address = Area::IOReg.relative(address);
        self.temp[address]
    }

    pub fn set(&mut self, address: u16, data: u8) {
        let address = Area::IOReg.relative(address);
        self.temp[address] = data;
    }
}
