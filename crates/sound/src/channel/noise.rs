
/// Sound Channel 4 - Noise
///  This channel is used to output white noise.
///  This is done by randomly switching the amplitude between high and low at a given frequency.
///  Depending on the frequency the noise will appear “harder” or “softer”.
///  It is also possible to influence the function of the random generator,
///  so the that the output becomes more regular, resulting in a limited ability to output Tone instead of Noise.
struct SoundChannel4 {
    /// FF20 - NR41 - Channel 4 Sound Length (W)
    ///  Bit 5-0 - Sound length data (Write only) (t1: 0-63)
    /// Sound Length = (64-t1)*(1/256) seconds The Length value is used only if Bit 6 in NR44 is set.
    sound_length: u8,
    /// FF21 - NR42 - Channel 4 Volume Envelope (R/W)
    ///  Bit 7-4 - Initial Volume of envelope (0-0Fh) (0=No Sound)
    ///  Bit 3   - Envelope Direction (0=Decrease, 1=Increase)
    ///  Bit 2-0 - Number of envelope sweep (n: 0-7) `(If zero, stop envelope operation.)`
    /// Length of 1 step = n*(1/64) seconds
    volume_envelope: u8,
    /// FF22 - NR43 - Channel 4 Polynomial Counter (R/W)
    ///  The amplitude is randomly switched between high and low at the given frequency. A higher frequency will make the noise to appear “softer”. When Bit 3 is set, the output will become more regular, and some frequencies will sound more like Tone than Noise.
    ///  Bit 7-4 - Shift Clock Frequency (s)
    ///  Bit 3   - Counter Step/Width (0=15 bits, 1=7 bits)
    ///  Bit 2-0 - Dividing Ratio of Frequencies (r)
    /// Frequency = 524288 Hz / r / 2^(s+1) ;For r=0 assume r=0.5 instead
    polynomial_counter: u8,
    /// FF23 - NR44 - Channel 4 Counter/consecutive; Inital (R/W)
    ///  Bit 7   - Initial (1=Restart Sound)     (Write Only)
    ///  Bit 6   - Counter/consecutive selection (Read/Write)
    ///            (1=Stop output when length in NR41 expires)
    initial: bool,
    counter_consecutive: bool,
}

struct NoiseChannel {
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
            blip: blip,
        }
    }

    fn wb(&mut self, a: u16, v: u8) {
        match a {
            0xFF20 => self.new_length = 64 - (v & 0x3F),
            0xFF21 => (),
            0xFF22 => {
                self.shift_width = if v & 8 == 8 { 6 } else { 14 };
                let freq_div = match v & 7 {
                    0 => 8,
                    n => (n as u32 + 1) * 16,
                };
                self.period = freq_div << (v >> 4);
            },
            0xFF23 => {
                self.length_enabled = v & 0x40 == 0x40;
                if v & 0x80 == 0x80 {
                    self.enabled = true;
                    self.length = self.new_length;
                    self.state = 0xFF;
                    self.delay = 0;
                }
            },
            _ => (),
        }
        self.volume_envelope.wb(a, v);
    }

    fn on(&self) -> bool {
        self.enabled
    }

    fn run(&mut self, start_time: u32, end_time: u32) {
        if !self.enabled || self.volume_envelope.volume == 0 {
            if self.last_amp != 0 {
                self.blip.add_delta(start_time, -self.last_amp);
                self.last_amp = 0;
                self.delay = 0;
            }
        }
        else {
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

    fn step_length(&mut self) {
        if self.length_enabled && self.length != 0 {
            self.length -= 1;
            if self.length == 0 {
                self.enabled = false;
            }
        }
    }
}
