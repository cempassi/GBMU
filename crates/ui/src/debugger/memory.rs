mod bios;

use iced_wgpu::Renderer;
use iced_winit::Element;

use crate::style::Theme;
use memory::Memory as MemoryData;

pub struct Memory {
    data: MemoryData,
}

pub trait View<Msg> {
    fn view(self, theme: Theme) -> Element<'static, Msg, Renderer>;
}

#[derive(Debug, Clone)]
pub enum MemoryMsg {}

impl Memory {
    pub fn new(data: MemoryData) -> Self {
        Self { data }
    }

    pub fn update(&self, _message: MemoryMsg) {}

    pub fn view(&mut self, theme: Theme) -> Element<MemoryMsg, Renderer> {
        let bios = self.data.borrow().get_area(memory::Area::Bios);
        bios.view(theme)
    }
}
