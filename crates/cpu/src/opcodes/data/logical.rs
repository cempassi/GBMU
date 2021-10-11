use crate::Flags;

pub fn and(src: u8, dst: u8) -> u16 {
    let data = src & dst;
    let mut flag = Flags::default();
    flag.set_z(data == 0);
    flag.set_h(true);
    (data as u16) << 8 | Flags::into_bytes(flag)[0] as u16
}

#[cfg(test)]
mod test_logicals_functions {
    use super::and;

    #[test]
    fn test_and_functions() {
        let src: u8 = 0xf0;
        let dst: u8 = 0x0f;
        assert_eq!(and(src, dst), 0x0050); // Result = 0x00 + F Reg = 0x50 cause of Z flag
    }
}
