use blip_buf::BlipBuf;

pub struct WaveChannel {
    enabled: bool,
    enabled_flag: bool,
    length: u16,
    new_length: u16,
    length_enabled: bool,
    frequency: u16,
    period: u32,
    last_amp: i32,
    delay: u32,
    volume_shift: u8,
    pub(crate) ram: [u8; 32],
    current_wave: u8,
    pub(crate) blip: BlipBuf,
}

impl WaveChannel {
    pub(crate) fn new(blip: BlipBuf) -> WaveChannel {
        WaveChannel {
            enabled: false,
            enabled_flag: false,
            length: 0,
            new_length: 0,
            length_enabled: false,
            frequency: 0,
            period: 2048,
            last_amp: 0,
            delay: 0,
            volume_shift: 0,
            ram: [0; 32],
            current_wave: 0,
            blip,
        }
    }

    pub(crate) fn set(&mut self, address: usize, data: u8) {
        match address {
            0xff1a => {
                self.enabled_flag = (data & 0x80) == 0x80;
                self.enabled = self.enabled && self.enabled_flag;
            }
            0xff1b => self.new_length = 256 - (data as u16),
            0xff1c => self.volume_shift = (data >> 5) & 0b11,
            0xff1d => {
                self.frequency = (self.frequency & 0x0700) | (data as u16);
                self.calculate_period();
            }
            0xff1e => {
                self.frequency = (self.frequency & 0x00ff) | (((data & 0b111) as u16) << 8);
                self.calculate_period();
                self.length_enabled = data & 0x40 == 0x40;
                if data & 0x80 == 0x80 && self.enabled_flag {
                    self.length = self.new_length;
                    self.enabled = true;
                    self.current_wave = 0;
                    self.delay = 0;
                }
            }
            0xff30..=0xff3f => {
                self.ram[(address - 0xff30) / 2] = data >> 4;
                self.ram[(address - 0xff30) / 2 + 1] = data & 0xf;
            }
            _ => (),
        }
    }

    fn calculate_period(&mut self) {
        if self.frequency > 2048 {
            self.period = 0;
        } else {
            self.period = (2048 - self.frequency as u32) * 2;
        }
    }

    pub(crate) fn on(&self) -> bool {
        self.enabled
    }

    pub(crate) fn run(&mut self, start_time: u32, end_time: u32) {
        if !self.enabled || self.period == 0 {
            if self.last_amp != 0 {
                self.blip.add_delta(start_time, -self.last_amp);
                self.last_amp = 0;
                self.delay = 0;
            }
        } else {
            let mut time = start_time + self.delay;

            // A sample may be muted, 100%, 50% or 25%.
            // To avoid loss of precision, the WaveChannel will output
            // i32 samples at 4x the usual amplitude. This will be taken
            // into account when mixing all the samples.
            let volshift = match self.volume_shift {
                0 => 4 + 2, // to mute a 4 bit sample mutiplied by 2^2
                1 => 0,
                2 => 1,
                3 => 2,
                _ => unreachable!(),
            };

            while time < end_time {
                let sample = self.ram[self.current_wave as usize];

                // shifted by 2 so that 25% does not lose precision
                let amp = ((sample << 2) >> volshift) as i32;

                if amp != self.last_amp {
                    self.blip.add_delta(time, amp - self.last_amp);
                    self.last_amp = amp;
                }

                time += self.period;
                self.current_wave = (self.current_wave + 1) % 32;
            }

            // next time, we have to wait an additional delay timesteps
            self.delay = time - end_time;
        }
    }

    pub(crate) fn step_length(&mut self) {
        if self.length_enabled && self.length != 0 {
            self.length -= 1;
            if self.length == 0 {
                self.enabled = false;
            }
        }
    }
}

#[cfg(test)]
mod test_wave_channel {
    use super::WaveChannel;
    use crate::sound::create_blipbuf;

    const SAMPLES_RATE: u32 = 48000;

    #[test]
    fn test_wave_channel_on() {
        let mut wave_channel = WaveChannel::new(create_blipbuf(SAMPLES_RATE));
        assert_eq!(wave_channel.on(), false);

        wave_channel.set(0xff1a, 0x80);
        assert_eq!(wave_channel.enabled_flag, true);

        wave_channel.set(0xff1e, 0xc0);
        assert_eq!(wave_channel.on(), true);
    }

    #[test]
    fn test_wave_channel_new_length() {
        let mut wave_channel = WaveChannel::new(create_blipbuf(SAMPLES_RATE));
        assert_eq!(wave_channel.new_length, 0);

        wave_channel.set(0xff1b, 0xff);
        assert_eq!(wave_channel.new_length, 256 - 0xff);
    }

    #[test]
    fn test_wave_channel_volume_shift() {
        let mut wave_channel = WaveChannel::new(create_blipbuf(SAMPLES_RATE));
        assert_eq!(wave_channel.volume_shift, 0);

        wave_channel.set(0xff1c, 0x6f);
        assert_eq!(wave_channel.volume_shift, (0x6f >> 5) & 0b11);
    }

    #[test]
    fn test_wave_channel_lowfrequency_and_period() {
        let mut wave_channel = WaveChannel::new(create_blipbuf(SAMPLES_RATE));
        assert_eq!(wave_channel.period, 2048);
        assert_eq!(wave_channel.frequency, 0);

        wave_channel.set(0xff1d, 0x42);
        assert_eq!(wave_channel.period, (2048 - 0x42) * 2);
        assert_eq!(wave_channel.frequency, 0x42);
    }

    #[test]
    fn test_wave_channel_highfrequency_and_period() {
        let mut wave_channel = WaveChannel::new(create_blipbuf(SAMPLES_RATE));
        assert_eq!(wave_channel.period, 2048);
        assert_eq!(wave_channel.frequency, 0);

        wave_channel.set(0xff1e, 0xc3);
        assert_eq!(wave_channel.period, (2048 - ((0xc3 & 0x03) << 8)) * 2);
        assert_eq!(wave_channel.frequency, ((0xc3 & 0x03) << 8));
    }

    #[test]
    fn test_wave_channel_write_ram() {
        let mut wave_channel = WaveChannel::new(create_blipbuf(SAMPLES_RATE));
        assert_eq!(wave_channel.length, 0);

        wave_channel.set(0xff30, 0x42);
        assert_eq!(wave_channel.ram[0], 0x4); // (0xff30 - 0xFF30) / 2 = 0
        assert_eq!(wave_channel.ram[1], 0x2); // (0xff30 - 0xFF30) / 2 + 1 = 1
    }

    #[test]
    fn test_wave_channel_step_length() {
        let mut wave_channel = WaveChannel::new(create_blipbuf(SAMPLES_RATE));
        assert_eq!(wave_channel.length, 0);

        wave_channel.set(0xff1a, 0x80);
        assert_eq!(wave_channel.enabled_flag, true);

        wave_channel.set(0xff1b, 0x42);
        assert_eq!(wave_channel.new_length, 256 - 0x42);
        assert_eq!(wave_channel.enabled, false);
        assert_eq!(wave_channel.length_enabled, false);

        wave_channel.set(0xff1e, 0xff);
        assert_eq!(wave_channel.length, wave_channel.new_length);
        assert_eq!(wave_channel.length_enabled, true);
        assert_eq!(wave_channel.enabled, true);

        wave_channel.step_length();
        assert_eq!(wave_channel.length, wave_channel.new_length - 1);
    }

    // #[test]
    // fn test_wave_channel_run() {
    //     let mut wave_channel = WaveChannel::new(create_blipbuf(SAMPLES_RATE));
    //     todo!()
    //     // wave_channel.run()
    // }
}
