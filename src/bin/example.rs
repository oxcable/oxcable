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
    use oxcable::oscillator::Oscillator;
    use oxcable::utils::tick::tick_until_enter;

    println!("Setting up signal chain...");
    let engine = AudioEngine::open().unwrap();
    let mut graph = DeviceGraph::new();

    let mic = graph.add_node(engine.new_input(1));
    let del = graph.add_node(Delay::new(0.5, 0.5, 0.5, 1));
    graph.add_edge(mic, 0, del, 0).unwrap();

    let osc = graph.add_node(Oscillator::new(oscillator::Sine, 440.0));
    let gain = graph.add_node(Gain::new(-12.0, 1));
    graph.add_edge(osc, 0, gain, 0).unwrap();

    let spk = graph.add_node(engine.new_output(2));
    graph.add_edge(del, 0, spk, 0).unwrap();
    graph.add_edge(gain, 0, spk, 1).unwrap();

    println!("Playing. Press Enter to quit...");
    tick_until_enter(&mut graph);
    println!("Done!");
}
