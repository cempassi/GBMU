use iced::Checkbox;

pub struct Register {}

pub enum Message {
    CheckboxToogled(bool),
}

impl Register {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&mut self) -> Checkbox<Message> {
        Checkbox::new(false, "Merge", Message::CheckboxToogled).into()
    }
}
