use crate::fonts;
use crate::theme::Theme;
use iced_wgpu::{Container, Renderer, Row, Text};
use iced_winit::Element;

#[derive(Default)]
pub struct Registers {
    theme: Theme,
    is_checked: bool,
}

#[derive(Debug, Clone)]
pub enum Message {}

impl Registers {
    pub fn update(&mut self, message: Message) {}

    pub fn view(&mut self) -> Element<Message, Renderer> {
        let number = format!("{:#x}", 256);
        let text = Text::new(number).font(fonts::HASKLIG_LIGHT);
        Container::new(text).style(self.theme).padding(5).into()
    }
}
