use crate::Flags;

fn carry(shift: usize, nbr1: usize, nbr2: usize, c: usize) -> bool {
    let c = c as usize;
    let max = (1 << shift) - 1;
    (nbr1 & max) + (nbr2 & max) + (c & max) > max
}

pub(crate) enum Data<T> {
    Carry(T),
    NoCarry(T),
}

impl Add<u8> for Data<u8> {
    fn add(&self, nbr: u8) -> (u8, Flags) {
        let (data, c) = match self {
            Data::Carry(data) => (*data as usize, 1),
            Data::NoCarry(data) => (*data as usize, 0),
        };
        let max: usize = (1 << 8) - 1;
        let res = (data + nbr as usize + c) & max;
        let h = carry(4, data, nbr as usize, c as usize);
        let c = carry(8, data, nbr as usize, c as usize);
        let z = res == 0;
        let mut flag = Flags::default();
        flag.set_z(z);
        flag.set_h(h);
        flag.set_c(c);
        (res as u8, flag)
    }
}

pub trait Add<T> {
    fn add(&self, data: T) -> (T, Flags);
}

impl Add<u16> for Data<u16> {
    fn add(&self, nbr: u16) -> (u16, Flags) {
        let (data, c) = match self {
            Data::Carry(data) => (*data as usize, 1),
            Data::NoCarry(data) => (*data as usize, 0),
        };
        let max: usize = (1 << 16) - 1;
        let res = (data + nbr as usize + c) & max;
        let h = carry(12, data, nbr as usize, c as usize);
        let c = carry(16, data, nbr as usize, c as usize);
        let z = res == 0;
        let mut flag = Flags::default();
        flag.set_z(z);
        flag.set_h(h);
        flag.set_c(c);
        (res as u16, flag)
    }
}

#[cfg(test)]
mod test_arithmetics_functions {
    use super::Add;
    use super::Data;
    use crate::Flags;

    #[test]
    fn test_add_8b() {
        let flag = Flags::default();
        let data: Data<u8> = Data::NoCarry(0x12);
        assert_eq!(data.add(0x22), (0x34, flag));
    }

    #[test]
    fn test_add_with_carry_8b() {
        let flag = Flags::default();
        let data: Data<u8> = Data::Carry(0x12);
        assert_eq!(data.add(0x22), (0x35, flag));
    }

    #[test]
    fn test_add_8b_h_flag() {
        let mut flag = Flags::default();
        let data: Data<u8> = Data::NoCarry(0x12);
        flag.set_h(true);
        assert_eq!(data.add(0x2f), (0x41, flag));
    }

    #[test]
    fn test_add_8b_c_flag() {
        let mut flag = Flags::default();
        let data: Data<u8> = Data::NoCarry(0x12);
        flag.set_c(true);
        assert_eq!(data.add(0xf0), (0x02, flag));
    }

    #[test]
    fn test_add_with_carry_8b_c_flag() {
        let mut flag = Flags::default();
        let data: Data<u8> = Data::Carry(0x12);
        flag.set_c(true);
        assert_eq!(data.add(0xf0), (0x03, flag));
    }

    #[test]
    fn test_add_with_carry_8b_h_flag() {
        let mut flag = Flags::default();
        let data: Data<u8> = Data::Carry(0x12);
        flag.set_h(true);
        assert_eq!(data.add(0x2f), (0x42, flag));
    }

    #[test]
    fn test_add_8b_h_c_flag() {
        let mut flag = Flags::default();
        let data: Data<u8> = Data::NoCarry(0x0a);
        flag.set_h(true);
        flag.set_c(true);
        assert_eq!(data.add(0xfa), (0x04, flag));
    }

    #[test]
    fn test_add_with_carry_8b_h_c_flag() {
        let mut flag = Flags::default();
        let data: Data<u8> = Data::Carry(0x0a);
        flag.set_h(true);
        flag.set_c(true);
        assert_eq!(data.add(0xfa), (0x05, flag));
    }

    #[test]
    fn test_add_8b_z_flag() {
        let mut flag = Flags::default();
        let data: Data<u8> = Data::NoCarry(0x00);
        flag.set_z(true);
        assert_eq!(data.add(0x00), (0x00, flag));
    }

    #[test]
    fn test_add_8b_z_c_flag() {
        let mut flag = Flags::default();
        let data: Data<u8> = Data::NoCarry(0x20);
        flag.set_c(true);
        flag.set_z(true);
        assert_eq!(data.add(0xe0), (0x00, flag));
    }

    #[test]
    fn test_add_8b_all_flags() {
        let mut flag = Flags::default();
        let data: Data<u8> = Data::NoCarry(0x08);
        flag.set_z(true);
        flag.set_c(true);
        flag.set_h(true);
        assert_eq!(data.add(0xf8), (0x00, flag));
    }

    #[test]
    fn test_add_with_carry_8b_all_flags() {
        let mut flag = Flags::default();
        let data: Data<u8> = Data::Carry(0x07);
        flag.set_z(true);
        flag.set_c(true);
        flag.set_h(true);
        assert_eq!(data.add(0xf8), (0x00, flag));
    }
}
