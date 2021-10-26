use super::Text;
use iced_wgpu::Row;

pub struct Cell {}

impl Cell {
    pub fn bold<'a, T: 'a>(text: String, size: u16) -> Row<'a, T> {
        let text = Text::new(&text).bold(size);
        Row::new().push(text)
    }

    pub fn light<'a, T: 'a>(text: String, size: u16) -> Row<'a, T> {
        let text = Text::new(&text).light(size);
        Row::new().push(text)
    }
}
