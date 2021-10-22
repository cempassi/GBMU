use super::button::Button;
use crate::style::Theme;
use iced_wgpu::{Renderer, Row};
use iced_winit::{Alignment, Element, Length, Space};
use soc::Runner;

pub struct Menu {
    buttons: Vec<Button>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuMsg {
    Tick,
    Line,
    Frame,
}

impl Menu {
    pub fn new(runner: Runner) -> Self {
        let tick = Button::new(runner.clone(), MenuMsg::Tick);
        let line = Button::new(runner.clone(), MenuMsg::Line);
        let frame = Button::new(runner, MenuMsg::Frame);

        Self {
            buttons: vec![tick, line, frame],
        }
    }

    pub fn update(&mut self, message: MenuMsg) {
        if let Some(button) = self
            .buttons
            .iter()
            .find(|&button| button.is_button(message))
        {
            button.update()
        };
    }

    pub fn view(&mut self, theme: Theme) -> Element<MenuMsg, Renderer> {
        self.buttons
            .iter_mut()
            .fold(
                Row::new()
                    .align_items(Alignment::End)
                    .width(Length::Fill)
                    .padding(5),
                |row, button| {
                    let space = Space::new(Length::Units(10), Length::Shrink);
                    let element: Element<MenuMsg, Renderer> = button.view(theme);
                    row.push(element).push(space)
                },
            )
            .into()
    }
}
