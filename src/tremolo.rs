//! A tremolo filter.
//!
//! Tremolo modifies the amplitude of an incoming signal (in the first channel), using
//! the output of a low frequency oscillator (in the second channel). It creates
//! a shuddering effect in the output audio.
//!
//! A intensity is set, in decibels. This controls how much gain or attenuation
//! the tremolo will apply. The LFO will then be used to oscillate this gain
//! over time.
//!
//! # Example
//!
//! To set up a tremolo, the channels must be properly configured. The following
//! will apply a tremolo to our microphone input, using a 10Hz LFO:
//!
//! ```
//! use oxcable::graph::DeviceGraph;
//! use oxcable::io::audio::AudioEngine;
//! use oxcable::oscillator::*;
//! use oxcable::tremolo::Tremolo;
//!
//! # // Wrap in a dummy function to prevent running in doctest.
//! # fn dummy() {
//! let engine = AudioEngine::with_buffer_size(256).unwrap();
//! let mut graph = DeviceGraph::new();
//!
//! let lfo = graph.add_node(Oscillator::new(Sine).freq(10.0));
//! let microphone = graph.add_node(engine.default_input(1).unwrap());
//! let tremolo = graph.add_node(Tremolo::new(3.0)); // 3dB tremolo
//! let speaker = graph.add_node(engine.default_output(1).unwrap());
//!
//! graph.add_edge(microphone, 0, tremolo, 0); // first channnel is input signal
//! graph.add_edge(lfo, 0, tremolo, 1); // second channel is LFO output
//! graph.add_edge(tremolo, 0, speaker, 0);
//! # }
//! ```

use num::traits::Float;

use types::{AudioDevice, MessageReceiver, Sample, Time};


/// Defines the messages that the Tremolo supports.
#[derive(Clone, Copy, Debug)]
pub enum Message {
    /// Set the tremolo intensity, in decibels.
    SetIntensity(f32)
}
pub use self::Message::*;


/// A tremolo filter.
pub struct Tremolo {
    intensity: f32,
}

impl Tremolo {
    /// Returns a new single-channel tremolo filter.
    ///
    /// `intensity` is in decibels.
    pub fn new(intensity: f32) -> Self {
        Tremolo {
            intensity: intensity
        }
    }
}

impl MessageReceiver for Tremolo {
    type Msg = Message;
    fn handle_message(&mut self, msg: Message) {
        match msg {
            SetIntensity(intensity) => self.intensity = intensity
        }
    }
}

impl AudioDevice for Tremolo {
    fn num_inputs(&self) -> usize {
        2
    }

    fn num_outputs(&self) -> usize {
        1
    }

    fn tick(&mut self, _: Time, inputs: &[Sample], outputs: &mut[Sample]) {
        let gain = 10.0.powf(inputs[1] * self.intensity/10.0);
        outputs[0] = gain*inputs[0];
    }
}
