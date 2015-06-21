//! Reads a linearly ramped sin wave, applies each dynamic processor to it and
//! writes the results out to wav files for analysis.

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use oxcable::dynamics::*;
    use oxcable::graph::DeviceGraph;
    use oxcable::io::wav::{WavReader, WavWriter};
    use oxcable::utils::tick::tick_n_times;

    // Initialize objects
    println!("Initializing signal chain...");
    let mut graph = DeviceGraph::new();
    let wavf = WavReader::new("wav/volume_up.wav");
    let samples = wavf.get_num_samples();
    let wav_in = graph.add_node(wavf);

    let comp = graph.add_node(Compressor::new(-6.0, 0.5, 0.0, 1));
    let out = graph.add_node(WavWriter::new("wav/test_compressor.wav", 1));
    graph.add_edge(wav_in, 0, comp, 0).unwrap();
    graph.add_edge(comp, 0, out, 0).unwrap();

    let lim = graph.add_node(Limiter::new(-6.0, 0.0, 1));
    let out = graph.add_node(WavWriter::new("wav/test_limiter.wav", 1));
    graph.add_edge(wav_in, 0, lim, 0).unwrap();
    graph.add_edge(lim, 0, out, 0).unwrap();

    let gate = graph.add_node(NoiseGate::new(-6.0, -9.0, 0.0, 1));
    let out = graph.add_node(WavWriter::new("wav/test_noise_gate.wav", 1));
    graph.add_edge(wav_in, 0, gate, 0).unwrap();
    graph.add_edge(gate, 0, out, 0).unwrap();

    // Write files
    println!("Writing to wav files...");
    tick_n_times(&mut graph, samples);
    println!("Done!");
}
