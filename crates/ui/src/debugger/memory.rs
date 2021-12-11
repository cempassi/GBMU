mod view;
use iced::{Column, Element};
use ppu::Ppu;

use crate::debugger::widgets::memory::Hexdump;
use crate::style::Theme;
use memory::{Bus, Memory as MemoryData, Rom};
use view::View;

pub struct Memory {
    active_tab: usize,
    _bios: Hexdump<Bus>,
    rom: Hexdump<Rom>,
    _vram: Hexdump<Ppu>,
}

#[derive(Debug, Clone)]
pub enum MemoryMsg {
    ActiveTab(usize),
}

impl Memory {
    pub fn new(data: MemoryData) -> Self {
        let bios = data.borrow().get_area(memory::Area::Bios);
        let ppu = data.borrow().get_ppu();
        let _bios = Hexdump::new("bios".to_string(), bios);
        let _rom = data.borrow().get_rom();
        let rom = Hexdump::new("rom".to_string(), _rom);
        let _vram = Hexdump::new("ppu".to_string(), ppu);
        let active_tab = 0;
        Self {
            active_tab,
            _bios,
            rom,
            _vram,
        }
    }

    pub fn update(&mut self, message: MemoryMsg) {
        match message {
            MemoryMsg::ActiveTab(tab) => self.active_tab = tab,
        }
    }

    pub fn view(&mut self, theme: Theme) -> Element<MemoryMsg> {
        Column::new().push(self.rom.view(theme)).into()
    }

    /// Get a reference to the memory's bios.
    pub fn _bios(&self) -> &Hexdump<Bus> {
        &self._bios
    }

    /// Get a reference to the memory's active tab.
    pub fn _active_tab(&self) -> usize {
        self.active_tab
    }
}
