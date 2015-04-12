//! Sets up a subtractive synth listening to the default MIDI input

extern crate oxcable;

#[cfg(not(test))]
fn main() {
    use std::rc::Rc;
    use oxcable::components::DeviceManager;
    use oxcable::instruments::subtractive_synth::SubtractiveSynth;
    use oxcable::io::audio::{AudioEngine, AudioOut};
    use oxcable::io::midi::{MidiEngine, MidiIn};

    println!("Initializing signal chain...");
    let audio_engine = Rc::new(AudioEngine::open().unwrap());
    let midi_engine = Rc::new(MidiEngine::open().unwrap());
    let mut midi = MidiIn::new(midi_engine);
    let mut synth = SubtractiveSynth::new(2);
    let mut spk = AudioOut::new(audio_engine, 1);

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
}
