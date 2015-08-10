//! A container for audio devices in an acyclic graph.
//!
//! A graph can be used when many audio devices need to connect in complex
//! topologies. It can connect each output channel of a device to any input
//! channel, provided that connection does not create a cycle.
//!
//! A graph is initialized by adding each device as a node in the graph, and
//! then specifying the edges between devices. The graph will automatically
//! process the devices in order of their dependencies.
//!
//! # Example
//! 
//! The following example creates a graph with two different branches into
//! a stereo output. It feeds an oscillator to the left channel, and
//! a microphone with delay into the right channel.
//!
//! ```
//! # // this example uses tick_forever(), so we wrap it in a function that gets
//! # // compiled by not run by doctest
//! # fn dummy() {
//! use oxcable::delay::Delay;
//! use oxcable::graph::{DeviceGraph, Tick};
//! use oxcable::io::audio::AudioEngine;
//! use oxcable::oscillator::*;
//!
//! let engine = AudioEngine::with_buffer_size(256).unwrap();
//! let mut graph = DeviceGraph::new();
//!
//! // Add nodes to graph
//! let oscillator = graph.add_node(Oscillator::new(Sine).freq(440.0));
//! let microphone = graph.add_node(engine.default_input(1).unwrap());
//! let delay = graph.add_node(Delay::new(0.5, 0.5, 0.5, 1));
//! let speaker = graph.add_node(engine.default_output(2).unwrap());
//!
//! // Connect devices together
//! graph.add_edge(oscillator, 0, speaker, 0);
//! graph.add_edge(microphone, 0, delay, 0);
//! graph.add_edge(delay, 0, speaker, 1);
//!
//! // Play audio ad nauseam.
//! graph.tick_forever();
//! # }
//! ```

use std::collections::VecDeque;

use types::{AudioDevice, Sample, Time};
pub use tick::Tick;


/// An acyclic graph for audio devices.
pub struct DeviceGraph {
    nodes: Vec<AudioNode>, // the actual nodes
    topology: Vec<usize>, // the order to tick the nodes
    bus: Vec<Sample>, // the audio bus to write samples to
    time: Time // the next timestep
}

impl DeviceGraph {
    /// Create an empty graph.
    pub fn new() -> Self {
        DeviceGraph {
            nodes: Vec::new(),
            topology: Vec::new(),
            bus: Vec::new(),
            time: 0
        }
    }

    /// Add a new device into the graph, with no connections. Returns
    /// a identifier that refers back to this device.
    pub fn add_node<D>(&mut self, device: D) -> AudioNodeIdx
            where D: 'static+AudioDevice {
        let node = AudioNode::new(device, &mut self.bus);
        let idx = self.nodes.len();
        self.nodes.push(node);
        self.topology.push(idx);
        AudioNodeIdx(idx)
    }

    /// Connect two devices in the graph. 
    ///
    /// `src` and `dest` are identifiers for the actual devices to connect.
    ///
    /// `src_ch` and `dest_ch` are the channel indices of the two devices.
    ///
    /// If invalid indices are provided, or if the specified edge would create
    /// a cycle in the graph, an Err is returned and no changes dest the graph are
    /// made.
    pub fn add_edge(&mut self, src: AudioNodeIdx, src_ch: usize,
                    dest: AudioNodeIdx, dest_ch: usize) -> Result<(), GraphError> {
        // Check device indices
        let AudioNodeIdx(src_i) = src;
        let AudioNodeIdx(dest_i) = dest;
        if src_i >= self.nodes.len() {
            return Err(GraphError::SrcOutOfRange);
        } else if dest_i >= self.nodes.len() {
            return Err(GraphError::DestOutOfRange);
        }

        // Check channels
        if self.nodes[src_i].device.num_outputs() <= src_ch {
            return Err(GraphError::SrcChOutOfRange);
        }
        if self.nodes[dest_i].device.num_inputs() <= dest_ch {
            return Err(GraphError::DestChOutOfRange);
        }
        while self.nodes[dest_i].inputs.len() < dest_ch {
            self.nodes[dest_i].inputs.push(None);
        }

        // Set input
        let (start,_) = self.nodes[src_i].outputs;
        self.nodes[dest_i].inputs[dest_ch] = Some(start+src_ch);
        self.topological_sort(dest_i, dest_ch)
    }

    /// Determines the topology of our device graph. If the graph has a cycle,
    /// then we remove the last edge. Otherwise, we set self.topology to
    /// a topologically sorted order.
    fn topological_sort(&mut self, dest_i: usize, dest_ch: usize) ->
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
            self.nodes[dest_i].inputs[dest_ch] = None;
            Err(GraphError::CreatesCycle)
        }
    }
}

impl Tick for DeviceGraph {
    fn tick(&mut self) {
        for &i in self.topology.iter() {
            self.nodes[i].tick(self.time, &mut self.bus);
        }
        self.time += 1;
    }
}


/// Errors returned while modifying the graph.
#[derive(Copy, Clone, Debug)]
pub enum GraphError {
    /// The provided source device does not exist.
    SrcOutOfRange,
    /// The source device does not have that channel.
    SrcChOutOfRange,
    /// The provided destination device does not exist.
    DestOutOfRange,
    /// The destination device does not have that channel.
    DestChOutOfRange,
    /// Adding this edge would create a cycle.
    CreatesCycle
}


/// An identifier used to refer back to a node in the graph.
#[derive(Copy, Clone, Debug)]
pub struct AudioNodeIdx(usize);


/// A wrapper for a node in the graph.
///
/// Management of indices in the bus is handled in the graph itself.
struct AudioNode {
    device: Box<AudioDevice>, // wraps the device
    inputs: Vec<Option<usize>>, // bus indices of the inputs
    input_buf: Vec<Sample>, // an allocated buffer for containing inputs
    outputs: (usize, usize) // the range of outputs in the bus
}

impl AudioNode {
    /// Wrap the device in a new node
    fn new<D>(device: D, bus: &mut Vec<Sample>) -> AudioNode
            where D: 'static+AudioDevice {
        let num_in = device.num_inputs();
        let num_out = device.num_outputs();
        let start = bus.len();
        for _ in 0..num_out {
            bus.push(0.0);
        }
        let end = bus.len();

        AudioNode {
            device: Box::new(device),
            inputs: vec![None; num_in],
            input_buf: vec![0.0; num_in],
            outputs: (start, end)
        }
    }

    /// Extract the inputs out of the bus, tick the device and place the outputs
    /// back into the bus.
    fn tick(&mut self, t: Time, bus: &mut[Sample]) {
        for (i, ch) in self.inputs.iter().enumerate() {
            self.input_buf[i] = ch.map_or(0.0, |j| bus[j]);
        }
        let (start, end) = self.outputs;
        self.device.tick(t, &self.input_buf, &mut bus[start..end]);
    }
}
