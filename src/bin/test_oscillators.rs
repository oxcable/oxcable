//! Writes wav files for every Oscillator waveform for analysis

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use std::vec::Vec;
    use oxcable::core::AudioDevice;
    use oxcable::io::wav::WavWriter;
    use oxcable::oscillator;
    use oxcable::oscillator::Oscillator;
    println!("Writing oscillators to wav files...");

    // Initialize objects
    let freq = 8000.0;
    let mut oscs = Vec::<oscillator::Oscillator>::new();
    let mut wavs = Vec::<WavWriter>::new();
    oscs.push(Oscillator::new(oscillator::Sine, freq));
    wavs.push(WavWriter::new("wav/test_sine.wav", 1));
    oscs.push(Oscillator::new(oscillator::Saw(oscillator::Aliased), freq));
    wavs.push(WavWriter::new("wav/test_saw_naive.wav", 1));
    oscs.push(Oscillator::new(oscillator::Saw(oscillator::PolyBlep), freq));
    wavs.push(WavWriter::new("wav/test_saw_blep.wav", 1));
    oscs.push(Oscillator::new(oscillator::Square(oscillator::Aliased), freq));
    wavs.push(WavWriter::new("wav/test_square_naive.wav", 1));
    oscs.push(Oscillator::new(oscillator::Square(oscillator::PolyBlep), freq));
    wavs.push(WavWriter::new("wav/test_square_blep.wav", 1));
    oscs.push(Oscillator::new(oscillator::Tri(oscillator::Aliased), freq));
    wavs.push(WavWriter::new("wav/test_tri_naive.wav", 1));
    oscs.push(Oscillator::new(oscillator::Tri(oscillator::PolyBlep), freq));
    wavs.push(WavWriter::new("wav/test_tri_blep.wav", 1));
    oscs.push(Oscillator::new(oscillator::WhiteNoise, freq));
    wavs.push(WavWriter::new("wav/test_white_noise.wav", 1));
    oscs.push(Oscillator::new(oscillator::PulseTrain, freq));
    wavs.push(WavWriter::new("wav/test_pulse_train.wav", 1));

    // Link oscillators to wav outs
    for i in range(0u, oscs.len()) {
        wavs[i].inputs.set_channel(0, oscs[i].output.get_channel(0));
    }

    // Write files
    for t in range(1, 44100) {
        for i in range(0, oscs.len()) {
            oscs[i].tick(t);
            wavs[i].tick(t);
        }
    }

    // Finish the wav files
    for i in range(0, wavs.len()) {
        wavs[i].update_data_size();
    }
    println!("Done");
}
