use crate::channel::noise::NoiseChannel;
use crate::channel::square::SquareChannel;
use crate::channel::wave::WaveChannel;
use crate::player::AudioPlayer;
use blip_buf::BlipBuf;
use shared::Error;

const CLOCKS_PER_SECOND: u32 = 1 << 22;
const OUTPUT_SAMPLE_COUNT: usize = 2000; // this should be less than blip_buf::MAX_FRAME

pub fn create_blipbuf(samples_rate: u32) -> BlipBuf {
    let mut blipbuf = BlipBuf::new(samples_rate);
    blipbuf.set_rates(CLOCKS_PER_SECOND as f64, samples_rate as f64);
    blipbuf
}

#[derive(Debug)]
pub struct Sound {
    on: bool,
    registerdata: [u8; 23],
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

impl Sound {
    pub fn new(player: Box<dyn AudioPlayer>) -> Sound {
        let blipbuf1 = create_blipbuf(player.samples_rate());
        let blipbuf2 = create_blipbuf(player.samples_rate());
        let blipbuf3 = create_blipbuf(player.samples_rate());
        let blipbuf4 = create_blipbuf(player.samples_rate());

        let output_period =
            (OUTPUT_SAMPLE_COUNT as u64 * CLOCKS_PER_SECOND as u64) / player.samples_rate() as u64;

        Sound {
            on: false,
            registerdata: [0; 23],
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
            player,
        }
    }

    #[allow(dead_code)]
    fn get(&mut self, address: usize) -> u8 {
        self.run();
        match address {
            0xff10..=0xff25 => self.registerdata[address as usize - 0xff10],
            0xff26 => {
                (self.registerdata[address as usize - 0xff10] & 0xf0)
                    | (if self.channel1.on() { 1 } else { 0 })
                    | (if self.channel2.on() { 2 } else { 0 })
                    | (if self.channel3.on() { 4 } else { 0 })
                    | (if self.channel4.on() { 8 } else { 0 })
            }
            0xff30..=0xff3f => {
                (self.channel3.ram[(address as usize - 0xff30) / 2] << 4)
                    | self.channel3.ram[(address as usize - 0xff30) / 2 + 1]
            }
            _ => 0,
        }
    }

    #[allow(dead_code)]
    fn set(&mut self, address: usize, data: u8) -> Result<(), Error> {
        if address != 0xff26 && !self.on {
            return Err(shared::Error::IllegalSet(address, data));
        }
        self.run();
        if (0xff10..=0xff26).contains(&address) {
            self.registerdata[address as usize - 0xff10] = data;
        }
        match address {
            0xff10..=0xff14 => self.channel1.set(address, data),
            0xff16..=0xff19 => self.channel2.set(address, data),
            0xff1a..=0xff1e => self.channel3.set(address, data),
            0xff20..=0xff23 => self.channel4.set(address, data),
            0xff24 => {
                self.volume_left = data & 0x7;
                self.volume_right = (data >> 4) & 0x7;
            }
            0xff25 => (),
            0xff26 => self.on = data & 0x80 == 0x80,
            0xff30..=0xff3f => self.channel3.set(address, data),
            _ => return Err(shared::Error::IllegalSet(address, data)),
        }
        Ok(())
    }

    pub fn do_cycle(&mut self, cycles: u32) {
        if !self.on {
            return;
        }

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
        } else {
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
            } else if self.time_divider & 1 == 1 {
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

    fn set_bufs_register(
        &mut self,
        buffer: &mut [f32; 2010],
        offset: u8,
        volume: f32,
        data: f32,
        index: usize,
    ) {
        if self.registerdata[0x15] & offset == offset {
            buffer[index] += data * volume;
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
                self.set_bufs_register(buf_left, 0x01, left_vol, *v as f32, i);
                self.set_bufs_register(buf_right, 0x10, right_vol, *v as f32, i);
            }

            let count2 = self.channel2.blip.read_samples(buf, false);
            for (i, v) in buf[..count2].iter().enumerate() {
                self.set_bufs_register(buf_left, 0x02, left_vol, *v as f32, i);
                self.set_bufs_register(buf_right, 0x20, right_vol, *v as f32, i);
            }

            // channel3 is the WaveChannel, that outputs samples with a 4x
            // increase in amplitude in order to avoid a loss of precision.
            let count3 = self.channel3.blip.read_samples(buf, false);
            for (i, v) in buf[..count3].iter().enumerate() {
                self.set_bufs_register(buf_left, 0x04, left_vol, (*v as f32) / 4.0, i);
                self.set_bufs_register(buf_right, 0x40, right_vol, (*v as f32) / 4.0, i);
            }

            let count4 = self.channel4.blip.read_samples(buf, false);
            for (i, v) in buf[..count4].iter().enumerate() {
                self.set_bufs_register(buf_left, 0x08, left_vol, *v as f32, i);
                self.set_bufs_register(buf_right, 0x80, right_vol, *v as f32, i);
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

// #[cfg(test)]
// mod sound_test {
//     use super::Sound;
//     use crate::CpalPlayer;
//
//     const fILE: &[u8; 2097152] = include_bytes!(
//         "/Users/guvillat/GBMU/roms/Pokemon - Version Or.gbc"
//     );
//
// #[test]
// fn test_sound_new() {
//     let sound = Sound::new(Box::new(v) as Box<dyn CpalPlayer>);
//     sound.
// }
//
// #[test]
// fn test_sound_get_ff10() {
//
// }
// }
