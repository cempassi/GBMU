use super::button::Button;
use crate::style::Theme;
use iced_native::Length;
use iced_wgpu::{Renderer, Row, Space};
use iced_winit::Element;
use soc::Status;

pub struct Menu {
    right: Vec<Button>,
    left: Vec<Button>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuMsg {
    Tick,
    Line,
    Frame,
    Instruction,
}

impl Menu {
    pub fn new(status: Status) -> Self {
        let tick = Button::new(status.clone(), MenuMsg::Tick);
        let instruction = Button::new(status.clone(), MenuMsg::Instruction);
        let line = Button::new(status.clone(), MenuMsg::Line);
        let frame = Button::new(status, MenuMsg::Frame);

        Self {
            left: vec![],
            right: vec![frame, line, instruction, tick],
        }
    }

    pub fn update(&mut self, message: MenuMsg) {
        if let Some(button) = self.left.iter().find(|&button| button.is_button(message)) {
            button.update()
        };
        if let Some(button) = self.right.iter().find(|&button| button.is_button(message)) {
            button.update()
        };
    }

    pub fn view(&mut self, theme: Theme) -> Element<MenuMsg, Renderer> {
        let right: Element<MenuMsg, Renderer> = self
            .right
            .iter_mut()
            .fold(Row::new().spacing(10), |row, button| {
                let element: Element<MenuMsg, Renderer> = button.view(theme);
                row.push(element)
            })
            .into();
        let left: Element<MenuMsg, Renderer> = self
            .left
            .iter_mut()
            .fold(Row::new().spacing(10), |row, button| {
                let element: Element<MenuMsg, Renderer> = button.view(theme);
                row.push(element)
            })
            .into();
        let space = Space::with_width(Length::Fill);
        Row::new()
            .push(left)
            .push(space)
            .push(right)
            .padding(5)
            .into()
    }
}
