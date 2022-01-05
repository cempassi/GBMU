use std::fs;
use std::path::PathBuf;
use std::process;

use log::*;
use anyhow::{Context, Ok, Result};
use structopt::StructOpt;
use structopt::clap::AppSettings::*;

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

#[derive(Debug, StructOpt)]
#[structopt(setting(ColorAuto), setting(ColoredHelp))]
#[structopt(author, about)]
struct Opt {
    /// A file containing a ROM to load into the emulator.
    #[structopt(long, short, parse(from_os_str))]
    rom: PathBuf,

    /// A file containing a binary dump of the Game Boy BIOS.
    ///
    /// If not supplied, the emulator will begin executing the ROM as if the BIOS had succeeded.
    #[structopt(long)]
    bios: Option<PathBuf>,

    /// Enable debug mode.
    #[structopt(short, long)]
    debug: bool,
}

fn run(opt: Opt) -> Result<()> {
    //let mut builder = Emulator::builder();

    //let mut emulator = builder.build();

    if let Some(bios) = &opt.bios {
        info!("loading BIOS from file '{}'", bios.display());
        let bios = fs::read(&bios).context("could not read BIOS")?;
        //emulator.load_bios(&bios).context("could not load BIOS")?;
    }

    info!("loading ROM from file '{}'", opt.rom.display());
    let rom = opt.rom.into_os_string().into_string().unwrap();
    //emulator.load_rom(&rom).context("could not load ROM")?;

    Windows::run(rom.as_str());
    Ok(())
}


//pub fn main() {
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
    //Windows::run("ressources/test_roms/instr_timing/instr_timing.gb");
    //Windows::run("roms/Tetris.gb");
//}

fn main() {
    pretty_env_logger::init();
    let opt = Opt::from_args();

    if let Err(e) = run(opt) {
        eprintln!("fatal error: {:?}", e);

        // if let Some(pixels::Error::AdapterNotFound) = e.downcast_ref() {
        //     eprintln!("help: ensure your graphics adapter supports Vulkan");
        // }

        process::exit(1);
    }
}
