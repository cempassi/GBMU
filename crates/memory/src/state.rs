use shared::Error;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
    Bios,
    Rom,
}

impl State {
    pub fn disable_bios(&mut self) -> Result<(), Error> {
        *self = Self::Rom;
        println!("Disabling Bios");
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Cycle {
    Cpu(u8),
    Finished,
}
