use crate::Flags;

pub fn xor(src: u8, dst: u8) -> u16 {
    let data = src ^ dst;
    let mut flag = Flags::default();
    flag.set_z(data == 0);
    (data as u16) << 8 | Flags::into_bytes(flag)[0] as u16
}

#[cfg(test)]
mod test_logicals_functions {
    use super::xor;

    #[test]
    fn test_xor_functions() {
        let src: u8 = 0x00;
        let dst: u8 = 0x00;
        assert_eq!(xor(src, dst), 0x0010);
    }
}
