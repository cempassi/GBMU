use crate::style::Theme;
use iced_wgpu::Renderer;
use iced_winit::Element;

use super::lcd::Lcd;

pub struct Ppu {
    ppu: ppu::Ppu,
    lcd: Lcd,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PpuMsg {
    Refresh,
}

impl Ppu {
    pub fn new(ppu: ppu::Ppu) -> Self {
        let lcd = Lcd::new(&ppu);
        Self { ppu, lcd }
    }

    pub fn update(&mut self, _message: PpuMsg) {
        self.lcd.update(&self.ppu)
    }

    pub fn view(&self, _theme: Theme) -> Element<PpuMsg, Renderer> {
        self.lcd.view()
    }
}
