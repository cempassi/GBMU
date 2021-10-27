use super::lcd::Lcd;

pub struct Ppu {
    ppu: ppu::Ppu,
    lcd: Lcd,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PpuMsg {}

impl Ppu {
    pub fn new(ppu: ppu::Ppu) -> Self {
        let lcd = Lcd::new(ppu.borrow().get_lcd());
        Self { ppu, lcd }
    }

    pub fn update(&mut self, _message: PpuMsg) {
        self.lcd.update(self.ppu.borrow().get_lcd())
    }
}
