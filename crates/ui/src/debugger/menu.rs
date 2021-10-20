use crate::style::{fonts, Theme};
use iced_wgpu::{button, Button, Renderer, Row, Text};
use iced_winit::{alignment::Alignment, Element, Length, Space};
use soc::Runner;

pub struct Menu {
    runner: Runner,
    next_tick: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum MenuMsg {
    TickPressed,
}

impl Menu {
    pub fn new(runner: Runner) -> Self {
        let next_tick = button::State::default();
        Self { runner, next_tick }
    }
    pub fn update(&mut self, message: MenuMsg) {
        match message {
            MenuMsg::TickPressed => self.runner.borrow_mut().tick(),
        }
    }

    pub fn view(&mut self, theme: Theme) -> Element<MenuMsg, Renderer> {
        let tick_text = Text::new("Tick").font(fonts::HASKLIG_LIGHT).size(20);
        let next_tick = Button::new(&mut self.next_tick, tick_text)
            .on_press(MenuMsg::TickPressed)
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
