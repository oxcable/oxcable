//! A simple delay module
//!
//! The delay filter plays back delayed copies of its input. The first delay is
//! played at full amplitude, and is then decayed linearly by the feedback
//! multiplier. The output is mixed with the raw input using the wetness
//! percentage.

#![experimental]


use std::vec::Vec;

use core::{SAMPLE_RATE, AudioDevice, Sample, Time};
use core::components::{InputArray, OutputArray};
use core::ringbuffer::RingBuffer;


pub struct Delay {
    pub inputs: InputArray,
    pub outputs: OutputArray,

    num_channels: uint,
    delay_buffers: Vec<RingBuffer<Sample>>,
    feedback: f32,
    wetness: f32
}

impl Delay {
    pub fn new(delay: f32, feedback: f32, wetness: f32, 
               num_channels: uint) -> Delay {
        // Create the delay buffers, starting with silence
        let delay_samples = (delay * SAMPLE_RATE as f32) as uint;
        let mut bufs = Vec::with_capacity(num_channels);
        for i in range(0, num_channels) {
            bufs.push(RingBuffer::new(delay_samples as uint));
            for _ in range(0, delay_samples) {
                bufs[i].push(0.0);
            }
        }

        Delay {
            inputs: InputArray::new(num_channels),
            outputs: OutputArray::new(num_channels),
            num_channels: num_channels,
            delay_buffers: bufs,
            feedback: feedback,
            wetness: wetness
        }
    }
}

impl AudioDevice for Delay {
    fn tick(&mut self, t: Time) {
        for i in range(0, self.num_channels) {
            let s = self.inputs.get(i, t).unwrap_or(0.0);

            // Get our delayed signal and feed it back with our input
            let delayed = self.delay_buffers[i].get(t).unwrap();
            self.delay_buffers[i].push(s + self.feedback*delayed);

            // Mix our wet signal with the input
            let out = self.wetness*delayed + (1.0-self.wetness)*s;
            self.outputs.push(i, out);
        }
    }
}
