//! Sets up a subtractive synth listening to the default MIDI input

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use oxcable::instruments::subtractive_synth::SubtractiveSynth;
    use oxcable::init;
    use oxcable::types::Device;
    use oxcable::io::audio::AudioOut;
    use oxcable::io::midi::MidiIn;

    println!("Initializing signal chain...");
    assert!(init::initialize().is_ok());

    let mut midi = MidiIn::new();
    let mut synth = SubtractiveSynth::new(2);
    let mut spk = AudioOut::new(1);

    synth.input.set_channel(midi.output.get_channel());
    spk.inputs.set_channel(0, synth.output.outputs.get_channel(0));

    println!("Playing...");
    let mut t = 0;
    loop {
        midi.tick(t);
        synth.tick(t);
        spk.tick(t);
        t += 1;
    }

    // midi.stop();
    // assert!(init::terminate().is_ok());
}
