use crate::bus::RegisterBus;
use crate::{GetAt, SetAt};
use memory::Memory;
use crate::{
    area::{Bits8, Bits16, Flag},
    Registers,
};

use super::consts::{BIT0, BIT7};

#[derive(Debug, Eq, PartialEq)]
pub enum CBOperation {
    RotateLeftCarry,
    RotateLeftNoCarry,
    RotateRightCarry,
    RotateRightNoCarry,
}

impl CBOperation {
    fn rotation(self, registers: &Registers, data: u8) -> u8 {
        let carry = match self {
            CBOperation::RotateLeftCarry | CBOperation::RotateLeftNoCarry  =>  (data & BIT7) != 0,
            CBOperation::RotateRightCarry | CBOperation::RotateRightNoCarry  => (data & BIT0) != 0,
        };
        let data = match self {
            CBOperation::RotateLeftCarry => (data << 1) | registers.borrow().get(Flag::C) as u8,
            CBOperation::RotateLeftNoCarry => (data << 1) | carry as u8,
            CBOperation::RotateRightCarry => {
                (data >> 1) | ((registers.borrow().get(Flag::C) as u8) << 7)
            }
            CBOperation::RotateRightNoCarry => (data >> 1) | ((carry as u8) << 7),
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
        let data = registers.clone().get_at(memory.clone(), Bits16::HL).await.unwrap();
        let data = self.rotation(&registers, data);
        registers.set_at(memory, Bits16::HL, data).await.unwrap();
    }
}
