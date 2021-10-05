use crate::Flags;

// const MAX_BIT3: usize = (1 << 4) - 1;

pub fn and(first: u8, second: u8) -> u16 {
    let data = first & second;
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
        let first: u8 = 0xf0;
        let second: u8 = 0x0f;
        assert_eq!(and(first, second), 0x0050); // Result = 0x00 + F Reg = 0x50 cause of Z flag
    }
}
