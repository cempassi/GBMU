use iced_wgpu::{scrollable, Renderer, Scrollable};
use iced_winit::Element;
use std::convert::From;

use memory::Memory as MemoryData;
use memory::Area;
use super::widgets::hexdump;
use crate::style::Theme;

pub struct Memory {
    state: hexdump::State,
    scrollable: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum MemoryMsg {}

impl From<MemoryData> for Memory {
    fn from(memory: MemoryData) -> Self {
        let data = memory.borrow().get_area(Area::Bios);
        let state = hexdump::State::new(data);
        let scrollable = scrollable::State::new();
        Self {
            state,
            scrollable,
        }
    }
}


impl Memory {
    pub fn update(&self, _message: MemoryMsg) {}

    pub fn view(&mut self, _theme: Theme) -> Element<MemoryMsg, Renderer> {
        let hexdump = hexdump::Hexdump::new(&mut self.state);
        Scrollable::new(&mut self.scrollable).push(hexdump).into()
    }
}
