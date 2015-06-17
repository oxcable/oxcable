//! Provides a simple delay module.


use std::vec::Vec;

use types::{SAMPLE_RATE, AudioDevice, Sample, Time};
use utils::ringbuffer::RingBuffer;


/// A delay that feeds back each channel independently.
///
/// The delay filter plays back delayed copies of its input. The first delay is
/// played at full amplitude, and is then decayed linearly by the feedback
/// multiplier. The output is mixed with the raw input using the wetness
/// percentage.
pub struct Delay {
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
            num_channels: num_channels,
            delay_buffers: bufs,
            feedback: feedback,
            wetness: wetness
        }
    }
}

impl AudioDevice for Delay {
    fn num_inputs(&self) -> usize {
        self.num_channels
    }

    fn num_outputs(&self) -> usize {
        self.num_channels
    }

    fn tick(&mut self, t: Time, inputs: &[Sample], outputs: &mut[Sample]) {
        for (i,s) in inputs.iter().enumerate() {
            // Get our delayed signal and feed it back with our input
            let delayed = self.delay_buffers[i].get(t).unwrap();
            self.delay_buffers[i].push(s + self.feedback*delayed);

            // Mix our wet signal with the input
            outputs[i] = self.wetness*delayed + (1.0-self.wetness)*s;
        }
    }
}
