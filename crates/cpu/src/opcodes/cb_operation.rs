use crate::bus::RegisterBus;
use crate::{
    area::{Bits16, Bits8, Flag},
    Registers,
};
use crate::{GetAt, SetAt};
use memory::Memory;

use super::consts::{BIT0, BIT7};

#[derive(Debug, Eq, PartialEq)]
pub enum Rotation {
    Left,
    LeftNoCarry,
    Right,
    RightNoCarry,
}

impl Rotation {
    fn rotation(self, registers: &Registers, data: u8) -> u8 {
        let carry = match self {
            Rotation::Left | Rotation::LeftNoCarry => (data & BIT7) != 0,
            Rotation::Right | Rotation::RightNoCarry => (data & BIT0) != 0,
        };
        let data = match self {
            Rotation::Left => (data << 1) | registers.borrow().get(Flag::C) as u8,
            Rotation::LeftNoCarry => (data << 1) | carry as u8,
            Rotation::Right => (data >> 1) | ((registers.borrow().get(Flag::C) as u8) << 7),
            Rotation::RightNoCarry => (data >> 1) | ((carry as u8) << 7),
        };
        registers.borrow_mut().set(Flag::C, carry);
        if data == 0 {
            registers.borrow_mut().set(Flag::Z, true);
        };
        data
    }

    pub fn rotate(self, registers: Registers, area: Bits8) {
        let data = registers.borrow().get(area);
        let data = self.rotation(&registers, data);
        registers.borrow_mut().set(area, data);
    }

    pub async fn rotate_hl(self, registers: Registers, memory: Memory) {
        let data = registers
            .clone()
            .get_at(memory.clone(), Bits16::HL)
            .await
            .unwrap();
        let data = self.rotation(&registers, data);
        registers.set_at(memory, Bits16::HL, data).await.unwrap();
    }
}
