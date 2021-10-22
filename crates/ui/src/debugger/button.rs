use crate::style::{fonts, Theme};
use iced_wgpu::{button, Renderer, Text};
use iced_winit::{Element, Length};

use super::menu::MenuMsg;
use soc::Runner;

pub struct Button {
    runner: Runner,
    message: MenuMsg,
    state: button::State,
}

impl Button {
    pub fn new(runner: Runner, message: MenuMsg) -> Self {
        let state = button::State::default();
        Self {
            runner,
            state,
            message,
        }
    }

    pub fn update(&self) {
        match self.message {
            MenuMsg::Tick => {
                self.runner.borrow_mut().tick();
            }
            MenuMsg::Line => {
                self.runner.borrow_mut().line();
            }
            MenuMsg::Frame => {
                self.runner.borrow_mut().frame();
            }
        }
    }

    pub fn is_button(&self, message: MenuMsg) -> bool {
        self.message == message
    }

    pub fn view(&mut self, theme: Theme) -> Element<MenuMsg, Renderer> {
        let text = Text::new(format!("{:?}", self.message))
            .font(fonts::HASKLIG_LIGHT)
            .size(20);
        button::Button::new(&mut self.state, text)
            .on_press(self.message)
            .width(Length::Fill)
            .style(theme)
            .into()
    }
}
