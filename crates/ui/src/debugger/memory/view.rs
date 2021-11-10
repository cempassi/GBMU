use iced::{alignment::{Horizontal, Vertical}, Length, Container, Column, Text};
use memory::{Bus, Rom};
use ppu::Ppu;
use iced_aw::TabLabel;
use crate::debugger::widgets::memory::Hexdump;
use super::MemoryMsg;
use crate::style::Theme;

const TAB_PADDING: u16 = 16;

pub trait View {

    fn title(&self) -> Text;

    fn tab_label(&self) -> TabLabel;

    fn view(&mut self, theme: Theme) -> Container<MemoryMsg>{
        let column = Column::new()
            .spacing(20)
            .push(self.title())
            .push(self.content(theme));

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(TAB_PADDING)
    }

    fn content(&mut self, theme: Theme) -> Column<MemoryMsg>;
}

impl View for Hexdump<Bus> {
    fn content(&mut self, _theme: crate::style::Theme) -> Column<MemoryMsg> {
        let data = self.data().clone();
        let rc = data.borrow();
        let vector = rc.as_ref().as_ref();
        self.render(vector)
    }

    fn title(&self) -> Text {
        self.title()
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text(self.name())
    }
}

impl View for Hexdump<Rom> {
    fn content(&mut self, _theme: crate::style::Theme) -> Column<MemoryMsg> {
        let data = self.data().clone();
        let rc = data.borrow();
        let vector = rc.as_ref().as_ref();
        self.render(vector)
    }

    fn title(&self) -> Text {
        self.title()
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text(self.name())
    }
}

impl View for Hexdump<Ppu> {
    fn content(&mut self,_theme: crate::style::Theme) -> Column<MemoryMsg> {
        let data = self.data().clone();
        let rc = data.borrow();
        let vector: &Vec<u8> = rc.as_ref();
        self.render(vector)
    }

    fn title(&self) -> Text {
        self.title()
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text(self.name())
    }
}
