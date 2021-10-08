use crate::opcodes::consts::{BIT3_MINUS_1, BIT7_MINUS_1};
use crate::opcodes::data::Data;
use crate::Flags;

pub fn signed(value: u8) -> u16 {
    if value & 0x80 != 0 {
        0xff00 | value as u16
    } else {
        value as u16
    }
}

fn carry(value: usize, nbr: usize, c: usize, max_c: usize, max_h: usize) -> (usize, Flags) {
    let data = (value + nbr as usize + c as usize) & max_c;
    let mut flag = Flags::default();
    flag.set_z(data == 0);
    flag.set_h((value & max_h) + (nbr & max_h) + (c & max_h) > max_h);
    flag.set_c((value & max_c) + (nbr & max_c) + (c & max_c) > max_c);
    (data, flag)
}

pub trait Add<T> {
    type Output;
    fn add(&self, data: T) -> Self::Output;
}

impl Add<u8> for Data<u8> {
    type Output = u16;
    fn add(&self, nbr: u8) -> Self::Output {
        let (value, c) = match self {
            Data::Carry(value) => (*value as usize, 1),
            Data::NoCarry(value) => (*value as usize, 0),
        };
        let (data, flag) = carry(value, nbr as usize, c, BIT7_MINUS_1, BIT3_MINUS_1);
        (data as u16) << 8 | Flags::into_bytes(flag)[0] as u16
    }
}

#[cfg(test)]
mod test_arithmetics_functions {
    use crate::opcodes::data::arithmetic::Add;
    use crate::opcodes::data::Data;

    #[test]
    fn test_add_8b() {
        let data: Data<u8> = Data::NoCarry(0x12);
        assert_eq!(data.add(0x22), 0x3400);
    }

    #[test]
    fn test_add_with_carry_8b() {
        let data: Data<u8> = Data::Carry(0x12);
        assert_eq!(data.add(0x22), 0x3500);
    }

    #[test]
    fn test_add_8b_h_flag() {
        let data: Data<u8> = Data::NoCarry(0x12);
        assert_eq!(data.add(0x2f), 0x4140);
    }

    #[test]
    fn test_add_8b_c_flag() {
        let data: Data<u8> = Data::NoCarry(0x12);
        assert_eq!(data.add(0xf0), 0x0280);
    }

    #[test]
    fn test_add_with_carry_8b_c_flag() {
        let data: Data<u8> = Data::Carry(0x12);
        assert_eq!(data.add(0xf0), 0x0380);
    }

    #[test]
    fn test_add_with_carry_8b_h_flag() {
        let data: Data<u8> = Data::Carry(0x12);
        assert_eq!(data.add(0x2f), 0x4240);
    }

    #[test]
    fn test_add_8b_h_c_flag() {
        let data: Data<u8> = Data::NoCarry(0x0a);
        assert_eq!(data.add(0xfa), 0x04c0);
    }

    #[test]
    fn test_add_with_carry_8b_h_c_flag() {
        let data: Data<u8> = Data::Carry(0x0a);
        assert_eq!(data.add(0xfa), 0x05c0);
    }

    #[test]
    fn test_add_8b_z_flag() {
        let data: Data<u8> = Data::NoCarry(0x00);
        assert_eq!(data.add(0x00), 0x0010);
    }

    #[test]
    fn test_add_8b_z_c_flag() {
        let data: Data<u8> = Data::NoCarry(0x20);
        assert_eq!(data.add(0xe0), 0x0090);
    }

    #[test]
    fn test_add_8b_all_flags() {
        let data: Data<u8> = Data::NoCarry(0x08);
        assert_eq!(data.add(0xf8), 0x00d0);
    }

    #[test]
    fn test_add_with_carry_8b_all_flags() {
        let data: Data<u8> = Data::Carry(0x07);
        assert_eq!(data.add(0xf8), 0x00d0);
    }
}
