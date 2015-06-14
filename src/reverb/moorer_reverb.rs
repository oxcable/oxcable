//! A simple IIR reverb filter.
//!
//! This algorithmic reverb filter follows the basic design specified by James
//! Moorer in his seminal paper, "About This Reverberation Business".

use num::traits::Float;

use types::{SAMPLE_RATE, AudioDevice, DeviceIOType, Sample, Time};
use utils::ringbuffer::RingBuffer;
use utils::helpers::decibel_to_ratio;
use reverb::rooms::Room;


/// A multichannel reverb filter, that operates on each channel indepedently.
pub struct MoorerReverb {
    num_channels: usize,
    gain: f32,
    wetness: f32,

    tapped_delay_lines: Vec<RingBuffer<Sample>>,
    tapped_delays: Vec<Time>,
    tapped_gains: Vec<f32>,

    comb_delay_lines: Vec<Vec<RingBuffer<Sample>>>,
    comb_delays: Vec<Time>,
    comb_gains: Vec<f32>,
    comb_out_buffer: Vec<RingBuffer<Sample>>,
    comb_out_gain: f32,
}

impl MoorerReverb {
    /// Creates and initializes a new reverberator.
    ///
    /// * `room`: a `Room` struct, as described in `rooms.rs`. This specifies
    ///   aspects of the reverberation character.
    /// * `rev_time`: the -60dB time, in seconds
    /// * `gain`: the output gain, in decibels
    /// * `wetness`: how much of the input signal to mix into the output
    /// * `num_channels`: number of channels to process
    pub fn new(room: Room, rev_time: f32, gain: f32, wetness:f32,
           num_channels: usize) -> MoorerReverb {
        assert!(room.tapped_delays.len() == room.tapped_gains.len());

        // Calculate delays
        let max_tapped_delay = room.tapped_delays[room.tapped_delays.len()-1];
        let comb_out_delay = max_tapped_delay;

        // Calculate comb gains based on reverberation time
        let mut comb_gains = Vec::with_capacity(room.comb_delays.len());
        for i in (0 .. room.comb_delays.len()) {
            let gain = 10.0.powf(-3.0*(room.comb_delays[i] as f32) *
                                 rev_time / (SAMPLE_RATE as f32));
            comb_gains.push(gain);
        }
        let comb_out_gain = 1.0 - 0.366/rev_time;

        // Allocate tapped delay lines
        let mut tapped_delay_lines = Vec::with_capacity(num_channels);
        for _i in (0 .. num_channels) {
            let mut rb = RingBuffer::new(max_tapped_delay as usize + 1);
            for _t in (0 .. max_tapped_delay) { rb.push(0.0); }
            tapped_delay_lines.push(rb);
        }

        // Allocate comb delay lines
        let mut comb_delay_lines = Vec::with_capacity(num_channels);
        for _i in (0 .. num_channels) {
            let mut channel_lines = Vec::with_capacity(room.comb_delays.len());
            for j in (0 .. room.comb_delays.len()) {
                let delay = room.comb_delays[j];
                let mut rb = RingBuffer::new((delay+1) as usize);
                for _t in (0 .. delay) { rb.push(0.0); }
                channel_lines.push(rb);
            }
            comb_delay_lines.push(channel_lines);
        }
        let mut comb_out_buffer = Vec::with_capacity(num_channels);
        for _i in (0 .. num_channels) {
            let mut rb = RingBuffer::new((comb_out_delay+1) as usize);
            for _t in (0 .. comb_out_delay) { rb.push(0.0); }
            comb_out_buffer.push(rb);
        }

        // Return struct
        MoorerReverb {
            num_channels: num_channels,
            gain: decibel_to_ratio(gain),
            wetness: wetness,
            tapped_delay_lines: tapped_delay_lines,
            tapped_delays: Vec::from(room.tapped_delays),
            tapped_gains: Vec::from(room.tapped_gains),
            comb_delay_lines: comb_delay_lines,
            comb_delays: Vec::from(room.comb_delays),
            comb_gains: comb_gains,
            comb_out_buffer: comb_out_buffer,
            comb_out_gain: comb_out_gain,
        }
    }
}

impl AudioDevice for MoorerReverb {
    fn num_inputs(&self) -> DeviceIOType {
        DeviceIOType::Exactly(self.num_channels)
    }

    fn num_outputs(&self) -> DeviceIOType {
        DeviceIOType::Exactly(self.num_channels)
    }

    fn tick(&mut self, t: Time, inputs: &[Sample], outputs: &mut[Sample]) {
        for (i,x) in inputs.iter().enumerate() {
            // Advance tapped delay line
            self.tapped_delay_lines[i].push(0.0);
            for (delay, gain) in self.tapped_delays.iter()
                .zip(self.tapped_gains.iter()) {
                self.tapped_delay_lines[i].add(t + *delay, *gain * x).unwrap();
            }
            let tapped_out = (x + self.tapped_delay_lines[i].get(t).unwrap()) /
                (self.tapped_delays.len() as f32);

            // Update comb filters
            let mut comb_out = 0.0;
            for j in (0 .. self.comb_delays.len()) {
                let gain  = self.comb_gains[j];
                let feedback = self.comb_delay_lines[i][j].get(t).unwrap();
                self.comb_delay_lines[i][j].push(tapped_out + gain * feedback);
                comb_out += feedback;
            }
            comb_out *= self.comb_out_gain/(self.comb_delays.len() as f32);
            self.comb_out_buffer[i].push(comb_out);


            // Finally store result
            let wet_out = tapped_out + self.comb_out_buffer[i].get(t).unwrap();
            let y = self.gain * (self.wetness*wet_out + (1.0-self.wetness)*x);
            outputs[i] = y;
        }
    }
}
