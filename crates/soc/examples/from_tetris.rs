use soc::SOC;
use std::convert::TryFrom;

fn main() {
    let _soc = SOC::try_from("roms/Tetris.gb");
}
