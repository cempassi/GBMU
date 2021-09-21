pub struct VolumeEnvelope {
    pub(crate) period: u8,
    goes_up: bool,
    delay: u8,
    initial_volume: u8,
    pub volume: u8,
}

impl VolumeEnvelope {
    pub fn new() -> VolumeEnvelope {
        VolumeEnvelope {
            period: 0,
            goes_up: false,
            delay: 0,
            initial_volume: 0,
            volume: 0,
        }
    }

    pub fn set(&mut self, address: usize, data: u8) {
        match address {
            0xff12 | 0xff17 | 0xff21 => {
                self.period = data & 0x7;
                self.goes_up = data & 0x8 == 0x8;
                self.initial_volume = data >> 4;
                self.volume = self.initial_volume;
            }
            0xff14 | 0xff19 | 0xff23 if data & 0x80 == 0x80 => {
                self.delay = self.period;
                self.volume = self.initial_volume;
            }
            _ => (),
        }
    }

    pub fn step(&mut self) {
        match self.delay {
            0 => (),
            1 => {
                self.delay = self.period;
                if self.goes_up && self.volume < 15 {
                    self.volume += 1;
                } else if !self.goes_up && self.volume > 0 {
                    self.volume -= 1;
                }
            }
            2..=7 => self.delay -= 1, // period is on 3 bit
            _ => (),
        }
    }
}

#[cfg(test)]
mod volume_envelope_test {
    use super::VolumeEnvelope;

    #[test]
    fn test_volume_envelope_set_period() {
        let mut volume_env = VolumeEnvelope::new();
        assert_eq!(volume_env.period, 0);

        volume_env.set(0xff12, 0x04);
        assert_eq!(volume_env.period, 4);
    }

    #[test]
    fn test_volume_envelope_set_goes_up() {
        let mut volume_env = VolumeEnvelope::new();
        assert_eq!(volume_env.goes_up, false);

        volume_env.set(0xff21, 0x4f);
        assert_eq!(volume_env.goes_up, true);

        volume_env.set(0xff12, 0x0);
        assert_eq!(volume_env.goes_up, false);
    }

    #[test]
    fn test_volume_envelope_set_initial_volume() {
        let mut volume_env = VolumeEnvelope::new();
        assert_eq!(volume_env.initial_volume, 0);

        volume_env.set(0xff17, 0xff);
        assert_eq!(volume_env.initial_volume, 0x0f);
    }

    #[test]
    fn test_volume_envelope_set_delay() {
        let mut volume_env = VolumeEnvelope::new();
        assert_eq!(volume_env.delay, 0);

        volume_env.set(0xff17, 0x07); // set period of 7
        assert_eq!(volume_env.period, 7);

        volume_env.set(0xff19, 0xC7); // set delay
        assert_eq!(volume_env.delay, 7);
    }

    #[test]
    fn test_volume_envelope_step() {
        let mut volume_env = VolumeEnvelope::new();
        assert_eq!(volume_env.goes_up, false);

        volume_env.set(0xff17, 0xf5); // set period of 5 and the volume at 15
        assert_eq!(volume_env.period, 5);
        assert_eq!(volume_env.volume, 15);

        volume_env.set(0xff23, 0x82); // set delay
        assert_eq!(volume_env.delay, 5);

        volume_env.step();
        assert_eq!(volume_env.delay, 4);
    }

    #[test]
    fn test_volume_envelope_increase_volume() {
        let mut volume_env = VolumeEnvelope::new();
        assert_eq!(volume_env.goes_up, false);

        volume_env.set(0xff17, 0x79); // set period of 1, the volume at 7 and goes_up at true
        assert_eq!(volume_env.period, 1);
        assert_eq!(volume_env.volume, 7);
        assert_eq!(volume_env.goes_up, true);

        volume_env.set(0xff23, 0x80); // set delay
        assert_eq!(volume_env.delay, 1);

        volume_env.step();
        assert_eq!(volume_env.volume, 8);
    }

    #[test]
    fn test_volume_envelope_decrease_volume() {
        let mut volume_env = VolumeEnvelope::new();
        assert_eq!(volume_env.goes_up, false);

        volume_env.set(0xff17, 0x11); // set period of 1, the volume at 1
        assert_eq!(volume_env.period, 1);
        assert_eq!(volume_env.volume, 1);

        volume_env.set(0xff23, 0x80); // set delay
        assert_eq!(volume_env.delay, 1);

        volume_env.step();
        assert_eq!(volume_env.volume, 0);
    }
}
