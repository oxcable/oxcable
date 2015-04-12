//! Sets up a subtractive synth listening to the default MIDI input

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use oxcable::init;
    use oxcable::components::DeviceManager;
    use oxcable::instruments::subtractive_synth::SubtractiveSynth;
    use oxcable::io::audio::AudioOut;
    use oxcable::io::midi::MidiIn;

    println!("Initializing signal chain...");
    assert!(init::initialize().is_ok());

    let mut midi = MidiIn::new();
    let mut synth = SubtractiveSynth::new(2);
    let mut spk = AudioOut::new(1);

    synth.input.set_channel(midi.output.get_channel());
    spk.inputs.set_channel(0, synth.output.outputs.get_channel(0));

    let mut manager = DeviceManager::new();
    manager.add_device(&mut midi);
    manager.add_device(&mut synth);
    manager.add_device(&mut spk);

    println!("Playing...");
    loop {
        manager.tick();
    }

    // midi.stop();
    // assert!(init::terminate().is_ok());
}
