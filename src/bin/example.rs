//! A simple example using several components
//! Also a lazy, cheeky way to test some simple processors

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use oxcable::oscillator;
    use oxcable::delay::Delay;
    use oxcable::graph::DeviceGraph;
    use oxcable::io::audio::AudioEngine;
    use oxcable::mixers::Gain;
    use oxcable::oscillator::*;
    use oxcable::tick::Tick;

    println!("Setting up signal chain...");
    let engine = AudioEngine::with_buffer_size(256).unwrap();
    let mut graph = DeviceGraph::new();
    let spk = graph.add_node(engine.default_output(2).unwrap());

    let mic = graph.add_node(engine.default_input(1).unwrap());
    let del = graph.add_node(Delay::new(0.5, 0.5, 0.5, 1));
    graph.add_edge(mic, 0, del, 0).unwrap();
    graph.add_edge(del, 0, spk, 0).unwrap();

    let lfo = graph.add_node(Oscillator::new(oscillator::Sine).freq(10.0));
    let osc = graph.add_node(
        Oscillator::new(Tri(PolyBlep)).freq(440.0).lfo_intensity(0.1)
    );
    let gain = graph.add_node(Gain::new(-6.0, 1));
    graph.add_edge(lfo, 0, osc, 0).unwrap();
    graph.add_edge(osc, 0, gain, 0).unwrap();
    graph.add_edge(gain, 0, spk, 1).unwrap();

    println!("Playing. Press Enter to quit...");
    graph.tick_until_enter();
    println!("Done!");
}
