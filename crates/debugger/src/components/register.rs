use iced::{Checkbox, Element};

#[derive(Default)]
pub struct Registers {
    is_checked: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    CheckboxToogled(bool),
}

impl Registers {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::CheckboxToogled(is_checked) => self.is_checked = is_checked,
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        Checkbox::new(self.is_checked, "Merge", |is_checked| {
            Message::CheckboxToogled(is_checked)
        })
        .into()
    }
}
