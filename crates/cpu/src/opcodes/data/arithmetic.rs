use crate::opcodes::data::Data;
use crate::Flags;

fn carry(shift: usize, nbr1: usize, nbr2: usize, c: usize) -> bool {
    let max = (1 << shift) - 1;
    (nbr1 & max) + (nbr2 & max) + (c & max) > max
}

fn borrow(shift: usize, nbr1: usize, nbr2: usize, c: usize) -> bool {
    let max = (1 << shift) - 1;
    (nbr1 & max) < (nbr2 & max) + (c & max)
}

pub trait Sub<T> {
    type Output;
    fn sub(&self, data: T) -> Self::Output;
}

impl Sub<u8> for Data<u8> {
    type Output = u16;
    fn sub(&self, nbr: u8) -> Self::Output {
        let (value, c) = match self {
            Data::Carry(value) => (*value as usize, 1),
            Data::NoCarry(value) => (*value as usize, 0),
        };
        let max: usize = (1 << 8) - 1;
        let data = (value.wrapping_sub(nbr as usize).wrapping_sub(c)) & max;
        let h = borrow(4, value, nbr as usize, c as usize);
        let c = borrow(8, value, nbr as usize, c as usize);
        let z = data == 0;
        let mut flag = Flags::default();
        flag.set_z(z);
        flag.set_n(true);
        flag.set_h(h);
        flag.set_c(c);
        (data as u16) << 8 | Flags::into_bytes(flag)[0] as u16
    }
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
        let max: usize = (1 << 8) - 1;
        let data = (value + nbr as usize + c as usize) & max;
        let h = carry(4, value, nbr as usize, c as usize);
        let c = carry(8, value, nbr as usize, c as usize);
        let z = data == 0;
        let mut flag = Flags::default();
        flag.set_z(z);
        flag.set_h(h);
        flag.set_c(c);
        (data as u16) << 8 | Flags::into_bytes(flag)[0] as u16
    }
}

impl Add<u16> for Data<u16> {
    type Output = (u16, Flags);
    fn add(&self, nbr: u16) -> Self::Output {
        let (value, c) = match self {
            Data::Carry(value) => (*value as usize, 1),
            Data::NoCarry(value) => (*value as usize, 0),
        };
        let max: usize = (1 << 16) - 1;
        let data = (value + nbr as usize + c) & max;
        let h = carry(12, value, nbr as usize, c as usize);
        let c = carry(16, value, nbr as usize, c as usize);
        let z = data == 0;
        let mut flag = Flags::default();
        flag.set_z(z);
        flag.set_h(h);
        flag.set_c(c);
        (data as u16, flag)
    }
}

#[cfg(test)]
mod test_arithmetics_functions {
    use crate::opcodes::data::arithmetic::{Add, Sub};
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

    #[test]
    fn test_sub_no_carry() {
        let data: Data<u8> = Data::NoCarry(0x12);
        assert_eq!(data.sub(0x10), 0x0220);
    }

    #[test]
    fn test_sub_carry() {
        let data: Data<u8> = Data::Carry(0x34);
        assert_eq!(data.sub(0x22), 0x1120);
    }

    #[test]
    fn test_sub_no_carry_h_flag() {
        let data: Data<u8> = Data::NoCarry(0x32);
        assert_eq!(data.sub(0x2f), 0x0360);
    }

    #[test]
    fn test_sub_carry_h_flag() {
        let data: Data<u8> = Data::Carry(0x32);
        assert_eq!(data.sub(0x2e), 0x0360);
    }

    #[test]
    fn test_sub_no_carry_c_flag() {
        let data: Data<u8> = Data::NoCarry(0x12);
        assert_eq!(data.sub(0xf0), 0x22a0);
    }

    #[test]
    fn test_sub_carry_c_flag() {
        let data: Data<u8> = Data::Carry(0x12);
        assert_eq!(data.sub(0xe0), 0x31a0);
    }

    #[test]
    fn test_sub_no_carry_all_flags() {
        let data: Data<u8> = Data::NoCarry(0x12);
        assert_eq!(data.sub(0x12), 0x0030);
    }

    #[test]
    fn test_sub_carry_all_flags() {
        let data: Data<u8> = Data::Carry(0x88);
        assert_eq!(data.sub(0x87), 0x0030);
    }
}