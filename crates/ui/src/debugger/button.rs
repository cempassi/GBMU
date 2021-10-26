use crate::style::Theme;
use iced_wgpu::{button, Renderer};
use iced_winit::Element;

use super::menu::MenuMsg;
use super::widgets::Text;
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
            MenuMsg::Ppu => {
                self.runner.borrow_mut().ppu();
            }
            MenuMsg::Cpu => {
                self.runner.borrow_mut().cpu();
            }
        }
    }

    pub fn is_button(&self, message: MenuMsg) -> bool {
        self.message == message
    }

    pub fn view(&mut self, theme: Theme) -> Element<MenuMsg, Renderer> {
        let text = format!("{:?}", self.message);
        let text = format!("{:^10}", text);
        let text = Text::new(&text).medium(20);
        button::Button::new(&mut self.state, text)
            .on_press(self.message)
            .style(theme)
            .into()
    }
}
