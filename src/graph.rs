//! Provides a way to link `AudioDevice`s into an acyclic graph.

use types::{AudioDevice, DeviceIOType, Sample, Time};
use utils::tick::Tick;

pub struct DeviceGraph {
    devices: Vec<AudioNode>,
    bus: Vec<Sample>,
    time: Time
}

impl DeviceGraph {
    pub fn new() -> DeviceGraph {
        DeviceGraph {
            devices: Vec::new(),
            bus: Vec::new(),
            time: 0
        }
    }

    pub fn add_node<D>(&mut self, device: D) -> AudioNodeIdx
            where D: 'static+AudioDevice {
        let node = AudioNode::new(device, &mut self.bus);
        self.devices.push(node);
        AudioNodeIdx(self.devices.len()-1)
    }

    pub fn add_edge(&mut self, from: AudioNodeIdx, from_ch: usize,
                    to: AudioNodeIdx, to_ch: usize) -> Result<(), GraphError> {
        // Check device indices
        let AudioNodeIdx(from_i) = from;
        let AudioNodeIdx(to_i) = to;
        if from_i >= self.devices.len() {
            return Err(GraphError::FromOutOfRange);
        } else if to_i >= self.devices.len() {
            return Err(GraphError::ToOutOfRange);
        }

        // Check channels
        match self.devices[from_i].device.num_outputs() {
            DeviceIOType::Exactly(i) if from_ch >= i =>
                return Err(GraphError::FromChOutOfRange),
            _ => ()
        }
        match self.devices[to_i].device.num_inputs() {
            DeviceIOType::Exactly(i) if to_ch >= i =>
                return Err(GraphError::ToChOutOfRange),
            _ => ()
        }
        while self.devices[to_i].inputs.len() < to_ch {
            self.devices[to_i].inputs.push(None);
        }

        // Set input
        let (start,_) = self.devices[from_i].outputs;
        self.devices[to_i].inputs[to_ch] = Some(start+from_ch);
        Ok(())
    }
}

impl Tick for DeviceGraph {
    fn tick(&mut self) {
        for i in 0..self.devices.len() {
            let inputs: Vec<Sample> = self.devices[i].inputs.iter().map(|input|
                input.map_or(0.0, |ch| self.bus[ch])).collect();
            let (start, end) = self.devices[i].outputs;
            self.devices[i].device.tick(self.time, &inputs,
                                        &mut self.bus[start..end]);
        }
        self.time += 1;
    }
}

#[derive(Copy, Clone, Debug)]
pub enum GraphError {
    FromOutOfRange, FromChOutOfRange,
    ToOutOfRange, ToChOutOfRange
}

#[derive(Copy, Clone, Debug)]
pub struct AudioNodeIdx(usize);
struct AudioNode {
    device: Box<AudioDevice>,
    inputs: Vec<Option<usize>>,
    outputs: (usize, usize)
}

impl AudioNode {
    fn new<D>(device: D, bus: &mut Vec<Sample>) -> AudioNode
            where D: 'static+AudioDevice {
        let num_in = match device.num_inputs() {
            DeviceIOType::Any => 0,
            DeviceIOType::Exactly(i) => i
        };
        let mut inputs = Vec::with_capacity(num_in);
        for _ in 0..num_in {
            inputs.push(None)
        }

        let num_out = match device.num_outputs() {
            DeviceIOType::Any => panic!("DeviceGraph does not support Any outputs"),
            DeviceIOType::Exactly(i) => i
        };
        let start = bus.len();
        for _ in 0..num_out {
            bus.push(0.0);
        }
        let end = bus.len();

        AudioNode {
            device: Box::new(device),
            inputs: inputs,
            outputs: (start, end)
        }
    }
}
