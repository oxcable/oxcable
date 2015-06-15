//! Sets up a subtractive synth listening to the default MIDI input

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use oxcable::chain::DeviceChain;
    use oxcable::instruments::subtractive_synth::SubtractiveSynth;
    use oxcable::io::audio::AudioEngine;
    use oxcable::io::midi::MidiEngine;
    use oxcable::utils::tick::tick_until_enter;

    println!("Initializing signal chain...");
    let audio_engine = AudioEngine::open().unwrap();
    let midi_engine = MidiEngine::open().unwrap();
    let mut chain = DeviceChain::from(
        SubtractiveSynth::new(midi_engine.choose_input(), 2)
    ).into(
        audio_engine.new_output(1)
    );

    println!("Playing. Press Enter to quit...");
    tick_until_enter(&mut chain);
}
