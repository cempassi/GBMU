use soc::SOC;
use windows::Windows;
use std::convert::TryFrom;

pub fn main() {
    Windows::run(SOC::try_from("roms/Tetris.gb").unwrap());
}
