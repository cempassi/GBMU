use iced::{Element, Row};

use super::DisassMsg;
use crate::debugger::widgets::Cell;

pub struct Header {}

impl Header {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&mut self) -> Element<DisassMsg> {
        let address = Cell::bold(format!("{:^10}", "Address"), 20);
        let instruction = Cell::bold(format!("{:^26}", "Instruction"), 20);
        let opcode = Cell::bold(format!("{:^6}", "Opcode"), 20);
        let cycles = Cell::bold(format!("{:^12}", "Cycles"), 20);
        let data = Cell::bold(format!("{:^6}", "Data"), 20);
        let row = Row::new();
        row.push(address)
            .push(instruction)
            .push(opcode)
            .push(cycles)
            .push(data)
            .into()
    }
}
