//! A simple example meant to demonstrate some basic usage of oxcable.
//!
//! This example is prone to minor changes, because I also use it as an easy way
//! to try out new audio devices.

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use oxcable::oscillator;
    use oxcable::filters::first_order::{Filter, LowPass};
    use oxcable::graph::{DeviceGraph, Tick};
    use oxcable::io::audio::AudioEngine;
    use oxcable::mixers::Gain;
    use oxcable::oscillator::*;

    println!("Setting up signal chain...");
    let engine = AudioEngine::with_buffer_size(256).unwrap();
    let mut graph = DeviceGraph::new();
    let speaker = graph.add_node(engine.default_output(2).unwrap());

    let microphone = graph.add_node(engine.default_input(1).unwrap());
    let filter = graph.add_node(Filter::new(LowPass(8000f32), 1));
    graph.add_edge(microphone, 0, filter, 0).unwrap();
    graph.add_edge(filter, 0, speaker, 0).unwrap();

    let lfo = graph.add_node(Oscillator::new(oscillator::Sine).freq(10.0));
    let oscillator = graph.add_node(
        Oscillator::new(Tri(PolyBlep)).freq(440.0).lfo_intensity(0.1)
    );
    let gain = graph.add_node(Gain::new(-6.0, 1));
    graph.add_edge(lfo, 0, oscillator, 0).unwrap();
    graph.add_edge(oscillator, 0, gain, 0).unwrap();
    graph.add_edge(gain, 0, speaker, 1).unwrap();

    println!("Playing. Press Enter to quit...");
    graph.tick_until_enter();
    println!("Done!");
}
