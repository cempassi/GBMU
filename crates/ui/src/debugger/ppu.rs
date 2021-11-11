mod control;
mod coordinates;

use crate::debugger::widgets::{Register, Text};
use crate::style::Theme;
use iced::{Alignment, Column, Element, Row};
use ppu::Registers;

pub struct Ppu {
    ppu: ppu::Ppu,
    data: ppu::Registers,
}

pub trait View<Msg> {
    fn view(&self, theme: Theme) -> Element<Msg>;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PpuMsg {
    Refresh,
}

impl Ppu {
    pub fn new(ppu: ppu::Ppu) -> Self {
        let data = ppu::Registers::default();
        Self { ppu, data }
    }

    pub fn update(&mut self, _message: PpuMsg) {
        self.ppu.borrow().update_registers(&mut self.data);
    }

    pub fn view(&self, theme: Theme) -> Element<PpuMsg> {
        self.data.view(theme)
    }
}

impl View<PpuMsg> for Registers {
    fn view(&self, theme: Theme) -> Element<PpuMsg> {
        let ppu = Text::new("Ppu").medium_it(20);
        let ppu = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(ppu);

        let status = Text::new("Status").medium_it(20);
        let status = Column::new().align_items(Alignment::Center).push(status);

        let mode = format!("{:?}", self.mode);
        let mode = Register::render("Mode".to_string(), mode);
        let ly_lyc = Register::render("Ly = Ly Compare".to_string(), self.lyc_ly.to_string());
        let line = Row::new().push(mode).push(ly_lyc);
        let status = status.push(line);

        let interupts = Text::new("Interrupts").medium_it(20);
        let interupts = Column::new().align_items(Alignment::Center).push(interupts);

        let hblank = Register::render("HBlank".to_string(), self.hblank_interupt.to_string());
        let vblank = Register::render("VBlank".to_string(), self.vblank_interupt.to_string());
        let line = Row::new().push(hblank).push(vblank);
        let interupts = interupts.push(line);

        let oam = Register::render("OAM".to_string(), self.oam_interupt.to_string());
        let ly_lyc = Register::render(
            "Ly = Ly Compare".to_string(),
            self.lyc_ly_interupt.to_string(),
        );
        let line = Row::new().push(oam).push(ly_lyc);
        let interupts = interupts.push(line);

        let control = self.control.view(theme);
        let coordinates = self.coordinates.view(theme);
        ppu.push(status)
            .push(control)
            .push(coordinates)
            .push(interupts)
            .into()
    }
}
