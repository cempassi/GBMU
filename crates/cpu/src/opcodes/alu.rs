use crate::Flags;

#[allow(dead_code)]
fn carry(shift: usize, nbr1: usize, nbr2: usize, c: usize) -> bool {
    let c = c as usize;
    let max = (1 << shift) - 1;
    (nbr1 & max) + (nbr2 & max) + (c & max) > max
}

#[allow(dead_code)]
fn borrow(shift: usize, nbr1: usize, nbr2: usize, c: usize) -> bool {
    let max = (1 << shift) - 1;
    (nbr1 & max) < (nbr2 & max) + (c & max)
}

#[allow(dead_code)]
fn add(shift: usize, nbr1: usize, nbr2: usize, c: bool, hb: usize, cb: usize) -> (usize, Flags) {
    let c = c as usize;
    let max = (1 << shift) - 1;
    let data = (nbr1 + nbr2 + c) & max;
    let h = carry(hb, nbr1, nbr2, c);
    let c = carry(cb, nbr1, nbr2, c);
    let z = data == 0;
    let mut flag = Flags::default();
    flag.set_z(z);
    flag.set_h(h);
    flag.set_c(c);
    (data, flag)
}

#[allow(dead_code)]
fn sub(shift: usize, nbr1: usize, nbr2: usize, c: bool, hb: usize, cb: usize) -> (usize, Flags) {
    let c = c as usize;
    let max = (1 << shift) - 1;
    let data = (nbr1.wrapping_sub(nbr2).wrapping_sub(c)) & max;
    let h = borrow(hb, nbr1, nbr2, c);
    let c = borrow(cb, nbr1, nbr2, c);
    let z = data == 0;
    let mut flag = Flags::default();
    flag.set_z(z);
    flag.set_h(h);
    flag.set_c(c);
    (data, flag)
}

#[allow(dead_code)]
pub fn signed(value: u8) -> u16 {
    if value & 0x80 != 0 {
        0xff00 | value as u16
    } else {
        value as u16
    }
}

#[allow(dead_code)]
pub fn add8(nbr1: u8, nbr2: u8, c: bool) -> (u8, Flags) {
    let (data, flag) = add(8, nbr1 as usize, nbr2 as usize, c, 4, 8);
    (data as u8, flag)
}

#[allow(dead_code)]
pub fn sub8(nbr1: u8, nbr2: u8, c: bool) -> (u8, Flags) {
    let (data, flag) = sub(8, nbr1 as usize, nbr2 as usize, c, 4, 8);
    (data as u8, flag)
}

#[allow(dead_code)]
pub fn add16(nbr1: u16, nbr2: u16, c: bool) -> (u16, Flags) {
    let (data, flag) = add(16, nbr1 as usize, nbr2 as usize, c, 12, 16);
    (data as u16, flag)
}

#[allow(dead_code)]
pub fn add_signed16(nbr1: u16, nbr2: u8, c: bool) -> (u16, Flags) {
    let (v, flag) = add(16, nbr1 as usize, signed(nbr2) as usize, c, 4, 8);
    (v as u16, flag)
}

#[cfg(test)]
mod test_alu {
    use super::{add16, add8, add_signed16, signed, sub8};
    use crate::Flags;

    #[test]
    fn test_add8() {
        let mut flag = Flags::default();
        assert_eq!(add8(0x12, 0x22, false), (0x34, flag));
        assert_eq!(add8(0x12, 0x22, true), (0x35, flag));
        flag.set_h(true);
        assert_eq!(add8(0x12, 0x2f, false), (0x41, flag));
        flag.set_h(false);
        flag.set_c(true);
        assert_eq!(add8(0x12, 0xf0, false), (0x02, flag));
        assert_eq!(add8(0x12, 0xf0, false), (0x02, flag));
        assert_eq!(add8(0x12, 0xf0, true), (0x03, flag));
        flag.set_c(false);
        flag.set_h(true);
        assert_eq!(add8(0x12, 0x2f, true), (0x42, flag));
        flag.set_c(true);
        assert_eq!(add8(0x0a, 0xfa, false), (0x04, flag));
        assert_eq!(add8(0x0a, 0xfa, true), (0x05, flag));
        flag.set_z(true);
        flag.set_h(false);
        flag.set_c(false);
        assert_eq!(add8(0x00, 0x00, false), (0x00, flag));
        flag.set_c(true);
        assert_eq!(add8(0x20, 0xe0, false), (0x00, flag));
        flag.set_z(true);
        flag.set_c(true);
        flag.set_h(true);
        assert_eq!(add8(0x08, 0xf8, false), (0x00, flag));
        assert_eq!(add8(0x07, 0xf8, true), (0x00, flag));
    }

    #[test]
    fn test_sub8() {
        let mut flag = Flags::default();
        assert_eq!(sub8(0x12, 0x10, false), (0x02, flag));
        assert_eq!(sub8(0x34, 0x22, true), (0x11, flag));
        flag.set_h(true);
        assert_eq!(sub8(0x32, 0x2f, false), (0x03, flag));
        assert_eq!(sub8(0x32, 0x2e, true), (0x03, flag));
        flag.set_c(true);
        flag.set_h(false);
        assert_eq!(sub8(0x12, 0xf0, false), (0x22, flag));
        assert_eq!(sub8(0x12, 0xe0, true), (0x31, flag));
        flag.set_h(true);
        assert_eq!(sub8(0x0a, 0xef, false), (0x1b, flag));
        assert_eq!(sub8(0x20, 0x5a, true), (0xc5, flag));
        flag.set_z(true);
        flag.set_h(false);
        flag.set_c(false);
        assert_eq!(sub8(0x12, 0x12, false), (0x00, flag));
        assert_eq!(sub8(0x88, 0x87, true), (0x00, flag));
    }

    #[test]
    fn test_add16() {
        let mut flag = Flags::default();
        assert_eq!(add16(0x1200, 0x1000, false), (0x2200, flag));
        assert_eq!(add16(0x1134, 0x1222, true), (0x2357, flag));
        flag.set_c(true);
        assert_eq!(add16(0xf231, 0x2a13, false), (0x1c44, flag));
        assert_eq!(add16(0xf231, 0x2a13, true), (0x1c45, flag));
        flag.set_h(true);
        assert_eq!(add16(0xf631, 0x2a03, false), (0x2034, flag));
        assert_eq!(add16(0xf631, 0x2a03, true), (0x2035, flag));
    }

    #[test]
    fn test_signed() {
        assert_eq!(signed(0x0a), 0x000a);
        assert_eq!(signed(0x8a), 0xff8a);
    }

    #[test]
    fn test_add_signed16() {
        let flag = Flags::default();
        assert_eq!(add_signed16(0x1200, 0x10, false), (0x1210, flag));
        assert_eq!(add_signed16(0x1134, 0x12, true), (0x1147, flag));
        assert_eq!(add_signed16(0xf231, 0x2a, false), (0xf25b, flag));
    }
}
