mod button;

use crate::style::Theme;
use button::Button;
use iced::{Element, Length, Row, Space};
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
    Second,
}

impl Menu {
    pub fn new(status: Status) -> Self {
        let tick = Button::new(status.clone(), MenuMsg::Tick);
        let instruction = Button::new(status.clone(), MenuMsg::Instruction);
        let line = Button::new(status.clone(), MenuMsg::Line);
        let frame = Button::new(status.clone(), MenuMsg::Frame);
        let second = Button::new(status, MenuMsg::Second);

        Self {
            left: vec![],
            right: vec![second, frame, line, instruction, tick],
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

    pub fn view(&mut self, theme: Theme) -> Element<MenuMsg> {
        let right: Element<MenuMsg> = self
            .right
            .iter_mut()
            .fold(Row::new().spacing(10), |row, button| {
                let element: Element<MenuMsg> = button.view(theme);
                row.push(element)
            })
            .into();
        let left: Element<MenuMsg> = self
            .left
            .iter_mut()
            .fold(Row::new().spacing(10), |row, button| {
                let element: Element<MenuMsg> = button.view(theme);
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
