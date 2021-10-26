pub struct Lcd {
    lcd: ppu::Lcd,
}

impl Lcd {
    pub fn new(src: &ppu::Lcd) -> Self {
        let lcd = Clone::clone(&*src);
        Self { lcd }
    }

    pub fn update(&mut self, lcd: &ppu::Lcd) {
        lcd.set(&mut self.lcd)
    }
}
