/// Sound Channel 1 - Tone & Sweep
struct SoundChannel1 {
    ///  FF10 - NR10 - Channel 1 Sweep register (R/W)
    ///   Bit 6-4 - Sweep Time
    ///   Bit 3   - Sweep Increase/Decrease
    ///              0: Addition    (frequency increases)
    ///              1: Subtraction (frequency decreases)
    ///   Bit 2-0 - Number of sweep shift (n: 0-7)
    sweep_reg: u8,
    ///  FF11 - NR11 - Channel 1 Sound length/Wave pattern duty (R/W)
    ///   Bit 7-6 - Wave Pattern Duty (Read/Write)
    ///   Bit 5-0 - Sound length data (Write Only) (t1: 0-63)
    slengnth_wavepattern: u8,
    ///  FF12 - NR12 - Channel 1 Volume Envelope (R/W)
    ///   Bit 7-4 - Initial Volume of envelope (0-0Fh) (0=No Sound)
    ///   Bit 3   - Envelope Direction (0=Decrease, 1=Increase)
    ///   Bit 2-0 - Number of envelope sweep (n: 0-7)
    ///            (If zero, stop envelope operation.)
    ///  Length of 1 step = n*(1/64) seconds
    volume_envelope: VolumeEnvelope,
    ///  FF13 - NR13 - Channel 1 Frequency lo (Write Only)
    ///   Lower 8 bits of 11 bit frequency (x). Next 3 bit are in NR14 ($FF14)
    ///
    ///  FF14 - NR14 - Channel 1 Frequency hi (R/W)
    ///   Bit 7   - Initial (1=Restart Sound)     (Write Only)
    ///   Bit 6   - Counter/consecutive selection (Read/Write)
    ///           (1=Stop output when length in NR11 expires)
    ///   Bit 2-0 - Frequency's higher 3 bits (x) (Write Only)
    frequency: u16,
}

/// Sound Channel 2 - Tone
///  `This sound channel works exactly as channel 1, except that it doesn’t have a Tone Envelope/Sweep Register.`
struct SoundChannel2 {
    ///  FF16 - NR21 - Channel 2 Sound Length/Wave Pattern Duty (R/W)
    ///   Bit 7-6 - Wave Pattern Duty (Read/Write)
    ///   Bit 5-0 - Sound length data (Write Only) (t1: 0-63)
    slengnth_wavepattern: u8,
    ///  FF17 - NR22 - Channel 2 Volume Envelope (R/W)
    ///   Bit 7-4 - Initial Volume of envelope (0-0Fh) (0=No Sound)
    ///   Bit 3   - Envelope Direction (0=Decrease, 1=Increase)
    ///   Bit 2-0 - Number of envelope sweep (n: 0-7) `(If zero, stop envelope operation.)`
    volume_envelope: u8,
    ///  FF18 - NR23 - Channel 2 Frequency lo data (W)
    ///   Frequency’s lower 8 bits of 11 bit data (x). Next 3 bits are in NR24 ($FF19).
    ///
    ///  FF19 - NR24 - Channel 2 Frequency hi data (R/W)
    ///   Bit 7   - Initial (1=Restart Sound)     (Write Only)`
    ///   Bit 6   - Counter/consecutive selection (Read/Write)`
    ///             (1=Stop output when length in NR21 expires)`
    ///   Bit 2-0 - Frequency's higher 3 bits (x) (Write Only)`
    frequency: u16,
}


struct SquareChannel {
    enabled : bool,
    duty : u8,
    phase : u8,
    length: u8,
    new_length: u8,
    length_enabled : bool,
    frequency: u16,
    period: u32,
    last_amp: i32,
    delay: u32,
    has_sweep : bool,
    sweep_frequency: u16,
    sweep_delay: u8,
    sweep_period: u8,
    sweep_shift: u8,
    sweep_frequency_increase: bool,
    volume_envelope: VolumeEnvelope,
    blip: BlipBuf,
}

impl SquareChannel {
    fn new(blip: BlipBuf, with_sweep: bool) -> SquareChannel {
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

    fn on(&self) -> bool {
        self.enabled
    }

    fn wb(&mut self, a: u16, v: u8) {
        match a {
            0xFF10 => {
                self.sweep_period = (v >> 4) & 0x7;
                self.sweep_shift = v & 0x7;
                self.sweep_frequency_increase = v & 0x8 == 0x8;
            },
            0xFF11 | 0xFF16 => {
                self.duty = v >> 6;
                self.new_length = 64 - (v & 0x3F);
            },
            0xFF13 | 0xFF18 => {
                self.frequency = (self.frequency & 0x0700) | (v as u16);
                self.length = self.new_length;
                self.calculate_period();
            },
            0xFF14 | 0xFF19 => {
                self.frequency = (self.frequency & 0x00FF) | (((v & 0b0000_0111) as u16) << 8);
                self.calculate_period();
                self.length_enabled = v & 0x40 == 0x40;

                if v & 0x80 == 0x80 {
                    self.enabled = true;
                    self.length = self.new_length;

                    self.sweep_frequency = self.frequency;
                    if self.has_sweep && self.sweep_period > 0 && self.sweep_shift > 0 {
                        self.sweep_delay = 1;
                        self.step_sweep();
                    }
                }
            },
            _ => (),
        }
        self.volume_envelope.wb(a, v);
    }

    fn calculate_period(&mut self) {
        if self.frequency > 2048 { self.period = 0; }
        else { self.period = (2048 - self.frequency as u32) * 4; }
    }

    // This assumes no volume or sweep adjustments need to be done in the meantime
    fn run(&mut self, start_time: u32, end_time: u32) {
        if !self.enabled || self.period == 0 || self.volume_envelope.volume == 0 {
            if self.last_amp != 0 {
                self.blip.add_delta(start_time, -self.last_amp);
                self.last_amp = 0;
                self.delay = 0;
            }
        }
        else {
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

    fn step_length(&mut self) {
        if self.length_enabled && self.length != 0 {
            self.length -= 1;
            if self.length == 0 {
                self.enabled = false;
            }
        }
    }

    fn step_sweep(&mut self) {
        if !self.has_sweep || self.sweep_period == 0 { return; }

        if self.sweep_delay > 1 {
            self.sweep_delay -= 1;
        }
        else {
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
                }
                else {
                    self.sweep_frequency -= offset;
                }
            }
            else {
                if self.sweep_frequency >= 2048 - offset {
                    self.sweep_frequency = 2048;
                }
                else {
                    self.sweep_frequency += offset;
                }
            }
        }
    }
}