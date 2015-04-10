//! Writes wav files for every first order filter for analysis

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use std::vec::Vec;
    use oxcable::types::Device;
    use oxcable::io::wav;
    use oxcable::filters::first_order;
    use oxcable::filters::first_order::Filter;

    // Initialize objects
    println!("Initializing signal chain...");
    let mut wav_in = wav::WavReader::new("wav/delta.wav");

    let cutoff = 1000.0;
    let mut filters = Vec::<first_order::Filter>::new();
    let mut wavs = Vec::<wav::WavWriter>::new();
    filters.push(Filter::new(first_order::LowPass(cutoff), 1));
    wavs.push(wav::WavWriter::new("wav/test_first_order_low_pass.wav", 1));
    filters.push(Filter::new(first_order::HighPass(cutoff), 1));
    wavs.push(wav::WavWriter::new("wav/test_first_order_high_pass.wav", 1));
    filters.push(Filter::new(first_order::LowShelf(cutoff, -6.0), 1));
    wavs.push(wav::WavWriter::new("wav/test_first_order_low_shelf.wav", 1));
    filters.push(Filter::new(first_order::HighShelf(cutoff, -6.0), 1));
    wavs.push(wav::WavWriter::new("wav/test_first_order_high_shelf.wav", 1));

    // Link oscillators to wav outs
    for i in (0 .. filters.len()) {
        filters[i].inputs.set_channel(0, wav_in.outputs.get_channel(0));
        wavs[i].inputs.set_channel(0, filters[i].outputs.get_channel(0));
    }

    // Write files
    println!("Writing first order filters to wav files...");
    for t in (0u64 .. 44100) {
        wav_in.tick(t);
        for i in (0 .. filters.len()) {
            filters[i].tick(t);
            wavs[i].tick(t);
        }
    }
    println!("Done");
}
