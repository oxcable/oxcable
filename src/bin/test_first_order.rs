//! Writes wav files for every first order filter for analysis

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use oxcable::io::wav::{WavReader, WavWriter};
    use oxcable::filters::first_order::*;
    use oxcable::graph::DeviceGraph;
    use oxcable::tick::tick_n_times;

    // Initialize objects
    println!("Initializing signal chain...");
    let mut graph = DeviceGraph::new();
    let wavf = WavReader::open("wav/delta.wav").unwrap();
    let wav_in = graph.add_node(wavf);
    let cutoff = 1000.0;

    let filt = graph.add_node(Filter::new(LowPass(cutoff), 1));
    let out = graph.add_node(WavWriter::create(
            "wav/test_first_order_low_pass.wav", 1).unwrap());
    graph.add_edge(wav_in, 0, filt, 0).unwrap();
    graph.add_edge(filt, 0, out, 0).unwrap();

    let filt = graph.add_node(Filter::new(HighPass(cutoff), 1));
    let out = graph.add_node(WavWriter::create(
            "wav/test_first_order_high_pass.wav", 1).unwrap());
    graph.add_edge(wav_in, 0, filt, 0).unwrap();
    graph.add_edge(filt, 0, out, 0).unwrap();

    let filt = graph.add_node(Filter::new(LowShelf(cutoff, -6.0), 1));
    let out = graph.add_node(WavWriter::create(
            "wav/test_first_order_low_shelf.wav", 1).unwrap());
    graph.add_edge(wav_in, 0, filt, 0).unwrap();
    graph.add_edge(filt, 0, out, 0).unwrap();

    let filt = graph.add_node(Filter::new(HighShelf(cutoff, -6.0), 1));
    let out = graph.add_node(WavWriter::create(
            "wav/test_first_order_high_shelf.wav", 1).unwrap());
    graph.add_edge(wav_in, 0, filt, 0).unwrap();
    graph.add_edge(filt, 0, out, 0).unwrap();

    // Write files
    println!("Writing first order filters to wav files...");
    tick_n_times(&mut graph, 44100);
    println!("Done!");
}
