//! A container for a series of audio devices.
//!
//! A chain can be used when a single series of audio devices passes its output
//! to the input of the next device. It is initialized from a single starting
//! device that will receive no input, and ends in a device who's output is
//! ignored.
//!
//!
//! # Example
//! The following will pass microphone input through a delay unit, then out to
//! the speaker:
//!
//! ```
//! # // Wrap in a dummy function to prevent running in doctest.
//! # fn dummy() {
//! use oxcable::delay::Delay;
//! use oxcable::chain::{DeviceChain, Tick};
//! use oxcable::io::audio::AudioEngine;
//!
//! let engine = AudioEngine::with_buffer_size(256).unwrap();
//! let mut chain = DeviceChain::from(
//!     engine.default_input(1).unwrap()
//! ).into(
//!     Delay::new(1.0, 0.5, 0.5, 1)
//! ).into(
//!     engine.default_output(1).unwrap()
//! );
//! chain.tick_forever();
//! # }
//! ```


use types::{AudioDevice, Sample, Time};
pub use tick::Tick;


/// A container for a series of audio devices.
pub struct DeviceChain {
    inputs: Vec<Sample>,
    devices: Vec<AudioNode>,
    time: Time
}

impl DeviceChain {
    /// Creates a new chain that starts from the provided device. This device
    /// will receive no inputs unless they are manually supplied using
    /// DeviceChain::get_input.
    pub fn from<D>(device: D) -> Self where D: 'static+AudioDevice {
        DeviceChain {
            inputs: vec![0.0; device.num_inputs()],
            devices: vec![AudioNode::new(device)],
            time: 0
        }
    }

    /// Appends the provided device to the end of the chain. This device will be
    /// passed the output of the last device as input. This method returns the
    /// same chain it was passed.
    pub fn into<D>(mut self, device: D) -> Self where D: 'static+AudioDevice {
        if self.devices[self.devices.len()-1].outputs.len() != device.num_inputs() {
            panic!("DeviceChain: number of outputs must match number of inputs");
        }
        self.devices.push(AudioNode::new(device));
        self
    }

    /// Return a mutable slice to the input of the first device in the chain.
    ///
    /// These inputs never get overwritten, so if you are supplying input you
    /// must manually zero the buffer again.
    pub fn get_input(&mut self) -> &mut[Sample] {
        &mut self.inputs
    }

    /// Returns a slice to the output of the last device in the chain.
    pub fn get_output(&self) -> &[Sample] {
        &self.devices[self.devices.len()-1].outputs
    }
}

impl Tick for DeviceChain {
    fn tick(&mut self) {
        self.devices[0].tick(self.time, &self.inputs);
        for i in 1..self.devices.len() {
            let inputs = self.devices[i-1].outputs.clone();
            self.devices[i].tick(self.time, &inputs);
        }
        self.time += 1;
    }
}


/// Wrap an audio device behind a pointer, and provide an allocated buffer to
/// contain its output.
struct AudioNode {
    device: Box<AudioDevice>,
    outputs: Vec<Sample>
}

impl AudioNode {
    /// Wraps the provided audio device in a new node and allocate its output
    /// buffer.
    fn new<D>(device: D) -> AudioNode where D: 'static+AudioDevice {
        let n = device.num_outputs();
        AudioNode {
            device: Box::new(device),
            outputs: vec![0.0; n]
        }
    }

    /// Ticks the device one time step.
    ///
    /// `inputs` should be the output of the previous device.
    fn tick(&mut self, t: Time, inputs: &[Sample]) {
        self.device.tick(t, inputs, &mut self.outputs);
    }
}
