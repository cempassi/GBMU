use crate::style::Theme;
use iced::{button, Element};

use super::MenuMsg;
use crate::debugger::widgets::Text;
use soc::Status;

pub struct Button {
    status: Status,
    message: MenuMsg,
    state: button::State,
}

impl Button {
    pub fn new(status: Status, message: MenuMsg) -> Self {
        let state = button::State::default();
        Self {
            status,
            state,
            message,
        }
    }

    pub fn update(&self) {
        match self.message {
            MenuMsg::Tick => {
                self.status.borrow_mut().tick();
            }
            MenuMsg::Line => {
                self.status.borrow_mut().line();
            }
            MenuMsg::Frame => {
                self.status.borrow_mut().frame();
            }
            MenuMsg::Instruction => {
                self.status.borrow_mut().instruction();
            }
        }
    }

    pub fn is_button(&self, message: MenuMsg) -> bool {
        self.message == message
    }

    pub fn view(&mut self, theme: Theme) -> Element<MenuMsg> {
        let text = format!("{:?}", self.message);
        let text = format!("{:^10}", text);
        let text = Text::new(&text).medium(20);
        button::Button::new(&mut self.state, text)
            .on_press(self.message)
            .style(theme)
            .into()
    }
}
