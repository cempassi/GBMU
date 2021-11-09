use super::Text;
use iced_wgpu::{Column, Row, Rule};

pub struct Memory {}

impl Memory {
    pub fn render<'a, Message: 'a>(data: &Vec<u8>) -> Row<'a, Message> {
        // Calculate length of data vector
        let len = data.len();
        let lines = len / 16 as usize + {
            if len % 16 as usize == 0 {
                0
            } else {
                1
            }
        };

        // Address layout
        let addresses = [0..lines + 1].iter().fold(Column::new(), |addresses, _| {
            let current = Text::new("00000000").bold(10);
            addresses.push(current)
        });

        // Vector data layout
        let data_template = "FF ".repeat(16);
        let data = Text::new(&data_template).bold(10);
        let data = Column::new().push(data);

        let data = [0..lines + 1].iter().fold(data, |data, _| {
            let current = Text::new(&data_template).light(10);
            data.push(current)
        });

        // Ascii representation layout
        let ascii_template = "F".repeat(16);
        let ascii = Text::new(&ascii_template).bold(10);
        let ascii = Column::new().push(ascii);
        let ascii = [0..lines + 1].iter().fold(ascii, |ascii, _| {
            let current = Text::new(&data_template).light(10);
            ascii.push(current)
        });

        // Separators
        let left_separator = Rule::vertical(4);
        let right_separator = Rule::vertical(4);

        Row::new()
            .push(addresses)
            .push(left_separator)
            .push(data)
            .push(right_separator)
            .push(ascii)
    }
}
