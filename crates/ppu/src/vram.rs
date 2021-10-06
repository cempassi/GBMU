const VRAM_SIZE: usize = 8192;

#[derive(Debug)]
pub struct Vram {
    data: Vec<u8>,
}

impl Default for Vram {
    fn default() -> Self {
        Self::new()
    }
}

impl AsRef<Vec<u8>> for Vram {
    fn as_ref(&self) -> &Vec<u8> {
        self.data.as_ref()
    }
}

impl Vram {
    pub fn new() -> Self {
        Self {
            data: vec![0; VRAM_SIZE],
        }
    }

    pub fn get(&self, address: usize) -> u8 {
        self.data[address]
    }

    pub fn set(&mut self, address: usize, data: u8) {
        self.data[address] = data;
    }
}

#[cfg(test)]
mod test_wram {
    use super::Vram;

    #[test]
    fn test_read_vram() {
        let vram = Vram::default();

        assert_eq!(vram.get(0x10), 0);
    }

    #[test]
    fn test_write_read_vram() {
        let mut vram = Vram::default();

        vram.set(0x42, 42);
        let read = vram.get(0x42);

        assert_eq!(read, 42);
    }
}
