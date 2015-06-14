//! Writes wav files for every first order filter for analysis

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use oxcable::io::wav;
    use oxcable::filters::first_order;
    use oxcable::filters::first_order::Filter;
    use oxcable::graph::DeviceGraph;
    use oxcable::utils::tick::Tick;

    // Initialize objects
    println!("Initializing signal chain...");
    let mut graph = DeviceGraph::new();
    let wavf = wav::WavReader::new("wav/delta.wav");
    let wav_in = graph.add_node(wavf);
    let cutoff = 1000.0;

    let filt = graph.add_node(Filter::new(first_order::LowPass(cutoff), 1));
    let out = graph.add_node(wav::WavWriter::new(
            "wav/test_first_order_low_pass.wav", 1));
    graph.add_edge(wav_in, 0, filt, 0).unwrap();
    graph.add_edge(filt, 0, out, 0).unwrap();

    let filt = graph.add_node(Filter::new(first_order::HighPass(cutoff), 1));
    let out = graph.add_node(wav::WavWriter::new(
            "wav/test_first_order_high_pass.wav", 1));
    graph.add_edge(wav_in, 0, filt, 0).unwrap();
    graph.add_edge(filt, 0, out, 0).unwrap();

    let filt = graph.add_node(Filter::new(first_order::LowShelf(cutoff, -6.0), 1));
    let out = graph.add_node(wav::WavWriter::new(
            "wav/test_first_order_low_shelf.wav", 1));
    graph.add_edge(wav_in, 0, filt, 0).unwrap();
    graph.add_edge(filt, 0, out, 0).unwrap();

    let filt = graph.add_node(Filter::new(first_order::HighShelf(cutoff, -6.0), 1));
    let out = graph.add_node(wav::WavWriter::new(
            "wav/test_first_order_high_shelf.wav", 1));
    graph.add_edge(wav_in, 0, filt, 0).unwrap();
    graph.add_edge(filt, 0, out, 0).unwrap();

    // Write files
    println!("Writing first order filters to wav files...");
    for _ in (0u64 .. 44100) {
        graph.tick();
    }
    println!("Done!");
}
