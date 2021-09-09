use iced::{Column, Element, Sandbox, Text};

pub struct Debugger {}

#[derive(Debug, Clone, Copy)]
pub enum Message {}

impl Sandbox for Debugger{
    type Message = Message;

    fn update(&mut self, _message: Message) {
        //
    }

    fn view(&mut self) -> Element<Self::Message> {
        Column::new()
            .push(Text::new("Hello, world!")).into()
    }

    fn new() -> Self {
        Self {}
    }

    fn title(&self) -> String {
        String::from("Hello World")
    }
}
