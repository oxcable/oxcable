//! Provides a simple delay module.

#![experimental]


use std::vec::Vec;

use core::components::{InputArray, OutputArray};
use core::ringbuffer::RingBuffer;
use core::types::{SAMPLE_RATE, Device, Sample, Time};


/// A delay that feeds back each channel independently.
///
/// The delay filter plays back delayed copies of its input. The first delay is
/// played at full amplitude, and is then decayed linearly by the feedback
/// multiplier. The output is mixed with the raw input using the wetness
/// percentage.
pub struct Delay {
    /// Input audio channels
    pub inputs: InputArray<Sample>,
    /// Output audio channels
    pub outputs: OutputArray<Sample>,

    num_channels: usize,
    delay_buffers: Vec<RingBuffer<Sample>>,
    feedback: f32,
    wetness: f32
}

impl Delay {
    /// Creates a new Delay object.
    ///
    /// * `delay`: the time to delay the input, in seconds
    /// * `feedback`: how much of our delayed signal to feed back into the next
    ///               delay
    /// * `wetness`: how much of our input signal to mix into the delayed signal
    ///              in the output
    pub fn new(delay: f32, feedback: f32, wetness: f32, 
               num_channels: usize) -> Delay {
        // Create the delay buffers, starting with silence
        let delay_samples = (delay * SAMPLE_RATE as f32) as u32;
        let mut bufs = Vec::with_capacity(num_channels);
        for i in (0 .. num_channels) {
            bufs.push(RingBuffer::new(delay_samples as usize));
            for _ in (0 .. delay_samples) {
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

impl Device for Delay {
    fn tick(&mut self, t: Time) {
        for i in (0 .. self.num_channels) {
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
