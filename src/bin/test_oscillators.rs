//! Writes wav files for every Oscillator waveform for analysis

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use oxcable::chain::{DeviceChain, Tick};
    use oxcable::io::wav::WavWriter;
    use oxcable::oscillator::*;

    // Initialize objects
    println!("Initializing signal chain...");
    let freq = 8000.0;
    let mut chains: Vec<DeviceChain> = Vec::new();
    chains.push(DeviceChain::from(Oscillator::new(Sine).freq(freq))
        .into(WavWriter::create("wav/test_sine.wav", 1).unwrap()));
    chains.push(DeviceChain::from(Oscillator::new(Saw(Aliased)).freq(freq))
        .into(WavWriter::create("wav/test_saw_naive.wav", 1).unwrap()));
    chains.push(DeviceChain::from(Oscillator::new(Saw(PolyBlep)).freq(freq))
        .into(WavWriter::create("wav/test_saw_blep.wav", 1).unwrap()));
    chains.push(DeviceChain::from(Oscillator::new(Square(Aliased)).freq(freq))
        .into(WavWriter::create("wav/test_square_naive.wav", 1).unwrap()));
    chains.push(DeviceChain::from(Oscillator::new(Square(PolyBlep)).freq(freq))
        .into(WavWriter::create("wav/test_square_blep.wav", 1).unwrap()));
    chains.push(DeviceChain::from(Oscillator::new(Tri(Aliased)).freq(freq))
        .into(WavWriter::create("wav/test_tri_naive.wav", 1).unwrap()));
    chains.push(DeviceChain::from(Oscillator::new(Tri(PolyBlep)).freq(freq))
        .into(WavWriter::create("wav/test_tri_blep.wav", 1).unwrap()));
    chains.push(DeviceChain::from(Oscillator::new(WhiteNoise).freq(freq))
        .into(WavWriter::create("wav/test_white_noise.wav", 1).unwrap()));
    chains.push(DeviceChain::from(Oscillator::new(PulseTrain).freq(freq))
        .into(WavWriter::create("wav/test_pulse_train.wav", 1).unwrap()));

    // Write files
    println!("Writing oscillators to wav files...");
    for chain in chains.iter_mut() {
        chain.tick_n_times(44100);
    }
    println!("Done");
}
