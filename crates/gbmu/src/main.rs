use soc::SOC;
use std::convert::TryFrom;
use windows::Windows;

pub fn main() {
    Windows::run(SOC::try_from("roms/Tetris.gb").unwrap());
}
