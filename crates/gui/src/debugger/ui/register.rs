use iced_wgpu::{Checkbox, Renderer, Row, Text};
use iced_winit::Element;

#[derive(Default)]
pub struct Registers {
    theme: crate::theme::Theme,
    is_checked: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    CheckboxToogled(bool),
}

impl Registers {
    pub fn update(&mut self, message: Message) {
        println!("Update of Registers reached");
        match message {
            Message::CheckboxToogled(is_checked) => self.is_checked = is_checked,
        }
        println!("I'm in the update function of the register");
    }

    pub fn view(&mut self) -> Element<Message, Renderer> {
        let checkbox = Checkbox::new(self.is_checked, "Merge", |is_checked| {
            Message::CheckboxToogled(is_checked)
        })
        .style(self.theme);

        let hello_world =
            Text::new("Hello, world! Are we doing this or what?").color([0.0, 0.0, 1.0]);
        Row::new().push(checkbox).push(hello_world).into()
    }
}
