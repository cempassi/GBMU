use apu::Apu;
use crate::Area;

#[derive(Debug)]
pub struct IO {
    apu: Apu,
    joypad: u8,
    temp: Vec<u8>
}

impl Default for IO {
    fn default() -> Self {
        Self::new()
    }
}

impl IO {
    pub fn new() -> Self {
        let apu = Apu::default();
        let joypad = 0;
        let temp = vec![0; 0xF7];
        Self {
            apu,
            joypad,
            temp
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
