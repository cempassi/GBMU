use crate::channel::volume::VolumeEnvelope;
use blip_buf::BlipBuf;

const WAVE_PATTERN: [[i32; 8]; 4] = [
    [-1, -1, -1, -1, 1, -1, -1, -1],
    [-1, -1, -1, -1, 1, 1, -1, -1],
    [-1, -1, 1, 1, 1, 1, -1, -1],
    [1, 1, 1, 1, -1, -1, 1, 1],
];

#[allow(dead_code)]
pub struct SquareChannel {
    enabled: bool,
    duty: u8,
    phase: u8,
    length: u8,
    new_length: u8,
    length_enabled: bool,
    frequency: u16,
    period: u32,
    last_amp: i32,
    delay: u32,
    has_sweep: bool,
    sweep_frequency: u16,
    sweep_delay: u8,
    sweep_period: u8,
    sweep_shift: u8,
    sweep_frequency_increase: bool,
    pub volume_envelope: VolumeEnvelope,
    pub blip: BlipBuf,
}

impl SquareChannel {
    #[allow(dead_code)]
    pub fn new(blip: BlipBuf, with_sweep: bool) -> SquareChannel {
        SquareChannel {
            enabled: false,
            duty: 1,
            phase: 1,
            length: 0,
            new_length: 0,
            length_enabled: false,
            frequency: 0,
            period: 2048,
            last_amp: 0,
            delay: 0,
            has_sweep: with_sweep,
            sweep_frequency: 0,
            sweep_delay: 0,
            sweep_period: 0,
            sweep_shift: 0,
            sweep_frequency_increase: false,
            volume_envelope: VolumeEnvelope::new(),
            blip,
        }
    }

    #[allow(dead_code)]
    pub fn on(&self) -> bool {
        self.enabled
    }

    #[allow(dead_code)]
    pub fn set(&mut self, address: usize, data: u8) {
        match address {
            0xff10 => {
                self.sweep_shift = data & 0x7;
                self.sweep_frequency_increase = data & 0x8 == 0x8;
                self.sweep_period = (data >> 4) & 0x7;
            }
            0xff11 | 0xff16 => {
                self.duty = data >> 6;
                self.new_length = 64 - (data & 0x3F);
            }
            0xff13 | 0xff18 => {
                self.frequency = (self.frequency & 0x0700) | (data as u16);
                self.length = self.new_length;
                self.calculate_period();
            }
            0xff14 | 0xff19 => {
                self.frequency = (self.frequency & 0x00ff) | (((data & 0b0000_0111) as u16) << 8);
                self.calculate_period();
                self.length_enabled = data & 0x40 == 0x40;

                if data & 0x80 == 0x80 {
                    self.enabled = true;
                    self.length = self.new_length;

                    self.sweep_frequency = self.frequency;
                    if self.has_sweep && self.sweep_period > 0 && self.sweep_shift > 0 {
                        self.sweep_delay = 1;
                        self.step_sweep();
                    }
                }
            }
            _ => (),
        }
        self.volume_envelope.set(address, data);
    }

    #[allow(dead_code)]
    fn calculate_period(&mut self) {
        if self.frequency > 2048 {
            self.period = 0;
        } else {
            self.period = (2048 - self.frequency as u32) * 4;
        }
    }

    #[allow(dead_code)]
    // This assumes no volume or sweep adjustments need to be done in the meantime
    pub fn run(&mut self, start_time: u32, end_time: u32) {
        if !self.enabled || self.period == 0 || self.volume_envelope.volume == 0 {
            if self.last_amp != 0 {
                self.blip.add_delta(start_time, -self.last_amp);
                self.last_amp = 0;
                self.delay = 0;
            }
        } else {
            let mut time = start_time + self.delay;
            let pattern = WAVE_PATTERN[self.duty as usize];
            let vol = self.volume_envelope.volume as i32;

            while time < end_time {
                let amp = vol * pattern[self.phase as usize];
                if amp != self.last_amp {
                    self.blip.add_delta(time, amp - self.last_amp);
                    self.last_amp = amp;
                }
                time += self.period;
                self.phase = (self.phase + 1) % 8;
            }

            // next time, we have to wait an additional delay timesteps
            self.delay = time - end_time;
        }
    }

    #[allow(dead_code)]
    pub fn step_length(&mut self) {
        if self.length_enabled && self.length != 0 {
            self.length -= 1;
            if self.length == 0 {
                self.enabled = false;
            }
        }
    }

    #[allow(dead_code)]
    pub fn step_sweep(&mut self) {
        if !self.has_sweep || self.sweep_period == 0 {
            return;
        }

        if self.sweep_delay > 1 {
            self.sweep_delay -= 1;
        } else {
            self.sweep_delay = self.sweep_period;
            self.frequency = self.sweep_frequency;
            if self.frequency == 2048 {
                self.enabled = false;
            }
            self.calculate_period();

            let offset = self.sweep_frequency >> self.sweep_shift;

            if self.sweep_frequency_increase {
                // F ~ (2048 - f)
                // Increase in frequency means subtracting the offset
                if self.sweep_frequency <= offset {
                    self.sweep_frequency = 0;
                } else {
                    self.sweep_frequency -= offset;
                }
            } else if self.sweep_frequency >= 2048 - offset {
                self.sweep_frequency = 2048;
            } else {
                self.sweep_frequency += offset;
            }
        }
    }
}

