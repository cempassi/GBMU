use super::Text;
use iced::Row;

pub struct Cell {}

impl Cell {
    pub fn bold<'a, Message: 'a>(text: String, size: u16) -> Row<'a, Message> {
        let text = Text::new(&text).bold(size);
        Row::new().push(text)
    }

    pub fn light<'a, Message: 'a>(text: String, size: u16) -> Row<'a, Message> {
        let text = Text::new(&text).light(size);
        Row::new().push(text)
    }
}
