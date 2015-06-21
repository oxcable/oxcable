//! Provides a way to link `AudioDevice`s into an acyclic graph.

use std::collections::VecDeque;

use types::{AudioDevice, Sample, Time};
use tick::Tick;


pub struct DeviceGraph {
    nodes: Vec<AudioNode>, // the actual nodes
    topology: Vec<usize>, // the order to tick the nodes
    bus: Vec<Sample>, // the audio bus to write samples to
    time: Time // the next timestep
}

impl DeviceGraph {
    pub fn new() -> DeviceGraph {
        DeviceGraph {
            nodes: Vec::new(),
            topology: Vec::new(),
            bus: Vec::new(),
            time: 0
        }
    }

    pub fn add_node<D>(&mut self, device: D) -> AudioNodeIdx
            where D: 'static+AudioDevice {
        let node = AudioNode::new(device, &mut self.bus);
        let idx = self.nodes.len();
        self.nodes.push(node);
        self.topology.push(idx);
        AudioNodeIdx(idx)
    }

    pub fn add_edge(&mut self, from: AudioNodeIdx, from_ch: usize,
                    to: AudioNodeIdx, to_ch: usize) -> Result<(), GraphError> {
        // Check device indices
        let AudioNodeIdx(from_i) = from;
        let AudioNodeIdx(to_i) = to;
        if from_i >= self.nodes.len() {
            return Err(GraphError::FromOutOfRange);
        } else if to_i >= self.nodes.len() {
            return Err(GraphError::ToOutOfRange);
        }

        // Check channels
        if self.nodes[from_i].device.num_outputs() <= from_ch {
            return Err(GraphError::FromChOutOfRange);
        }
        if self.nodes[to_i].device.num_inputs() <= to_ch {
            return Err(GraphError::ToChOutOfRange);
        }
        while self.nodes[to_i].inputs.len() < to_ch {
            self.nodes[to_i].inputs.push(None);
        }

        // Set input
        let (start,_) = self.nodes[from_i].outputs;
        self.nodes[to_i].inputs[to_ch] = Some(start+from_ch);
        self.topological_sort(to_i, to_ch)
    }

    /// Determines the topology of our device graph. If the graph has a cycle,
    /// then we remove the last edge. Otherwise, we set self.topology to
    /// a topologically sorted order.
    fn topological_sort(&mut self, to_i: usize, to_ch: usize) ->
            Result<(), GraphError> {
        // Intialize our set of input edges, and our set of edgeless nodes
        let mut topology = Vec::new();
        let mut inputs: Vec<Vec<_>> = self.nodes.iter().map(
            |node| node.inputs.iter().filter_map(|&o| o).collect()
        ).collect();
        let mut no_inputs: VecDeque<_> = inputs.iter().enumerate().filter_map(
            |(i, ins)| if ins.len() == 0 { Some(i) } else { None }
        ).collect();

        // While there are nodes with no input, we choose one, add it as the
        // next node in our topology, and remove all edges from that node. Any
        // nodes that lose their final edge are added to the edgeless set.
        loop {
            match no_inputs.pop_front() {
                Some(i) => {
                    topology.push(i);
                    let (out_start, out_end) = self.nodes[i].outputs;
                    for out in out_start..out_end {
                        for (j, ins) in inputs.iter_mut().enumerate() {
                            let mut idx = None;
                            for k in 0..ins.len() {
                                if ins[k] == out {
                                    idx = Some(k);
                                    break;
                                }
                            }
                            match idx {
                                Some(k) => {
                                    ins.swap_remove(k);
                                    if ins.len() == 0 {
                                        no_inputs.push_back(j);
                                    }
                                },
                                None => ()
                            }
                        }
                    }
                },
                None => break
            }
        }

        if topology.len() == self.nodes.len() {
            self.topology = topology;
            Ok(())
        } else {
            self.nodes[to_i].inputs[to_ch] = None;
            Err(GraphError::CreatesCycle)
        }
    }
}

impl Tick for DeviceGraph {
    fn tick(&mut self) {
        for &i in self.topology.iter() {
            let inputs: Vec<Sample> = self.nodes[i].inputs.iter().map(|input|
                input.map_or(0.0, |ch| self.bus[ch])).collect();
            let (start, end) = self.nodes[i].outputs;
            self.nodes[i].device.tick(self.time, &inputs,
                                        &mut self.bus[start..end]);
        }
        self.time += 1;
    }
}

#[derive(Copy, Clone, Debug)]
pub enum GraphError {
    FromOutOfRange, FromChOutOfRange, ToOutOfRange, ToChOutOfRange, CreatesCycle
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
        let num_in = device.num_inputs();
        let mut inputs = Vec::with_capacity(num_in);
        for _ in 0..num_in {
            inputs.push(None)
        }

        let num_out = device.num_outputs();
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
