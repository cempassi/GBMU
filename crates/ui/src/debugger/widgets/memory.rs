use super::Text;
use ascii::{AsciiChar, AsciiString, ToAsciiChar};
use iced_native::Length;
use iced_wgpu::{Column, Row, Rule};

const TEXT_SIZE: u16 = 20;

pub struct Hexdump {}

impl Hexdump {
    pub fn render<'a, Message: 'a>(data: &[u8]) -> Row<'a, Message> {
        // Calculate length of data vector
        let len = data.len();
        let lines = len / 16_usize + {
            if len % 16_usize == 0 {
                0
            } else {
                1
            }
        };

        // Address layout
        let addresses = Text::new(" ".repeat(8)).bold(TEXT_SIZE);
        let mut addresses = Column::new().height(Length::Shrink).push(addresses);

        for line in 0..lines {
            let current = Text::new(format!("{:#08X}", line * 0x10)).bold(TEXT_SIZE);
            addresses = addresses.push(current);
        }

        // Vector data layout
        let byte_header = " ".to_string() + &"FF ".repeat(16);
        let bytes = Text::new(&byte_header).bold(TEXT_SIZE);
        let mut bytes = Column::new().push(bytes).height(Length::Shrink);

        // Ascii representation layout
        let ascii_template = "0123456789ABCDEF".to_string();
        let ascii = Text::new(&ascii_template).bold(TEXT_SIZE);
        let mut ascii = Column::new().push(ascii).height(Length::Shrink);

        for line in data.chunks(16) {
            let mut byte_str = " ".to_string();
            let mut ascii_str: AsciiString = AsciiString::with_capacity(16);
            for data in line {
                byte_str.push_str(&format! {"{:02X} ", data});
                match data.to_ascii_char() {
                    Ok(char) if char.is_ascii_printable() => ascii_str.push(char),
                    _ => ascii_str.push(AsciiChar::Dot),
                }
            }
            bytes = bytes.push(Text::new(byte_str).light(TEXT_SIZE));
            ascii = ascii.push(Text::new(ascii_str.to_string()).light(TEXT_SIZE));
        }

        // Separators
        let left_separator = Rule::vertical(4);
        let right_separator = Rule::vertical(4);

        Row::new()
            .push(addresses)
            .push(left_separator)
            .push(bytes)
            .push(right_separator)
            .push(ascii)
    }
}
