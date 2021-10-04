use crate::Flags;

fn carry(shift: usize, nbr1: usize, nbr2: usize, c: usize) -> bool {
    let c = c as usize;
    let max = (1 << shift) - 1;
    (nbr1 & max) + (nbr2 & max) + (c & max) > max
}

pub trait Add<T> {
    fn add(self, nbr2: usize, c: bool) -> (T, Flags);
}

impl Add<u8> for u8 {
    fn add(self, nbr2: usize, c: bool) -> (u8, Flags) {
        let c = c as usize;
        let max: usize = (1 << 8) - 1;
        let data = (self as usize + nbr2 + c) & max;
        let h = carry(4, self as usize, nbr2, c);
        let c = carry(8, self as usize, nbr2, c);
        let z = data == 0;
        let mut flag = Flags::default();
        flag.set_z(z);
        flag.set_h(h);
        flag.set_c(c);
        (data as u8, flag)
    }
}

impl Add<u16> for u16 {
    fn add(self, nbr2: usize, c: bool) -> (u16, Flags) {
        let c = c as usize;
        let max: usize = (1 << 16) - 1;
        let data = (self as usize + nbr2 + c) & max;
        let h = carry(12, self as usize, nbr2, c);
        let c = carry(16, self as usize, nbr2, c);
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
    use super::Add;
    use crate::Flags;

    #[test]
    fn test_add_8b() {
        let flag = Flags::default();
        let data: u8 = 0x12;
        assert_eq!(data.add(0x22, false), (0x34, flag));
    }

    #[test]
    fn test_adc_8b() {
        let flag = Flags::default();
        let data: u8 = 0x12;
        assert_eq!(data.add(0x22, true), (0x35, flag));
    }

    #[test]
    fn test_add_8b_h_flag() {
        let mut flag = Flags::default();
        let data: u8 = 0x12;
        flag.set_h(true);
        assert_eq!(data.add(0x2f, false), (0x41, flag));
    }

    #[test]
    fn test_add_8b_c_flag() {
        let mut flag = Flags::default();
        let data: u8 = 0x12;
        flag.set_c(true);
        assert_eq!(data.add(0xf0, false), (0x02, flag));
    }

    #[test]
    fn test_adc_8b_c_flag() {
        let mut flag = Flags::default();
        let data: u8 = 0x12;
        flag.set_c(true);
        assert_eq!(data.add(0xf0, true), (0x03, flag));
    }

    #[test]
    fn test_adc_8b_h_flag() {
        let mut flag = Flags::default();
        let data: u8 = 0x12;
        flag.set_h(true);
        assert_eq!(data.add(0x2f, true), (0x42, flag));
    }

    #[test]
    fn test_add_8b_h_c_flag() {
        let mut flag = Flags::default();
        let data: u8 = 0x0a;
        flag.set_h(true);
        flag.set_c(true);
        assert_eq!(data.add(0xfa, false), (0x04, flag));
    }

    #[test]
    fn test_adc_8b_h_c_flag() {
        let mut flag = Flags::default();
        let data: u8 = 0x0a;
        flag.set_h(true);
        flag.set_c(true);
        assert_eq!(data.add(0xfa, true), (0x05, flag));
    }

    #[test]
    fn test_add_8b_z_flag() {
        let mut flag = Flags::default();
        let data: u8 = 0x00;
        flag.set_z(true);
        assert_eq!(data.add(0x00, false), (0x00, flag));
    }

    #[test]
    fn test_add_8b_z_c_flag() {
        let mut flag = Flags::default();
        let data: u8 = 0x20;
        flag.set_c(true);
        flag.set_z(true);
        assert_eq!(data.add(0xe0, false), (0x00, flag));
    }

    #[test]
    fn test_add_8b_all_flags() {
        let mut flag = Flags::default();
        let data: u8 = 0x08;
        flag.set_z(true);
        flag.set_c(true);
        flag.set_h(true);
        assert_eq!(data.add(0xf8, false), (0x00, flag));
    }

    #[test]
    fn test_adc_8b_all_flags() {
        let mut flag = Flags::default();
        let data: u8 = 0x07;
        flag.set_z(true);
        flag.set_c(true);
        flag.set_h(true);
        assert_eq!(data.add(0xf8, true), (0x00, flag));
    }
}
