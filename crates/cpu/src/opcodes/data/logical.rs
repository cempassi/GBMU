use crate::Flags;

pub fn or(src: u8, dst: u8) -> u16 {
    let data = src | dst;
    let mut flag = Flags::default();
    flag.set_z(data == 0);
    (data as u16) << 8 | Flags::into_bytes(flag)[0] as u16
}

#[cfg(test)]
mod test_logicals_functions {
    use super::or;

    #[test]
    fn test_or_functions() {
        let src: u8 = 0x00;
        let dst: u8 = 0x00;
        assert_eq!(or(src, dst), 0x0010);
    }
}