#[cfg(test)]
mod test_square_channel {
    use super::SquareChannel;
    use blip_buf::BlipBuf;

    const CLOCKS_PER_SECOND: u32 = 1 << 22;
    const SAMPLES_RATE: u32 = 48000;

    fn create_blipbuf(samples_rate: u32) -> BlipBuf {
        let mut blipbuf = BlipBuf::new(samples_rate);
        blipbuf.set_rates(CLOCKS_PER_SECOND as f64, samples_rate as f64);
        blipbuf
    }

    #[test]
    fn test_square_channel_on() {
        let mut square_channel = SquareChannel::new(create_blipbuf(SAMPLES_RATE), true);
        assert_eq!(square_channel.on(), false);

        square_channel.set(0xff14, 0x80); // 0x07 => sweep_shift of 7 | sweep_increase = false | sweep_period = 5
        assert_eq!(square_channel.on(), true);
    }

    #[test]
    fn test_square_channel_sweep() {
        let mut square_channel = SquareChannel::new(create_blipbuf(SAMPLES_RATE), true);
        assert_eq!(square_channel.sweep_shift, 0);

        square_channel.set(0xff10, 0x57); // 0x07 => sweep_shift of 7 | sweep_increase = false | sweep_period = 5
        assert_eq!(square_channel.sweep_frequency_increase, false);
        assert_eq!(square_channel.sweep_shift, 7);
        assert_eq!(square_channel.sweep_period, 5);
    }

    #[test]
    fn test_square_channel_duty_and_newlength() {
        let mut square_channel = SquareChannel::new(create_blipbuf(SAMPLES_RATE), false);
        assert_eq!(square_channel.duty, 1);

        square_channel.set(0xff11, 0x8f);
        assert_eq!(square_channel.duty, 2);
        assert_eq!(square_channel.new_length, 64 - 0xf);
    }

    #[test]
    fn test_square_channel_lowfreq_and_period() {
        let mut square_channel = SquareChannel::new(create_blipbuf(SAMPLES_RATE), false);
        assert_eq!(square_channel.period, 2048);

        square_channel.set(0xff11, 0x8f);
        assert_eq!(square_channel.new_length, 64 - 0xf);

        square_channel.set(0xff13, 0xff);
        assert_eq!(square_channel.frequency, 0xff);
        assert_eq!(square_channel.period, 7172);
        assert_eq!(square_channel.length, square_channel.new_length);
    }

    #[test]
    fn test_square_channel_highfreq_and_period() {
        let mut square_channel = SquareChannel::new(create_blipbuf(SAMPLES_RATE), true);
        assert_eq!(square_channel.period, 2048);

        square_channel.set(0xff11, 0x8f);
        assert_eq!(square_channel.new_length, 64 - 0xf);

        square_channel.set(0xff18, 0xff);
        assert_eq!(square_channel.frequency, 0xff);
        assert_eq!(square_channel.period, 7172);
        assert_eq!(square_channel.length, square_channel.new_length);

        square_channel.set(0xff19, 0xcf);
        assert_eq!(square_channel.frequency, 2047);
        assert_eq!(square_channel.period, 4);
        assert_eq!(square_channel.length_enabled, true);
    }

    #[test]
    fn test_square_channel_step_sweep() {
        let mut square_channel = SquareChannel::new(create_blipbuf(SAMPLES_RATE), true);
        assert_eq!(square_channel.has_sweep, true);

        square_channel.set(0xff10, 0x57); // 0x07 => sweep_shift of 7 | sweep_increase = false | sweep_period = 5
        assert_eq!(square_channel.sweep_frequency_increase, false);
        assert_eq!(square_channel.sweep_shift, 7);
        assert_eq!(square_channel.sweep_period, 5);
        square_channel.set(0xff19, 0xcf);
        assert_eq!(square_channel.sweep_frequency, 0x70e);
        assert_eq!(square_channel.sweep_delay, 5);
    }

    #[test]
    fn test_square_channel_step_length() {
        let mut square_channel = SquareChannel::new(create_blipbuf(SAMPLES_RATE), true);
        assert_eq!(square_channel.period, 2048);

        square_channel.set(0xff11, 0xbf);
        assert_eq!(square_channel.new_length, 64 - 0x3f);

        square_channel.set(0xff14, 0xc0);
        assert_eq!(square_channel.length_enabled, true);
        assert_eq!(square_channel.length, 1);

        square_channel.step_length();
        assert_eq!(square_channel.length, 0);
        assert_eq!(square_channel.enabled, false);
    }

    // #[test]
    // fn test_square_channel_run() {
    //     let mut square_channel = SquareChannel::new(create_blipbuf(SAMPLES_RATE), true);
    //     assert_eq!(square_channel.last_amp, 0);
    //     assert_eq!(square_channel.volume_envelope.volume, 0);
    //     assert_eq!(square_channel.enabled, false);
    //
    //     square_channel.set(0xff19, 0x80);
    //     assert_eq!(square_channel.enabled, true);
    //
    //     square_channel.set(0xff17, 0xf0);
    //     assert_eq!(square_channel.volume_envelope.volume, 15);
    //
    //     todo!()
    //     // square_channel.run()
    // }
}
