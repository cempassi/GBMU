mod lcd;

use crate::debugger::widgets::Text;
use crate::style::Theme;
use iced_graphics::Alignment;
use iced_wgpu::{Column, Renderer};
use iced_winit::Element;
use lcd::Lcd;

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
        let title = Text::new("Ppu").medium_it(20);
        let ppu = Column::new().align_items(Alignment::Center);
        ppu.push(title).push(self.lcd.view()).into()
    }
}
