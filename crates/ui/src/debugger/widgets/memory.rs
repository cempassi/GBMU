use super::Text;
use crate::debugger::memory::MemoryMsg;
use ascii::{AsciiChar, AsciiString, ToAsciiChar};
use iced::{scrollable, Column, Length, Row, Scrollable};

const TEXT_SIZE: u16 = 20;

pub struct Hexdump<T> {
    name: String,
    state: scrollable::State,
    data: T,
}

impl<T> Hexdump<T> {
    pub fn new(name: String, data: T) -> Self {
        let state = scrollable::State::default();
        Self { name, state, data }
    }

    pub fn title(&self) -> iced_wgpu::Text {
        Text::new(self.name.clone()).bold(10)
    }

    pub fn _name(&self) -> String {
        self.name.clone()
    }

    pub fn render(&mut self, data: &[u8]) -> Column<MemoryMsg> {
        // Address layout
        let addresses = Text::new(" ".repeat(8)).bold(TEXT_SIZE);

        // Vector data layout
        let byte_header = " ".to_string() + &"FF ".repeat(16);
        let bytes = Text::new(&byte_header).bold(TEXT_SIZE);

        // Ascii representation layout
        let ascii_template = "0123456789ABCDEF".to_string();
        let ascii = Text::new(&ascii_template).bold(TEXT_SIZE);

        let title = Row::new().push(addresses).push(bytes).push(ascii);

        let mut hexdump = Scrollable::new(&mut self.state)
            .width(Length::Shrink)
            .height(Length::Shrink);

        for (i, line) in data.chunks(16).enumerate() {
            let mut byte_str = " ".to_string();
            let mut ascii_str: AsciiString = AsciiString::with_capacity(16);
            let mut row = Row::new();
            for data in line {
                byte_str.push_str(&format! {"{:02X} ", data});
                match data.to_ascii_char() {
                    Ok(char) if char.is_ascii_printable() => ascii_str.push(char),
                    _ => ascii_str.push(AsciiChar::Dot),
                }
            }
            let line = format!("{:#08X}", i * 0x10) + &byte_str + &ascii_str.to_string();
            row = row.push(Text::new(line).light(TEXT_SIZE));
            hexdump = hexdump.push(row);
        }

        Column::new().push(title).push(hexdump)
    }

    /// Get a reference to the hexdump's data.
    pub fn data(&self) -> &T {
        &self.data
    }
}
