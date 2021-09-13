
/// Sound Channel 3 - Wave Output
///  This channel can be used to output digital sound, the length of the sample buffer (Wave RAM) is limited to 32 digits.
///  This sound channel can be also used to output normal tones when initializing the Wave RAM by a square wave.
///  This channel doesn’t have a volume envelope register.
struct SoundChannel3 {
    /// FF1A - NR30 - Channel 3 Sound on/off (R/W)
    ///  Bit 7 - Sound Channel 3 Off  (0=Stop, 1=Playback)  (Read/Write)
    sound_enable: bool,
    /// FF1B - NR31 - Channel 3 Sound Length (W)
    ///  Bit 7-0 - Sound length (Write only) (t1: 0 - 255)
    ///  Sound Length = (256-t1)*(1/256) seconds This value is used only if Bit 6 in NR34 is set.
    sound_length: u8,
    /// FF1C - NR32 - Channel 3 Select output level (R/W)
    ///  Bits 6-5 - Select output level (Read/Write)
    ///  Bits 6-5	Output level
    ///  %00	Mute (No sound)
    ///  %01	100% volume (Produce Wave Pattern RAM Data as it is)
    ///  %10	50% volume (Produce Wave Pattern RAM data shifted once to the right)
    ///  %11	25% volume (Produce Wave Pattern RAM data shifted twice to the right)
    output_lvl: u8,
    /// FF1D - NR33 - Channel 3 Frequency’s lower data (W)
    ///  Lower 8 bits of an 11 bit frequency (x).
    ///
    /// FF1E - NR34 - Channel 3 Frequency’s higher data (R/W)
    ///  Bit 7   - Initial (1=Restart Sound)     (Write Only)`
    ///  Bit 6   - Counter/consecutive selection (Read/Write)`
    ///           (1=Stop output when length in NR31 expires)`
    ///  Bit 2-0 - Frequency's higher 3 bits (x) (Write Only)`
    frequency: u16,
    /// FF30-FF3F - Wave Pattern RAM
    ///  Contents - Waveform storage for arbitrary sound data
    ///  This storage area holds 32 4-bit samples that are played back, upper 4 bits first.
    ///  Wave RAM should only be accessed while CH3 is disabled (NR30 bit 7 reset), otherwise accesses will behave weirdly.
    ///  On almost all models, the byte will be written at the offset CH3 is currently reading.
    ///  On GBA, the write will simply be ignored.
    wave_patt_ram: [u8; 32],
}

struct WaveChannel {
    enabled : bool,
    enabled_flag : bool,
    length: u16,
    new_length: u16,
    length_enabled : bool,
    frequency: u16,
    period: u32,
    last_amp: i32,
    delay: u32,
    volume_shift: u8,
    waveram: [u8; 32],
    current_wave: u8,
    blip: BlipBuf,
}

impl WaveChannel {
    fn new(blip: BlipBuf) -> WaveChannel {
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
            waveram: [0; 32],
            current_wave: 0,
            blip: blip,
        }
    }

    fn wb(&mut self, a: u16, v: u8) {
        match a {
            0xFF1A => {
                self.enabled_flag = (v & 0x80) == 0x80;
                self.enabled = self.enabled && self.enabled_flag;
            }
            0xFF1B => self.new_length = 256 - (v as u16),
            0xFF1C => self.volume_shift = (v >> 5) & 0b11,
            0xFF1D => {
                self.frequency = (self.frequency & 0x0700) | (v as u16);
                self.calculate_period();
            }
            0xFF1E => {
                self.frequency = (self.frequency & 0x00FF) | (((v & 0b111) as u16) << 8);
                self.calculate_period();
                self.length_enabled = v & 0x40 == 0x40;
                if v & 0x80 == 0x80 && self.enabled_flag {
                    self.length = self.new_length;
                    self.enabled = true;
                    self.current_wave = 0;
                    self.delay = 0;
                }
            },
            0xFF30 ..= 0xFF3F => {
                self.waveram[(a as usize - 0xFF30) / 2] = v >> 4;
                self.waveram[(a as usize - 0xFF30) / 2 + 1] = v & 0xF;
            },
            _ => (),
        }
    }

    fn calculate_period(&mut self) {
        if self.frequency > 2048 { self.period = 0; }
        else { self.period = (2048 - self.frequency as u32) * 2; }
    }

    fn on(&self) -> bool {
        self.enabled
    }

    fn run(&mut self, start_time: u32, end_time: u32) {
        if !self.enabled || self.period == 0 {
            if self.last_amp != 0 {
                self.blip.add_delta(start_time, -self.last_amp);
                self.last_amp = 0;
                self.delay = 0;
            }
        }
        else {
            let mut time = start_time + self.delay;

            // A sample may be muted, 100%, 50% or 25%.
            // To avoid loss of precision, the WaveChannel will output
            // i32 samples at 4x the usual amplitude. This will be taken
            // into account when mixing all the samples.
            let volshift = match self.volume_shift {
                0 => 4 + 2,  // to mute a 4 bit sample mutiplied by 2^2
                1 => 0,
                2 => 1,
                3 => 2,
                _ => unreachable!(),
            };

            while time < end_time {
                let sample = self.waveram[self.current_wave as usize];

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

    fn step_length(&mut self) {
        if self.length_enabled && self.length != 0 {
            self.length -= 1;
            if self.length == 0 {
                self.enabled = false;
            }
        }
    }
}
