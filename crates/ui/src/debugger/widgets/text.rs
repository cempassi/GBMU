use crate::style::fonts;
use iced::Text as T;

pub struct Text<Name: Into<String>> {
    text: Name,
}

impl<Name> Text<Name>
where
    Name: Into<String>,
{
    pub fn new(text: Name) -> Self {
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
