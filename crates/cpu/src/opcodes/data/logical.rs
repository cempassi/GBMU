use crate::Flags;

pub fn xor(first: u8, second: u8) -> u16 {
    let data = first ^ second;
    let mut flag = Flags::default();
    flag.set_z(data == 0);
    (data as u16) << 8 | Flags::into_bytes(flag)[0] as u16
}

#[cfg(test)]
mod test_logicals_functions {
    use super::xor;

    #[test]
    fn test_xor_functions() {
        let first: u8 = 0x00;
        let second: u8 = 0x00;
        assert_eq!(xor(first, second), 0x0010);
    }
}
