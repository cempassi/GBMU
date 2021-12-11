/// Audio Processing Unit

#[derive(Debug)]
#[allow(dead_code)]
pub struct Apu {
    data: Vec<u8>,
}

impl Apu {
    pub fn new() -> Self {
        let data = vec![0; 0x30];
        Self { data }
    }
}

impl Default for Apu {
    fn default() -> Self {
        Self::new()
    }
}
