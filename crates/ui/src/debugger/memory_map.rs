//mod memory;
use crate::style::Theme;
use super::widgets::hexdump;
use iced_wgpu::{scrollable, Renderer, Scrollable};
use iced_winit::Element;

const LOREM_IPSUM: &[u8] = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed \
                             interdum massa interdum gravida gravida. Nam ullamcorper.";

pub struct Memory {
    hexdump: hexdump::State,
    scrollable: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum MemoryMsg {}

impl Memory {
    pub fn new() -> Self {
        let mut hexdump = hexdump::State::default();
        hexdump.load(LOREM_IPSUM);
        let scrollable = scrollable::State::new();

        Self {
            hexdump,
            scrollable,
        }
    }

    pub fn update(&self, _message: MemoryMsg) {}

    pub fn view(&mut self, _theme: Theme) -> Element<MemoryMsg, Renderer> {
        let hexdump = hexdump::Hexdump::new(&mut self.hexdump);
        Scrollable::new(&mut self.scrollable).push(hexdump).into()
    }
}
