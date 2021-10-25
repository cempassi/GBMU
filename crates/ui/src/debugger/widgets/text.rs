use crate::style::fonts;
use iced_wgpu::Text as T;

pub struct Text {
    text: String,
}

impl Text {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn bold(self, size: u16) -> T {
        T::new(self.text).font(fonts::HASKLIG_BOLD).size(size)
    }

    pub fn light(self, size: u16) -> T {
        T::new(self.text).font(fonts::HASKLIG_LIGHT).size(size)
    }
}
