struct VolumeEnvelope {
    period : u8,
    goes_up : bool,
    delay : u8,
    initial_volume : u8,
    volume : u8,
}

impl VolumeEnvelope {
    fn new() -> VolumeEnvelope {
        VolumeEnvelope {
            period: 0,
            goes_up: false,
            delay: 0,
            initial_volume: 0,
            volume: 0
        }
    }

    fn set(&mut self, address: usize, data: u8) -> Result<(), Error>{
        match address {
            0xff12 | 0xff17 | 0xff21 => {
                self.period = data & 0x7;
                self.goes_up = v &0x8 == 0x8;
                self.initial_volume = v >> 4;
                self.volume = self.initial_volume;
            },
            0xFF14 | 0xFF19 | 0xFF23 if v & 0x80 == 0x80 => {
                self.delay = self.period;
                self.volume = self.initial_volume;
            },
            _ => unreachable!(),
        }
        Ok (())
    }


    fn step(&mut self) {
        if self.delay > 1 {
            self.delay -= 1;
        }
        else if self.delay == 1 {
            self.delay = self.period;
            if self.goes_up && self.volume < 15 {
                self.volume += 1;
            }
            else if !self.goes_up && self.volume > 0 {
                self.volume -= 1;
            }
        }
    }

}
