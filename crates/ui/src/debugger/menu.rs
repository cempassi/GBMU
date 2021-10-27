use super::button::Button;
use crate::style::Theme;
use iced_native::Length;
use iced_wgpu::{Renderer, Row, Space};
use iced_winit::Element;
use soc::Runner;

pub struct Menu {
    right: Vec<Button>,
    left: Vec<Button>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuMsg {
    Tick,
    Line,
    Frame,
    Cpu,
    Ppu,
}

impl Menu {
    pub fn new(runner: Runner) -> Self {
        let tick = Button::new(runner.clone(), MenuMsg::Tick);
        let line = Button::new(runner.clone(), MenuMsg::Line);
        let frame = Button::new(runner.clone(), MenuMsg::Frame);
        let cpu = Button::new(runner.clone(), MenuMsg::Cpu);
        let ppu = Button::new(runner, MenuMsg::Ppu);

        Self {
            left: vec![cpu, ppu],
            right: vec![tick, line, frame],
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
