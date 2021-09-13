use crate::channel;
use shared::{traits::Bus, Error};
use blip_buf::BlipBuf;

const WAVE_PATTERN : [[i32; 8]; 4] = [[-1,-1,-1,-1,1,-1,-1,-1],[-1,-1,-1,-1,1,1,-1,-1],[-1,-1,1,1,1,1,-1,-1],[1,1,1,1,-1,-1,1,1]];
const CLOCKS_PER_SECOND : u32 = 1 << 22;
const OUTPUT_SAMPLE_COUNT : usize = 2000; // this should be less than blip_buf::MAX_FRAME

pub trait AudioPlayer : Send {
    fn play(&mut self, left_channel: &[f32], right_channel: &[f32]);
    fn samples_rate(&self) -> u32;
    fn underflowed(&self) -> bool;
}

fn create_blipbuf(samples_rate: u32) -> BlipBuf {
    let mut blipbuf = BlipBuf::new(samples_rate);
    blipbuf.set_rates(CLOCKS_PER_SECOND as f64, samples_rate as f64);
    blipbuf
}

pub struct Sound {
    on: bool,
    registerdata: [u8; 0x17],
    time: u32,
    prev_time: u32,
    next_time: u32,
    time_divider: u8,
    output_period: u32,
    channel1: SquareChannel,
    channel2: SquareChannel,
    channel3: WaveChannel,
    channel4: NoiseChannel,
    volume_left: u8,
    volume_right: u8,
    need_sync: bool,
    player: Box<dyn AudioPlayer>,
}

impl Bus<usize> for Sound {
    type Item = u8;
    type Result = Result<(), Error>;
    type Data = u8;

    fn get(&self, _: usize) -> Self::Item {
        todo!()
    }

    fn set(&mut self, _: usize, data: Self::Data) -> Self::Result {
        todo!()
    }
}

impl Sound {
    pub fn new(player: Box<dyn AudioPlayer>) -> Sound {
        let blipbuf1 = create_blipbuf(player.samples_rate());
        let blipbuf2 = create_blipbuf(player.samples_rate());
        let blipbuf3 = create_blipbuf(player.samples_rate());
        let blipbuf4 = create_blipbuf(player.samples_rate());

        let output_period = (OUTPUT_SAMPLE_COUNT as u64 * CLOCKS_PER_SECOND as u64) / player.samples_rate() as u64;

        Sound {
            on: false,
            registerdata: [0; 0x17],
            time: 0,
            prev_time: 0,
            next_time: CLOCKS_PER_SECOND / 256,
            time_divider: 0,
            output_period: output_period as u32,
            channel1: SquareChannel::new(blipbuf1, true),
            channel2: SquareChannel::new(blipbuf2, false),
            channel3: WaveChannel::new(blipbuf3),
            channel4: NoiseChannel::new(blipbuf4),
            volume_left: 7,
            volume_right: 7,
            need_sync: false,
            player: player,
        }
    }

    pub fn rb(&mut self, a: u16) -> u8 {
        self.run();
        match a {
            0xFF10 ..= 0xFF25 => self.registerdata[a as usize - 0xFF10],
            0xFF26 => {
                (self.registerdata[a as usize - 0xFF10] & 0xF0)
                    | (if self.channel1.on() { 1 } else { 0 })
                    | (if self.channel2.on() { 2 } else { 0 })
                    | (if self.channel3.on() { 4 } else { 0 })
                    | (if self.channel4.on() { 8 } else { 0 })
            }
            0xFF30 ..= 0xFF3F => {
                (self.channel3.waveram[(a as usize - 0xFF30) / 2] << 4) |
                    self.channel3.waveram[(a as usize - 0xFF30) / 2 + 1]
            },
            _ => 0,
        }
    }

    pub fn wb(&mut self, a: u16, v: u8) {
        if a != 0xFF26 && !self.on { return; }
        self.run();
        if a >= 0xFF10 && a <= 0xFF26 {
            self.registerdata[a as usize - 0xFF10] = v;
        }
        match a {
            0xFF10 ..= 0xFF14 => self.channel1.wb(a, v),
            0xFF16 ..= 0xFF19 => self.channel2.wb(a, v),
            0xFF1A ..= 0xFF1E => self.channel3.wb(a, v),
            0xFF20 ..= 0xFF23 => self.channel4.wb(a, v),
            0xFF24 => {
                self.volume_left = v & 0x7;
                self.volume_right = (v >> 4) & 0x7;
            }
            0xFF26 => self.on = v & 0x80 == 0x80,
            0xFF30 ..= 0xFF3F => self.channel3.wb(a, v),
            _ => (),
        }
    }

    pub fn do_cycle(&mut self, cycles: u32)
    {
        if !self.on { return; }

        self.time += cycles;

        if self.time >= self.output_period {
            self.do_output();
        }
    }

    pub fn sync(&mut self) {
        self.need_sync = true;
    }

