use super::Text;
use iced::{Container, Row};
use std::fmt::Display;

const TEXT_SIZE: u16 = 20;
const NAME_WIDTH: usize = 15;
const DATA_WIDTH: usize = 7;

pub struct Register {}

impl Register {
    pub fn render<'a, Message: 'a, Str, Data>(name: Str, data: Data) -> Row<'a, Message>
    where
        Str: Into<String> + Display,
        Data: Into<String> + Display,
    {
        let name = format!("{:<width$}", name, width = NAME_WIDTH);
        let name = Text::new(name).bold(TEXT_SIZE);

        let data = format!(": {:<width$}", data, width = DATA_WIDTH);
        let data = Text::new(data).light(TEXT_SIZE);
        let data = Container::new(data);

        Row::new().push(name).push(data)
    }
}
