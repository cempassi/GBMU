use windows::Windows;

// ressources/test_roms/cpu_instrs/individual/01-special.gb (PASSED)
// ressources/test_roms/cpu_instrs/individual/02-interrupts.gb (PASSED)
// ressources/test_roms/cpu_instrs/individual/03-op sp,hl.gb (PASSED)
// ressources/test_roms/cpu_instrs/individual/04-op r,imm.gb (PASSED)
// ressources/test_roms/cpu_instrs/individual/05-op rp.gb (PASSED)
// ressources/test_roms/cpu_instrs/individual/06-ld r,r.gb (PASSED)
// ressources/test_roms/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb (PASSED)
// ressources/test_roms/cpu_instrs/individual/08-misc instrs.gb(PASSED)
// ressources/test_roms/cpu_instrs/individual/09-op r,r.gb (PASSED)
// ressources/test_roms/cpu_instrs/individual/10-bit ops.gb (PASSED)
// ressources/test_roms/cpu_instrs/individual/11-op a,(hl).gb (PASSED)

pub fn main() {
    // Windows::run("ressources/test_roms/cpu_instrs/cpu_instrs.gb");
    // Windows::run("ressources/test_roms/cpu_instrs/individual/01-special.gb");
    // Windows::run("ressources/test_roms/cpu_instrs/individual/02-interrupts.gb");
    //Windows::run("ressources/test_roms/cpu_instrs/individual/03-op sp,hl.gb");
    // Windows::run("ressources/test_roms/cpu_instrs/individual/04-op r,imm.gb");
    // Windows::run("ressources/test_roms/cpu_instrs/individual/05-op rp.gb");
    // Windows::run("ressources/test_roms/cpu_instrs/individual/06-ld r,r.gb");
    // Windows::run("ressources/test_roms/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb");
    // Windows::run("ressources/test_roms/cpu_instrs/individual/08-misc instrs.gb");
    // Windows::run("ressources/test_roms/cpu_instrs/individual/09-op r,r.gb");
    // Windows::run("ressources/test_roms/cpu_instrs/individual/10-bit ops.gb");
    // Windows::run("ressources/test_roms/cpu_instrs/individual/11-op a,(hl).gb");
    Windows::run("roms/Tetris.gb");
}
