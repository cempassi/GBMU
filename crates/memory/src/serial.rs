use crate::consts;

#[derive(Debug, Default)]
pub struct Serial {
    data: u8,
    control: u8,
}

impl Serial {
    pub fn get(&self, address: u16) -> u8 {
        if address == consts::SERIAL_DATA {
            self.data
        } else {
            self.control
        }
    }

    pub fn set(&mut self, address: u16, data: u8) {
        if address == consts::SERIAL_DATA {
            self.data = data;
        } else {
            self.control = data;
        }
    }
}
