use crate::opcodes::consts::BIT3_MINUS_1;
use crate::Flags;

pub fn and(src: u8, dst: u8) -> u16 {
    let data = src & dst;
    let mut flag = Flags::default();
    flag.set_z(data == 0);
    flag.set_h(true);
    (data as u16) << 8 | Flags::into_bytes(flag)[0] as u16
}

pub fn or(src: u8, dst: u8) -> u16 {
    let data = src | dst;
    let mut flag = Flags::default();
    flag.set_z(data == 0);
    (data as u16) << 8 | Flags::into_bytes(flag)[0] as u16
}

pub fn xor(src: u8, dst: u8) -> u16 {
    let data = src ^ dst;
    let mut flag = Flags::default();
    flag.set_z(data == 0);
    (data as u16) << 8 | Flags::into_bytes(flag)[0] as u16
}

pub fn cmp(reg_a: u8, byte: u8) -> Flags {
    let mut flag = Flags::default();
    flag.set_z(reg_a == byte);
    flag.set_n(true);
    flag.set_h(reg_a & (BIT3_MINUS_1 as u8) < byte & (BIT3_MINUS_1 as u8));
    flag.set_c(reg_a < byte);
    flag
}

#[cfg(test)]
mod test_logicals_functions {
    use super::and;
    use super::cmp;
    use super::or;
    use super::xor;
    use crate::Flags;

    #[test]
    fn test_cmp_functions() {
        let first: u8 = 0x05;
        let second: u8 = 0x0f;
        let mut flag = Flags::default();
        flag.set_n(true);
        flag.set_c(true);
        flag.set_h(true);
        assert_eq!(cmp(first, second), flag);
    }

    #[test]
    fn test_xor_functions() {
        let src: u8 = 0x00;
        let dst: u8 = 0x00;
        assert_eq!(xor(src, dst), 0x0010);
    }

    #[test]
    fn test_or_functions() {
        let src: u8 = 0x00;
        let dst: u8 = 0x00;
        assert_eq!(or(src, dst), 0x0010);
    }
    #[test]
    fn test_and_functions() {
        let src: u8 = 0xf0;
        let dst: u8 = 0x0f;
        assert_eq!(and(src, dst), 0x0050); // Result = 0x00 + F Reg = 0x50 cause of Z flag
    }
}
