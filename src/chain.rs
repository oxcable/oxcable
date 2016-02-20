//! A container for a series of audio devices.
//!
//! A chain can be used when a single series of audio devices passes its output
//! to the input of the next device. It is initialized from a single starting
//! device that will receive no input, and ends in a device who's output is
//! ignored.
//!
//!
//! # Example
//! The following will pass microphone input through a low pass filter, then out
//! to the speaker:
//!
//! ```no_run
//! use oxcable::chain::{DeviceChain, Tick};
//! use oxcable::filters::first_order::{Filter, LowPass};
//! use oxcable::io::audio::AudioEngine;
//!
//! let engine = AudioEngine::with_buffer_size(256).unwrap();
//! let mut chain = DeviceChain::from(
//!     engine.default_input(1).unwrap()
//! ).into(
//!     Filter::new(LowPass(8000f32), 1)
//! ).into(
//!     engine.default_output(1).unwrap()
//! );
//! chain.tick_forever();
//! ```


use types::{AudioDevice, Sample, Time};
pub use tick::Tick;


/// A container for a series of audio devices.
pub struct DeviceChain {
    bus: Vec<Sample>,
    devices: Vec<AudioNode>,
    time: Time
}

impl DeviceChain {
    /// Creates a new chain that starts from the provided device. This device
    /// will receive no inputs unless they are manually supplied using
    /// DeviceChain::get_input.
    pub fn from<D>(device: D) -> Self where D: 'static+AudioDevice {
        let mut chain = DeviceChain {
            bus: Vec::new(),
            devices: Vec::new(),
            time: 0
        };
        for _ in 0..device.num_inputs() {
            chain.bus.push(0.0);
        }
        chain.devices.push(AudioNode::new(device, &mut chain.bus));
        chain
    }

    /// Appends the provided device to the end of the chain. This device will be
    /// passed the output of the last device as input. This method returns the
    /// same chain it was passed.
    ///
    /// # Panics
    ///
    /// Panics if the provided device does not have as many inputs as the
    /// previous device has outputs.
    pub fn into<D>(mut self, device: D) -> Self where D: 'static+AudioDevice {
        if self.devices[self.devices.len()-1].device.num_outputs() !=
                device.num_inputs() {
            panic!("DeviceChain: number of outputs must match number of inputs");
        }
        self.devices.push(AudioNode::new(device, &mut self.bus));
        self
    }

    /// Return a mutable slice to the input of the first device in the chain.
    ///
    /// These inputs never get overwritten, so if you are supplying input you
    /// must manually zero the buffer again.
    pub fn get_input(&mut self) -> &mut[Sample] {
        &mut self.bus[..self.devices[0].device.num_inputs()]
    }

    /// Returns a slice to the output of the last device in the chain.
    pub fn get_output(&self) -> &[Sample] {
        let outputs = self.devices[self.devices.len()-1].device.num_outputs();
        &self.bus[self.bus.len()-outputs..]
    }
}

impl Tick for DeviceChain {
    fn tick(&mut self) {
        for device in self.devices.iter_mut() {
            device.tick(self.time, &mut self.bus);
        }
        self.time += 1;
    }
}


/// Wrap an audio device behind a pointer, and stores an index into the bus.
struct AudioNode {
    device: Box<AudioDevice>,
    bus_start: usize,
    bus_end: usize,
    split_point: usize
}

impl AudioNode {
    /// Wraps the provided audio device in a new node and allocate space for it
    /// in the bus.
    fn new<D>(device: D, bus: &mut Vec<Sample>) -> AudioNode
            where D: 'static+AudioDevice {
        let inputs = device.num_inputs();
        let outputs = device.num_outputs();
        let bus_start = bus.len() - inputs;
        for _ in 0..outputs {
            bus.push(0.0);
        }
        AudioNode {
            device: Box::new(device),
            bus_start: bus_start,
            bus_end: bus_start + inputs + outputs,
            split_point: inputs,
        }
    }

    /// Ticks the device one time step.
    fn tick(&mut self, t: Time, bus: &mut [Sample]) {
        let bus_slice = &mut bus[self.bus_start .. self.bus_end];
        let (inputs, outputs) = bus_slice.split_at_mut(self.split_point);
        self.device.tick(t, inputs, outputs);
    }
}


#[cfg(test)]
mod test {
    use testing::MockAudioDevice;
    use super::{DeviceChain, Tick};

    #[test]
    fn test_success() {
        let mut mock1 = MockAudioDevice::new("mock1", 0, 1);
        let mut mock2 = MockAudioDevice::new("mock2", 1, 2);
        let mut mock3 = MockAudioDevice::new("mock3", 2, 0);
        mock1.will_tick(&[], &[1.0]);
        mock2.will_tick(&[1.0], &[2.0, 3.0]);
        mock3.will_tick(&[2.0, 3.0], &[]);

        DeviceChain::from(mock1).into(mock2).into(mock3).tick();
    }

    #[test]
    fn test_input() {
        let mut mock = MockAudioDevice::new("mock1", 1, 0);
        mock.will_tick(&[1.0], &[]);

        let mut chain = DeviceChain::from(mock);
        chain.get_input()[0] = 1.0;
        chain.tick();
    }

    #[test]
    fn test_output() {
        let mut mock = MockAudioDevice::new("mock1", 0, 1);
        mock.will_tick(&[], &[1.0]);

        let mut chain = DeviceChain::from(mock);
        chain.tick();
        assert_eq!(chain.get_output(), [1.0]);
    }

    #[test]
    #[should_panic]
    fn test_wrong_number_inputs() {
        let mock1 = MockAudioDevice::new("mock1", 0, 1);
        let mock2 = MockAudioDevice::new("mock2", 2, 1);
        DeviceChain::from(mock1).into(mock2);
    }
}
