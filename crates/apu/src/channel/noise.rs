use crate::channel::VolumeEnvelope;
use blip_buf::BlipBuf;

pub struct NoiseChannel {
    enabled: bool,
    length: u8,
    new_length: u8,
    length_enabled: bool,
    volume_envelope: VolumeEnvelope,
    period: u32,
    shift_width: u8,
    state: u16,
    delay: u32,
    last_amp: i32,
    blip: BlipBuf,
}

impl NoiseChannel {
    #[allow(dead_code)]
    fn new(blip: BlipBuf) -> NoiseChannel {
        NoiseChannel {
            enabled: false,
            length: 0,
            new_length: 0,
            length_enabled: false,
            volume_envelope: VolumeEnvelope::new(),
            period: 2048,
            shift_width: 14,
            state: 1,
            delay: 0,
            last_amp: 0,
            blip,
        }
    }

    #[allow(dead_code)]
    fn set(&mut self, address: usize, data: u8) {
        match address {
            0xff20 => self.new_length = 64 - (data & 0x3f),
            // 0xff21 => (),
            0xff22 => {
                self.shift_width = if data & 8 == 8 { 6 } else { 14 };
                let freq_div = match data & 7 {
                    0 => 8,
                    n => (n as u32 + 1) * 16,
                };
                self.period = freq_div << (data >> 4);
            }
            0xff23 => {
                self.length_enabled = data & 0x40 == 0x40;
                if data & 0x80 == 0x80 {
                    self.enabled = true;
                    self.length = self.new_length;
                    self.state = 0xff;
                    self.delay = 0;
                }
            }
            _ => (),
        }
        self.volume_envelope.set(address, data);
    }

    #[allow(dead_code)]
    fn on(&self) -> bool {
        self.enabled
    }

    #[allow(dead_code)]
    fn run(&mut self, start_time: u32, end_time: u32) {
        if !self.enabled || self.volume_envelope.volume == 0 {
            if self.last_amp != 0 {
                self.blip.add_delta(start_time, -self.last_amp);
                self.last_amp = 0;
                self.delay = 0;
            }
        } else {
            let mut time = start_time + self.delay;
            while time < end_time {
                let oldstate = self.state;
                self.state <<= 1;
                let bit = ((oldstate >> self.shift_width) ^ (self.state >> self.shift_width)) & 1;
                self.state |= bit;

                let amp = match (oldstate >> self.shift_width) & 1 {
                    0 => -(self.volume_envelope.volume as i32),
                    _ => self.volume_envelope.volume as i32,
                };

                if self.last_amp != amp {
                    self.blip.add_delta(time, amp - self.last_amp);
                    self.last_amp = amp;
                }

                time += self.period;
            }
            self.delay = time - end_time;
        }
    }

    #[allow(dead_code)]
    fn step_length(&mut self) {
        if self.length_enabled && self.length != 0 {
            self.length -= 1;
            if self.length == 0 {
                self.enabled = false;
            }
        }
    }
}

#[cfg(test)]
mod test_noise_channel {
    use super::NoiseChannel;
    use blip_buf::BlipBuf;

    const CLOCKS_PER_SECOND: u32 = 1 << 22;
    const SAMPLES_RATE: u32 = 48000;

    fn create_blipbuf(samples_rate: u32) -> BlipBuf {
        let mut blipbuf = BlipBuf::new(samples_rate);
        blipbuf.set_rates(CLOCKS_PER_SECOND as f64, samples_rate as f64);
        blipbuf
    }

    #[test]
    fn test_noise_channel_on() {
        let mut noise_channel = NoiseChannel::new(create_blipbuf(SAMPLES_RATE));
        assert_eq!(noise_channel.on(), false);

        noise_channel.set(0xff23, 0x80);
        assert_eq!(noise_channel.on(), true);
    }

    #[test]
    fn test_noise_channel_new_length() {
        let mut noise_channel = NoiseChannel::new(create_blipbuf(SAMPLES_RATE));
        assert_eq!(noise_channel.new_length, 0);

        noise_channel.set(0xff20, 0x33);
        assert_eq!(noise_channel.new_length, 0xd);
    }

    #[test]
    fn test_noise_channel_length() {
        let mut noise_channel = NoiseChannel::new(create_blipbuf(SAMPLES_RATE));
        assert_eq!(noise_channel.length, 0);

        noise_channel.set(0xff20, 0x3f);
        assert_eq!(noise_channel.new_length, 64 - 0x3f);
        assert_eq!(noise_channel.enabled, false);
        assert_eq!(noise_channel.length_enabled, false);
        assert_eq!(noise_channel.state, 1);

        noise_channel.set(0xff23, 0xc0);
        assert_eq!(noise_channel.length, noise_channel.new_length);
        assert_eq!(noise_channel.length_enabled, true);
        assert_eq!(noise_channel.state, 0xff);
        assert_eq!(noise_channel.enabled, true);
    }

    #[test]
    fn test_noise_channel_shift_width_and_period() {
        let mut noise_channel = NoiseChannel::new(create_blipbuf(SAMPLES_RATE));
        assert_eq!(noise_channel.shift_width, 14);

        noise_channel.set(0xff22, 0x08);
        assert_eq!(noise_channel.shift_width, 6);
        assert_eq!(noise_channel.period, 8);
    }

    #[test]
    fn test_noise_channel_period() {
        let mut noise_channel = NoiseChannel::new(create_blipbuf(SAMPLES_RATE));
        assert_eq!(noise_channel.period, 2048);

        noise_channel.set(0xff22, 0x07);
        assert_eq!(noise_channel.shift_width, 14);
        assert_eq!(noise_channel.period, (0x07 + 1) * 16);
    }

    #[test]
    fn test_noise_channel_volume() {
        let mut noise_channel = NoiseChannel::new(create_blipbuf(SAMPLES_RATE));
        assert_eq!(noise_channel.volume_envelope.volume, 0);

        noise_channel.set(0xff17, 0xf5); // set period of 5 and the volume at 15
        assert_eq!(noise_channel.volume_envelope.period, 5);
        assert_eq!(noise_channel.volume_envelope.volume, 15);
    }

    #[test]
    fn test_noise_channel_step_length() {
        let mut noise_channel = NoiseChannel::new(create_blipbuf(SAMPLES_RATE));
        assert_eq!(noise_channel.period, 2048);

        noise_channel.set(0xff20, 0x3f);
        assert_eq!(noise_channel.new_length, 64 - 0x3f);
        assert_eq!(noise_channel.enabled, false);
        assert_eq!(noise_channel.length_enabled, false);
        assert_eq!(noise_channel.state, 1);

        noise_channel.set(0xff23, 0xcf);
        assert_eq!(noise_channel.length, noise_channel.new_length);
        assert_eq!(noise_channel.length_enabled, true);
        assert_eq!(noise_channel.state, 0xff);
        assert_eq!(noise_channel.enabled, true);

        noise_channel.step_length();
        assert_eq!(noise_channel.length, 0);
    }

    // #[test]
    // fn test_noise_channel_run() {
    //     let mut noise_channel = NoiseChannel::new(create_blipbuf(SAMPLES_RATE));
    //     todo!()
    //     // noise_channel.run()
    // }
}
