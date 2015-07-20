//! Reads input from a wav file and mirrors it to a different wav file.
//!
//! The destination wav file should be an exact duplicate of the source.

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use oxcable::chain::{DeviceChain, Tick};
    use oxcable::io::wav::{WavReader, WavWriter};
    use oxcable::types::AudioDevice;

    println!("Initializing signal chain...");
    let inwav = WavReader::open("wav/song.wav").unwrap();
    let samples = inwav.get_num_samples();
    let outwav = WavWriter::create("wav/test_wav.wav", inwav.num_outputs()).unwrap();
    let mut chain = DeviceChain::from(inwav).into(outwav);

    println!("Mirroring wav/song.wav input to wav/test_wav.wav...");
    chain.tick_n_times(samples);
    println!("Done!");
}
