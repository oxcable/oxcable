//! A simple delay filter.
//!
//! The delay filter plays back delayed copies of its input. The first delay is
//! played at full amplitude, and is then decayed linearly by the feedback
//! multiplier.
//!
//! The output is mixed with the raw input using the wetness percentage.

use types::{SAMPLE_RATE, AudioDevice, Sample, Time};
use utils::ringbuffer::RingBuffer;


/// Defines the messages that the Delay supports
#[derive(Clone, Copy, Debug)]
pub enum Message {
    SetDelay(f32),
    SetFeedback(f32),
    SetWetness(f32)
}
pub use self::Message::*;


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
    ///               delay; should be between 0.0 and 1.0.
    /// * `wetness`: how much of our input signal to mix into the delayed signal
    ///              in the output; should be between 0.0 and 1.0.
    pub fn new(delay: f32, feedback: f32, wetness: f32,
               num_channels: usize) -> Self {
        // Create the delay buffers, starting with silence
        let delay_samples = (delay * SAMPLE_RATE as f32) as usize;
        let init = vec![0.0; delay_samples];

        Delay {
            num_channels: num_channels,
            delay_buffers: vec![RingBuffer::from(&init[..]); num_channels],
            feedback: feedback,
            wetness: wetness
        }
    }

    /// Applies the message to our Delay
    pub fn handle_message(&mut self, msg: Message) {
        match msg {
            SetDelay(delay) => {
                let delay_samples = (delay * SAMPLE_RATE as f32) as usize;
                for rb in self.delay_buffers.iter_mut() {
                    rb.resize(delay_samples);
                }
            },
            SetFeedback(feedback) => {
                self.feedback = feedback;
            },
            SetWetness(wetness) => {
                self.wetness = wetness;
            }
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
            let delayed = self.delay_buffers[i][t];
            self.delay_buffers[i].push(s + self.feedback*delayed);

            // Mix our wet signal with the input
            outputs[i] = self.wetness*delayed + (1.0-self.wetness)*s;
        }
    }
}
