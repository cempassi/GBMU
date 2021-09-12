mod register;

use iced_glow::Renderer;
use iced_glutin::Clipboard;
use iced_native::{Column, Command, Element, Program, Text};

#[derive(Debug, Clone)]
pub enum Message {
    ForRegister(register::Message),
}

pub struct UserInterface {
    registers: register::Registers,
}


impl Program for UserInterface {
    type Clipboard = Clipboard;
    type Message = Message;
    type Renderer = Renderer;

    fn update(
        &mut self,
        message: Message,
        _clipboard: &mut Self::Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::ForRegister(message) => {
                self.registers.update(message);
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Message, Self::Renderer> {
        let column = Column::new()
            .push(Text::new("Hello, world! Are we doing this or what?").color([0.0, 0.0, 1.0]));

        Element::new(column)
    }
}

impl UserInterface {
    pub fn new() -> Self {
        Self {
            registers: register::Registers::default(),
        }
    }

    // fn title(&self) -> String {
    //     String::from("Hello World")
    // }
}
