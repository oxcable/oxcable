//! Reads input from a microphone and mirrors it to a speaker ad nauseum.

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use oxcable::chain::DeviceChain;
    use oxcable::io::wav::{WavReader, WavWriter};
    use oxcable::types::AudioDevice;
    use oxcable::tick::Tick;

    println!("Initializing signal chain...");
    let inwav = WavReader::open("wav/song.wav").unwrap();
    let samples = inwav.get_num_samples();
    let outwav = WavWriter::create("wav/test_wav.wav", inwav.num_outputs()).unwrap();
    let mut chain = DeviceChain::from(inwav).into(outwav);

    println!("Mirroring wav/song.wav input to wav/test_wav.wav...");
    chain.tick_n_times(samples);
    println!("Done!");
}
