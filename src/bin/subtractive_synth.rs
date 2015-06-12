//! Sets up a subtractive synth listening to the default MIDI input

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use std::rc::Rc;
    use oxcable::chain::DeviceChain;
    use oxcable::instruments::subtractive_synth::SubtractiveSynth;
    use oxcable::io::audio::AudioEngine;
    use oxcable::io::midi::{MidiEngine, MidiIn};

    println!("Initializing signal chain...");
    let audio_engine = AudioEngine::open().unwrap();
    let midi_engine = Rc::new(MidiEngine::open().unwrap());
    let mut chain = DeviceChain::from(SubtractiveSynth::new(
            MidiIn::new(midi_engine.clone()), 2))
        .into(audio_engine.new_output(1));

    println!("Playing. Press Enter to quit...");
    chain.tick_until_enter();
}
