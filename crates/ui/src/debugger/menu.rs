mod line;
mod tick;

use crate::style::Theme;
use iced_wgpu::{Renderer, Row};
use iced_winit::{alignment::Alignment, Element, Length, Space};
use line::Line;
use soc::Runner;
use tick::Tick;

pub struct Menu {
    line: Line,
    tick: Tick,
}

#[derive(Debug, Clone, Copy)]
pub enum MenuMsg {
    TickPressed,
    LinePressed,
}

impl Menu {
    pub fn new(runner: Runner) -> Self {
        let line = Line::new(runner.clone());
        let tick = Tick::new(runner);
        Self { tick, line }
    }
    pub fn update(&mut self, message: MenuMsg) {
        match message {
            MenuMsg::TickPressed => self.tick.update(),
            MenuMsg::LinePressed => self.line.update(),
        }
    }

    pub fn view(&mut self, theme: Theme) -> Element<MenuMsg, Renderer> {
        let line = self.line.view(theme);
        let tick = self.tick.view(theme);
        let space = Space::new(Length::Units(35), Length::Units(0));
        Row::new()
            .align_items(Alignment::End)
            .width(Length::Fill)
            .padding(5)
            .push(tick)
            .push(space)
            .push(line)
            .into()
    }
}
