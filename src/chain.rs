//! Provides a linear chain to cascade `AudioDevice`s together.

use types::{AudioDevice, Sample, Time};
pub use tick::Tick;

pub struct DeviceChain {
    devices: Vec<AudioNode>,
    time: Time
}

impl DeviceChain {
    pub fn from<D>(device: D) -> DeviceChain where D: 'static+AudioDevice {
        DeviceChain { devices: vec![AudioNode::new(device)], time: 0 }
    }

    pub fn into<D>(mut self, device: D) -> DeviceChain where D: 'static+AudioDevice {
        if self.devices[self.devices.len()-1].outputs.len() != device.num_inputs() {
            panic!("DeviceChain: number of outputs must match number of inputs");
        }
        self.devices.push(AudioNode::new(device));
        self
    }

    pub fn get_output(&self) -> Vec<Sample> {
        self.devices[self.devices.len()-1].outputs.clone()
    }
}

impl Tick for DeviceChain {
    fn tick(&mut self) {
        self.devices[0].tick(self.time, &[0.0;0]);
        for i in 1..self.devices.len() {
            let inputs = self.devices[i-1].outputs.clone();
            self.devices[i].tick(self.time, &inputs);
        }
        self.time += 1;
    }
}

struct AudioNode {
    device: Box<AudioDevice>,
    outputs: Vec<Sample>
}

impl AudioNode {
    fn new<D>(device: D) -> AudioNode where D: 'static+AudioDevice {
        let n = device.num_outputs();
        AudioNode {
            device: Box::new(device),
            outputs: vec![0.0; n]
        }
    }

    fn tick(&mut self, t: Time, inputs: &[Sample]) {
        self.device.tick(t, inputs, &mut self.outputs);
    }
}