    fn do_output(&mut self) {
        self.run();
        debug_assert!(self.time == self.prev_time);
        self.channel1.blip.end_frame(self.time);
        self.channel2.blip.end_frame(self.time);
        self.channel3.blip.end_frame(self.time);
        self.channel4.blip.end_frame(self.time);
        self.next_time -= self.time;
        self.time = 0;
        self.prev_time = 0;

        if !self.need_sync || self.player.underflowed() {
            self.need_sync = false;
            self.mix_buffers();
        }
        else {
            // Prevent the BlipBuf's from filling up and triggering an assertion
            self.clear_buffers();
        }
    }

    fn run(&mut self) {
        while self.next_time <= self.time {
            self.channel1.run(self.prev_time, self.next_time);
            self.channel2.run(self.prev_time, self.next_time);
            self.channel3.run(self.prev_time, self.next_time);
            self.channel4.run(self.prev_time, self.next_time);

            self.channel1.step_length();
            self.channel2.step_length();
            self.channel3.step_length();
            self.channel4.step_length();

            if self.time_divider == 0 {
                self.channel1.volume_envelope.step();
                self.channel2.volume_envelope.step();
                self.channel4.volume_envelope.step();
            }
            else if self.time_divider & 1 == 1 {
                self.channel1.step_sweep();
            }

            self.time_divider = (self.time_divider + 1) % 4;
            self.prev_time = self.next_time;
            self.next_time += CLOCKS_PER_SECOND / 256;
        }

        if self.prev_time != self.time {
            self.channel1.run(self.prev_time, self.time);
            self.channel2.run(self.prev_time, self.time);
            self.channel3.run(self.prev_time, self.time);
            self.channel4.run(self.prev_time, self.time);

            self.prev_time = self.time;
        }
    }

    fn mix_buffers(&mut self) {
        let sample_count = self.channel1.blip.samples_avail() as usize;
        debug_assert!(sample_count == self.channel2.blip.samples_avail() as usize);
        debug_assert!(sample_count == self.channel3.blip.samples_avail() as usize);
        debug_assert!(sample_count == self.channel4.blip.samples_avail() as usize);

        let mut outputted = 0;

        let left_vol = (self.volume_left as f32 / 7.0) * (1.0 / 15.0) * 0.25;
        let right_vol = (self.volume_right as f32 / 7.0) * (1.0 / 15.0) * 0.25;

        while outputted < sample_count {
            let buf_left = &mut [0f32; OUTPUT_SAMPLE_COUNT + 10];
            let buf_right = &mut [0f32; OUTPUT_SAMPLE_COUNT + 10];
            let buf = &mut [0i16; OUTPUT_SAMPLE_COUNT + 10];

            let count1 = self.channel1.blip.read_samples(buf, false);
            for (i, v) in buf[..count1].iter().enumerate() {
                if self.registerdata[0x15] & 0x01 == 0x01 {
                    buf_left[i] += *v as f32 * left_vol;
                }
                if self.registerdata[0x15] & 0x10 == 0x10 {
                    buf_right[i] += *v as f32 * right_vol;
                }
            }

            let count2 = self.channel2.blip.read_samples(buf, false);
            for (i, v) in buf[..count2].iter().enumerate() {
                if self.registerdata[0x15] & 0x02 == 0x02 {
                    buf_left[i] += *v as f32 * left_vol;
                }
                if self.registerdata[0x15] & 0x20 == 0x20 {
                    buf_right[i] += *v as f32 * right_vol;
                }
            }

            // channel3 is the WaveChannel, that outputs samples with a 4x
            // increase in amplitude in order to avoid a loss of precision.
            let count3 = self.channel3.blip.read_samples(buf, false);
            for (i, v) in buf[..count3].iter().enumerate() {
                if self.registerdata[0x15] & 0x04 == 0x04 {
                    buf_left[i] += ((*v as f32) / 4.0) * left_vol;
                }
                if self.registerdata[0x15] & 0x40 == 0x40 {
                    buf_right[i] += ((*v as f32) / 4.0) * right_vol;
                }
            }

            let count4 = self.channel4.blip.read_samples(buf, false);
            for (i, v) in buf[..count4].iter().enumerate() {
                if self.registerdata[0x15] & 0x08 == 0x08 {
                    buf_left[i] += *v as f32 * left_vol;
                }
                if self.registerdata[0x15] & 0x80 == 0x80 {
                    buf_right[i] += *v as f32 * right_vol;
                }
            }

            debug_assert!(count1 == count2);
            debug_assert!(count1 == count3);
            debug_assert!(count1 == count4);

            self.player.play(&buf_left[..count1], &buf_right[..count1]);

            outputted += count1;
        }
    }

    fn clear_buffers(&mut self) {
        self.channel1.blip.clear();
        self.channel2.blip.clear();
        self.channel3.blip.clear();
        self.channel4.blip.clear();
    }
}
