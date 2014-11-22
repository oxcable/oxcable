//! Reads a linearly ramped sin wave, applies each dynamic processor to it and
//! writes the results out to wav files for analysis.

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use std::vec::Vec;
    use oxcable::core::types::Device;
    use oxcable::dynamics;
    use oxcable::io::wav;

    // Initialize objects
    println!("Initializing signal chain..."); 
    let mut wav_in = wav::WavReader::new("wav/volume_up.wav");
    let mut wav_outs = Vec::new();

    let mut comp = box dynamics::Compressor::new(-6.0, 0.5, 0.0, 1);
    comp.inputs.set_channel(0, wav_in.outputs.get_channel(0));
    wav_outs.push(wav::WavWriter::new("wav/test_compressor.wav", 1));
    wav_outs[0].inputs.set_channel(0, comp.outputs.get_channel(0));

    let mut lim = dynamics::Limiter::new(-6.0, 0.0, 1);
    lim.inputs.set_channel(0, wav_in.outputs.get_channel(0));
    wav_outs.push(wav::WavWriter::new("wav/test_limiter.wav", 1));
    wav_outs[1].inputs.set_channel(0, lim.outputs.get_channel(0));

    let mut gate = dynamics::NoiseGate::new(-6.0, -9.0, 0.0, 1);
    gate.inputs.set_channel(0, wav_in.outputs.get_channel(0));
    wav_outs.push(wav::WavWriter::new("wav/test_noise_gate.wav", 1));
    wav_outs[2].inputs.set_channel(0, gate.outputs.get_channel(0));

    // Write files
    println!("Writing to wav files..."); 
    let mut t = 0;
    while !wav_in.is_done() {
        wav_in.tick(t);
        comp.tick(t);
        lim.tick(t);
        gate.tick(t);
        for i in range(0, wav_outs.len()) {
            wav_outs[i].tick(t);
        }
        t += 1;
    }

    // Finish the wav files
    for i in range(0, wav_outs.len()) {
        wav_outs[i].update_data_size();
    }
    println!("Done");
}
