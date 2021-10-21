use crate::style::{fonts, Theme};
use iced_wgpu::{button, Button, Renderer, Row, Text};
use iced_winit::{alignment::Alignment, Element, Length, Space};

use super::MenuMsg;
use soc::Runner;

pub struct Line {
    runner: Runner,
    state: button::State,
}

impl Line {
    pub fn new(runner: Runner) -> Self {
        let state = button::State::default();
        Self { runner, state }
    }

    pub fn update(&mut self) {
        self.runner.borrow_mut().line();
    }

    pub fn view(&mut self, theme: Theme) -> Element<MenuMsg, Renderer> {
        let tick_text = Text::new("Line").font(fonts::HASKLIG_LIGHT).size(20);
        let next_tick = Button::new(&mut self.state, tick_text)
            .on_press(MenuMsg::LinePressed)
            .padding(10)
            .style(theme);
        let space = Space::new(Length::Units(35), Length::Units(0));
        Row::new()
            .align_items(Alignment::End)
            .width(Length::Fill)
            .padding(5)
            .push(space)
            .push(next_tick)
            .into()
    }
}
