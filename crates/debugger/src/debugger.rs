use crate::components::register;
use iced::{Column, Element, Sandbox, Text};

pub struct Debugger {
    registers: register::Registers,
}

#[derive(Debug, Clone)]
pub enum Message {
    ForRegister(register::Message),
}

impl Sandbox for Debugger {
    type Message = Message;

    fn update(&mut self, message: Message) {
        match message {
            Message::ForRegister(message) => self.registers.update(message),
        }
    }

    fn view(&mut self) -> Element<Message> {
        let column = Column::new().push(Text::new("Hello, world!")).push(
            self.registers
                .view()
                .map(move |message| Message::ForRegister(message)),
        );

        Element::new(column)
    }

    fn new() -> Self {
        Self {
            registers: register::Registers::default(),
        }
    }

    fn title(&self) -> String {
        String::from("Hello World")
    }
}
