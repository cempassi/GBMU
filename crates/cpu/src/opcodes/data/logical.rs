use crate::opcodes::data::consts::MAX_BIT3;
use crate::Flags;

pub fn cmp(reg_a: u8, byte: u8) -> Flags {
    let mut flag = Flags::default();
    flag.set_z(reg_a == byte);
    flag.set_n(true);
    flag.set_h(reg_a & (MAX_BIT3 as u8) < byte & (MAX_BIT3 as u8));
    flag.set_c(reg_a < byte);
    flag
}

#[cfg(test)]
mod test_logicals_functions {
    use super::cmp;
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
}
