use crate::style::fonts;
use iced_wgpu::Text as T;

pub struct Text<'a> {
    text: &'a str,
}

impl<'a> Text<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    pub fn bold(self, size: u16) -> T {
        T::new(self.text).font(fonts::HASKLIG_BOLD).size(size)
    }

    pub fn light(self, size: u16) -> T {
        T::new(self.text).font(fonts::HASKLIG_LIGHT).size(size)
    }

    pub fn medium(self, size: u16) -> T {
        T::new(self.text).font(fonts::HASKLIG_MEDIUM).size(size)
    }
    pub fn medium_it(self, size: u16) -> T {
        T::new(self.text).font(fonts::HASKLIG_MEDIUM_IT).size(size)
    }
}
