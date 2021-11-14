mod button;

use std::collections::VecDeque;

use crate::style::Theme;
use button::Button;
use iced::{Element, Length, Row, Space};
use itertools::Itertools;
use soc::{mode::Mode, Status};

pub struct Menu {
    right: Vec<Button>,
    left: Vec<Button>,
    breakpoints: VecDeque<u16>,
    status: Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuMsg {
    Tick,
    Line,
    Frame,
    Instruction,
    Second,
    Breakpoint,
}

impl Menu {
    pub fn new(status: Status) -> Self {
        let tick = Button::new(status.clone(), MenuMsg::Tick);
        let instruction = Button::new(status.clone(), MenuMsg::Instruction);
        let line = Button::new(status.clone(), MenuMsg::Line);
        let frame = Button::new(status.clone(), MenuMsg::Frame);
        let second = Button::new(status.clone(), MenuMsg::Second);
        let breakpoint = Button::new(status.clone(), MenuMsg::Breakpoint);
        let breakpoints = VecDeque::new();

        Self {
            left: vec![],
            right: vec![breakpoint, second, frame, line, instruction, tick],
            breakpoints,
            status,
        }
    }

    pub fn add_breakpoints(&mut self, breakpoint: u16) {
        let index = match self
            .breakpoints
            .iter_mut()
            .find_position(|&&mut x| x == breakpoint)
        {
            Some((index, _)) => Some(index),
            _ => None,
        };
        match index {
            Some(index) => {
                println!("Removing breakpoint at {}", breakpoint);
                let _ = self.breakpoints.remove(index);
            }
            None => {
                println!("Added breakpoint at {}", breakpoint);
                self.breakpoints.push_back(breakpoint)
            }
        }
    }

    pub fn update(&mut self, message: MenuMsg) {
        match message {
            MenuMsg::Breakpoint => {
                if let Some(breakpoint) = self.breakpoints.pop_front() {
                    self.status.borrow_mut().mode(Mode::Breakpoint(breakpoint));
                }
            }
            _ => {
                if let Some(button) = self.left.iter().find(|&button| button.is_button(message)) {
                    button.update()
                };
                if let Some(button) = self.right.iter().find(|&button| button.is_button(message)) {
                    button.update()
                };
            }
        }
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
